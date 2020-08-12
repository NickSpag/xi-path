// Copyright 2018 The xi-editor Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Requests and notifications from the core to front-ends.

use std::time::Instant;

use serde_json::{self, Value};
use xi_rpc::{self, RpcPeer, test_utils::DummyPeer};

use crate::config::Table;
use crate::plugins::rpc::ClientPluginInfo;
use crate::plugins::Command;
use crate::styles::{Style, ThemeSettings};
use crate::syntax::LanguageId;
use crate::tabs::ViewId;
use crate::{find::FindStatus, width_cache::{WidthReq, WidthResponse}, view::Replace};

pub trait Frontend: Send + 'static  {
    fn update_view(&self, view_id: ViewId, update: &Update);
    fn scroll_to(&self, view_id: ViewId, line: usize, col: usize);
    fn config_changed(&self, view_id: ViewId, changes: &Table);
    fn available_themes(&self, theme_names: Vec<String>);
    fn available_languages(&self, languages: Vec<LanguageId>);
    fn theme_changed(&self, name: &str, theme: &ThemeSettings);
    fn language_changed(&self, view_id: ViewId, new_lang: &LanguageId);
    fn plugin_started(&self, view_id: ViewId, plugin: &str);
    /// Notify the client that a plugin has stopped.
    ///
    /// `code` is not currently used; in the future may be used to
    /// pass an exit code.
    fn plugin_stopped(&self, view_id: ViewId, plugin: &str, code: i32);
    /// Notify the client of the available plugins.
    fn available_plugins(&self, view_id: ViewId, plugins: &[ClientPluginInfo]);
    fn update_cmds(&self, view_id: ViewId, plugin: &str, cmds: &[Command]);
    fn def_style(&self, style: &Style) ;
    fn find_status(&self, view_id: ViewId, queries: &Vec<FindStatus>);
    fn replace_status(&self, view_id: ViewId, replace: &Replace);
    /// Ask front-end to measure widths of strings.
    fn measure_width(&self, reqs: &[WidthReq]) -> WidthResponse;
    //fn alert<S: AsRef<str>>(&self, msg: S);
    fn add_status_item(
        &self,
        view_id: ViewId,
        source: &str,
        key: &str,
        value: &str,
        alignment: &str,
    );
    fn update_status_item(&self, view_id: ViewId, key: &str, value: &str);
    fn remove_status_item(&self, view_id: ViewId, key: &str);
    fn show_hover(&self, view_id: ViewId, request_id: usize, result: String);
    fn schedule_idle(&self, token: usize);
    fn schedule_timer(&self, timeout: Instant, token: usize);
}

pub type FrontendClient = Box<dyn Frontend>;

pub enum ClientType
{
    Rpc(RpcPeer),
    Direct(FrontendClient)
}

/// An interface to the frontend.
///   originally- Client(RpcPeer). So I just created an enum of the type of client we want, one member with rpc, one with Frontend trait, 
///   and calls to Client match the enum and continue on with Json if its the old RpcPeer, or direct calls to the Frontend trait if not
///   probably a rust central way to do this. just pushing forward for sake of educational momentum
pub struct Client(ClientType);

impl Client {
    pub fn new(peer: RpcPeer) -> Self {
        Client(ClientType::Rpc(peer))
    }

    pub fn update_view(&self, view_id: ViewId, update: &Update) {
        match &self.0 {
            ClientType::Rpc(r) => r.send_rpc_notification(
                "update",
                &json!({
                    "view_id": view_id,
                    "update": update,
                }),
            ),
            ClientType::Direct(fe) => fe.update_view(view_id, update),
        };
    }

    pub fn scroll_to(&self, view_id: ViewId, line: usize, col: usize) {
        match &self.0 {
            ClientType::Rpc(r) => r.send_rpc_notification(
                "scroll_to",
                &json!({
                    "view_id": view_id,
                    "line": line,
                    "col": col,
                })
            ),
            ClientType :: Direct(fe) => fe.scroll_to(view_id, line, col),
        };
    }

    pub fn config_changed(&self, view_id: ViewId, changes: &Table) {
        match &self.0 {
            ClientType::Rpc(r) => r.send_rpc_notification(
                "config_changed",
                &json!({
                    "view_id": view_id,
                    "changes": changes,
                })),
            ClientType::Direct(fe)=> fe.config_changed(view_id, changes),
        };
    }

    pub fn available_themes(&self, theme_names: Vec<String>) {
        match &self.0 {
            ClientType::Rpc(r) => 
                r.send_rpc_notification("available_themes", &json!({ "themes": theme_names })),
            ClientType::Direct(fe) => fe.available_themes(theme_names),
        };
    }

    pub fn available_languages(&self, languages: Vec<LanguageId>) {
        match &self.0 {
            ClientType::Rpc(r) => r.send_rpc_notification("available_languages", &json!({ "languages": languages })),
            ClientType::Direct(d) => d.available_languages(languages),
        };
    }

    pub fn theme_changed(&self, name: &str, theme: &ThemeSettings) {
        match &self.0 {
            ClientType::Rpc(r) => r.send_rpc_notification(
                "theme_changed",
                &json!({
                    "name": name,
                    "theme": theme,
                })),
            ClientType::Direct(fe) => fe.theme_changed(name, theme),
         };
    }

    pub fn language_changed(&self, view_id: ViewId, new_lang: &LanguageId) {
        match &self.0 {
            ClientType::Rpc(r) => r.send_rpc_notification(
                "language_changed",
                &json!({
                    "view_id": view_id,
                    "language_id": new_lang,
                })),
            ClientType::Direct(fe) => fe.language_changed(view_id, new_lang),
        };
    }

    pub fn plugin_started(&self, view_id: ViewId, plugin: &str) {
        match &self.0 {
            ClientType::Rpc(r) => r.send_rpc_notification(
                "plugin_started",
                &json!({
                    "view_id": view_id,
                    "plugin": plugin,
                })),
            ClientType::Direct(fe) => fe.plugin_started(view_id, plugin),
        };
    }

    pub fn plugin_stopped(&self, view_id: ViewId, plugin: &str, code: i32) {
        match &self.0 {
            ClientType::Rpc(r) => r.send_rpc_notification(
                "plugin_stopped",
                &json!({
                    "view_id": view_id,
                    "plugin": plugin,
                    "code": code,
                })),
            ClientType::Direct(fe) => fe.plugin_started(view_id, plugin),
        }
    }

    pub fn available_plugins(&self, view_id: ViewId, plugins: &[ClientPluginInfo]) {
        match &self.0 {
            ClientType::Rpc(r) => r.send_rpc_notification(
                "available_plugins",
                &json!({
                    "view_id": view_id,
                    "plugins": plugins })),
            ClientType::Direct(fe) => fe.available_plugins(view_id, plugins),
        }
    }

    pub fn update_cmds(&self, view_id: ViewId, plugin: &str, cmds: &[Command]) {
        match &self.0 {
            ClientType::Rpc(r) => r.send_rpc_notification(
            "update_cmds",
            &json!({
                "view_id": view_id,
                "plugin": plugin,
                "cmds": cmds,
            })),
            ClientType::Direct(fe) => fe.update_cmds(view_id, plugin, cmds),
        }    
    }

    pub fn def_style(&self, style: &Style) {
        match &self.0 {
            ClientType::Rpc(r) => r.send_rpc_notification("def_style", &json!(style)),
            ClientType::Direct(fe) => fe.def_style(style),
        };
    }

    pub fn find_status(&self, view_id: ViewId, queries: &Vec<FindStatus>) {
        match &self.0 {
            ClientType::Rpc(r) => r.send_rpc_notification(
                "find_status",
                &json!({
                    "view_id": view_id,
                    "queries": queries,
                })),
            ClientType::Direct(fe) => fe.find_status(view_id, &queries),
        };
    }

    pub fn replace_status(&self, view_id: ViewId, replace: &Replace) {
        match &self.0 {
            ClientType::Rpc(r) => r.send_rpc_notification(
                "replace_status",
                &json!({
                    "view_id": view_id,
                    "status": replace,
                })),
            ClientType::Direct(fe) => fe.replace_status(view_id, &replace),
        };
    }

    pub fn measure_width(&self, reqs: &[WidthReq]) -> Result<WidthResponse, xi_rpc::Error> {
        match &self.0 {
            ClientType::Rpc(r) => {
                let req_json = serde_json::to_value(reqs).expect("failed to serialize width req");
                let resp = r.send_rpc_request("measure_width", &req_json)?;
                Ok(serde_json::from_value(resp).expect("failed to deserialize width response"))
            },
            ClientType::Direct(fe)=> Ok(fe.measure_width(reqs)),
        }
    }

    pub fn alert<S: AsRef<str>>(&self, msg: S) {
        match &self.0 {
            ClientType::Rpc(r) => r.send_rpc_notification("alert", &json!({ "msg": msg.as_ref() })),
            ClientType::Direct(_) => (),
        };
    }

    pub fn add_status_item(
        &self,
        view_id: ViewId,
        source: &str,
        key: &str,
        value: &str,
        alignment: &str,
    ) {
        match &self.0 {
            ClientType::Rpc(r) => r.send_rpc_notification(
                "add_status_item",
                &json!({
                    "view_id": view_id,
                    "source": source,
                    "key": key,
                    "value": value,
                    "alignment": alignment
                })
            ),
            ClientType::Direct(fe) => fe.add_status_item(view_id, source, key, value, alignment),
        };
    }

    pub fn update_status_item(&self, view_id: ViewId, key: &str, value: &str) {
        match &self.0 {
            ClientType::Rpc(r) => r.send_rpc_notification(
                "update_status_item",
                &json!({
                    "view_id": view_id,
                    "key": key,
                    "value": value,
                })
            ),
            ClientType::Direct(fe) => fe.update_status_item(view_id, key, value),
        };
    }

    pub fn remove_status_item(&self, view_id: ViewId, key: &str) {
        match &self.0 {
            ClientType::Rpc(r) => r.send_rpc_notification(
                "remove_status_item",
                &json!({
                    "view_id": view_id,
                    "key": key,
                })),
            ClientType::Direct(fe)=>fe.remove_status_item(view_id, key),
        };
    }

    pub fn show_hover(&self, view_id: ViewId, request_id: usize, result: String) {
        match &self.0 {
            ClientType::Rpc(r) => r.send_rpc_notification(
                "show_hover",
                &json!({
                    "view_id": view_id,
                    "request_id": request_id,
                    "result": result,
                })
            ),
            ClientType::Direct(fe) => fe.show_hover(view_id, request_id, result),
        };
    }

    pub fn schedule_idle(&self, token: usize) {
        match &self.0 {
            ClientType::Rpc(r) => r.schedule_idle(token),
            ClientType::Direct(fe) => fe.schedule_idle(token),
        };
    }

    pub fn schedule_timer(&self, timeout: Instant, token: usize) {
        match &self.0 {
            ClientType::Rpc(r) => r.schedule_timer(timeout, token),
            ClientType::Direct(fe) => fe.schedule_timer(timeout, token),
        };
    }
}

#[derive(Debug, Serialize)]
pub struct Update {
    pub(crate) ops: Vec<UpdateOp>,
    pub(crate) pristine: bool,
    pub(crate) annotations: Vec<Value>,
}

#[derive(Debug, Serialize)]
pub(crate) struct UpdateOp {
    op: OpType,
    n: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    lines: Option<Vec<Value>>,
    #[serde(rename = "ln")]
    #[serde(skip_serializing_if = "Option::is_none")]
    first_line_number: Option<usize>,
}

impl UpdateOp {
    pub(crate) fn invalidate(n: usize) -> Self {
        UpdateOp { op: OpType::Invalidate, n, lines: None, first_line_number: None }
    }

    pub(crate) fn skip(n: usize) -> Self {
        UpdateOp { op: OpType::Skip, n, lines: None, first_line_number: None }
    }

    pub(crate) fn copy(n: usize, line: usize) -> Self {
        UpdateOp { op: OpType::Copy, n, lines: None, first_line_number: Some(line) }
    }

    pub(crate) fn insert(lines: Vec<Value>) -> Self {
        UpdateOp { op: OpType::Insert, n: lines.len(), lines: Some(lines), first_line_number: None }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
enum OpType {
    #[serde(rename = "ins")]
    Insert,
    Skip,
    Invalidate,
    Copy,
}

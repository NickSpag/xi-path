use xi_core_lib::client::Frontend;

pub struct PathfinderFrontend{
}

impl PathfinderFrontend {
    fn new() -> Self {
        PathfinderFrontend { }
    }
}

impl Frontend for PathfinderFrontend{

    fn update_view(&self, view_id: xi_core_lib::ViewId, update: &xi_core_lib::client::Update) {
        todo!()
    }
    fn scroll_to(&self, view_id: xi_core_lib::ViewId, line: usize, col: usize) {
        todo!()
    }
    fn config_changed(&self, view_id: xi_core_lib::ViewId, changes: &xi_core_lib::ConfigTable) {
        todo!()
    }
    fn available_themes(&self, theme_names: Vec<String>) {
        todo!()
    }
    fn available_languages(&self, languages: Vec<xi_core_lib::LanguageId>) {
        todo!()
    }
    fn theme_changed(&self, name: &str, theme: &xi_core_lib::styles::ThemeSettings) {
        todo!()
    }
    fn language_changed(&self, view_id: xi_core_lib::ViewId, new_lang: &xi_core_lib::LanguageId) {
        todo!()
    }
    fn plugin_started(&self, view_id: xi_core_lib::ViewId, plugin: &str) {
        todo!()
    }
    fn plugin_stopped(&self, view_id: xi_core_lib::ViewId, plugin: &str, code: i32) {
        todo!()
    }
    fn available_plugins(&self, view_id: xi_core_lib::ViewId, plugins: &[xi_core_lib::plugin_rpc::ClientPluginInfo]) {
        todo!()
    }
    fn update_cmds(&self, view_id: xi_core_lib::ViewId, plugin: &str, cmds: &[xi_core_lib::plugin_manifest::Command]) {
        todo!()
    }
    fn def_style(&self, style: &Value)  {
        todo!()
    }
    fn find_status(&self, view_id: xi_core_lib::ViewId, queries: &Value) {
        todo!()
    }
    fn replace_status(&self, view_id: xi_core_lib::ViewId, replace: &Value) {
        todo!()
    }
    fn measure_width(&self, reqs: &[xi_core_lib::width_cache::WidthReq]) -> Result<xi_core_lib::width_cache::WidthResponse, xi_rpc::Error> {
        todo!()
    }
    fn add_status_item(
        &self,
        view_id: xi_core_lib::ViewId,
        source: &str,
        key: &str,
        value: &str,
        alignment: &str,
    ) {
        todo!()
    }
    fn update_status_item(&self, view_id: xi_core_lib::ViewId, key: &str, value: &str) {
        todo!()
    }
    fn remove_status_item(&self, view_id: xi_core_lib::ViewId, key: &str) {
        todo!()
    }
    fn show_hover(&self, view_id: xi_core_lib::ViewId, request_id: usize, result: String) {
        todo!()
    }
    fn schedule_idle(&self, token: usize) {
        todo!()
    }
    fn schedule_timer(&self, timeout: std::time::Instant, token: usize) {
        todo!()
    }
}
use xi_core_lib:: {
    ViewId, ConfigTable,LanguageId,
    styles:: { Style, ThemeSettings }, 
    client:: { Frontend, Update }, 
    find::FindStatus, 
    view::Replace,
    plugin_manifest::Command,
    plugin_rpc::ClientPluginInfo,
    width_cache:: { WidthReq, WidthResponse }
};

pub struct PathfinderFrontend{
}

impl PathfinderFrontend {
    fn new() -> Self {
        PathfinderFrontend { }
    }
}

impl Frontend for PathfinderFrontend{

    fn update_view(&self, view_id: ViewId, update: &Update) {
        todo!()
    }
    fn scroll_to(&self, view_id: ViewId, line: usize, col: usize) {
        todo!()
    }
    fn config_changed(&self, view_id: ViewId, changes: &ConfigTable) {
        todo!()
    }
    fn available_themes(&self, theme_names: Vec<String>) {
        todo!()
    }
    fn available_languages(&self, languages: Vec<LanguageId>) {
        todo!()
    }
    fn theme_changed(&self, name: &str, theme: &ThemeSettings) {
        todo!()
    }
    fn language_changed(&self, view_id: ViewId, new_lang: &LanguageId) {
        todo!()
    }
    fn plugin_started(&self, view_id: ViewId, plugin: &str) {
        todo!()
    }
    fn plugin_stopped(&self, view_id: ViewId, plugin: &str, code: i32) {
        todo!()
    }
    fn available_plugins(&self, view_id: ViewId, plugins: &[ClientPluginInfo]) {
        todo!()
    }
    fn update_cmds(&self, view_id: ViewId, plugin: &str, cmds: &[Command]) {
        todo!()
    }
    fn def_style(&self, style: &Style)  {
        todo!()
    }
    fn find_status(&self, view_id: ViewId, queries: &Vec<FindStatus>) {
        todo!()
    }
    fn replace_status(&self, view_id: ViewId, replace: &Replace) {
        todo!()
    }
    fn measure_width(&self, reqs: &[xi_core_lib::width_cache::WidthReq]) -> Result<xi_core_lib::width_cache::WidthResponse, xi_rpc::Error> {
        todo!()
    }
    fn add_status_item(
        &self,
        view_id: ViewId,
        source: &str,
        key: &str,
        value: &str,
        alignment: &str,
    ) {
        todo!()
    }
    fn update_status_item(&self, view_id: ViewId, key: &str, value: &str) {
        todo!()
    }
    fn remove_status_item(&self, view_id: ViewId, key: &str) {
        todo!()
    }
    fn show_hover(&self, view_id: ViewId, request_id: usize, result: String) {
        todo!()
    }
    fn schedule_idle(&self, token: usize) {
        todo!()
    }
    fn schedule_timer(&self, timeout: std::time::Instant, token: usize) {
        todo!()
    }
}
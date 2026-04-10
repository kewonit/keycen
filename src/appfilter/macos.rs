use super::ActiveWindowDetector;

pub struct MacosDetector;

impl MacosDetector {
    pub fn new() -> Self {
        MacosDetector
    }
}

impl ActiveWindowDetector for MacosDetector {
    fn get_focused_process_name(&self) -> Option<String> {
        use objc2_app_kit::NSWorkspace;

        let workspace = unsafe { NSWorkspace::sharedWorkspace() };
        let app = unsafe { workspace.frontmostApplication()? };
        let name = unsafe { app.localizedName()? };
        Some(name.to_string())
    }
}

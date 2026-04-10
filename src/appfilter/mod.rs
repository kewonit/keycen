#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

use std::collections::HashSet;

/// Trait for detecting the currently focused application
pub trait ActiveWindowDetector: Send {
    /// Get the process name of the currently focused window
    /// Returns None if detection fails
    fn get_focused_process_name(&self) -> Option<String>;
}

/// App filter that checks exclusion lists
pub struct AppFilter {
    detector: Box<dyn ActiveWindowDetector>,
    excluded_apps: HashSet<String>, // lowercase process names
}

impl AppFilter {
    pub fn new(excluded_apps: Vec<String>) -> Self {
        let excluded: HashSet<String> = excluded_apps
            .into_iter()
            .map(|s| s.to_lowercase())
            .collect();

        AppFilter {
            detector: create_detector(),
            excluded_apps: excluded,
        }
    }

    /// Check if the currently focused app is in the exclusion list
    pub fn is_excluded(&self) -> bool {
        if self.excluded_apps.is_empty() {
            return false;
        }

        match self.detector.get_focused_process_name() {
            Some(name) => self.excluded_apps.contains(&name.to_lowercase()),
            None => false, // If we can't detect, don't exclude (safe default)
        }
    }

    /// Update the exclusion list (e.g., after config reload)
    pub fn update_exclusions(&mut self, excluded_apps: Vec<String>) {
        self.excluded_apps = excluded_apps
            .into_iter()
            .map(|s| s.to_lowercase())
            .collect();
    }
}

/// Create the platform-specific detector
fn create_detector() -> Box<dyn ActiveWindowDetector> {
    #[cfg(target_os = "windows")]
    {
        Box::new(windows::WindowsDetector::new())
    }

    #[cfg(target_os = "linux")]
    {
        Box::new(linux::LinuxDetector::new())
    }

    #[cfg(target_os = "macos")]
    {
        Box::new(macos::MacosDetector::new())
    }
}

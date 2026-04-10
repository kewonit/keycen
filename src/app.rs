use crate::appfilter::AppFilter;
use crate::config::watcher::ConfigWatcher;
use crate::config::AppConfig;
use crate::filter::rustrict_filter::RustrictFilter;
use crate::filter::ProfanityFilter;
use crate::input::{self, InputMode};
use crate::tray;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tray_icon::menu::MenuEvent;

pub struct App {
    config: AppConfig,
    config_path: std::path::PathBuf,
    daemon_mode: bool,
}

impl App {
    pub fn new(config: AppConfig, config_path: std::path::PathBuf, daemon_mode: bool) -> Self {
        App {
            config,
            config_path,
            daemon_mode,
        }
    }

    pub fn run(self) {
        // Create the profanity filter
        let filter: Box<dyn ProfanityFilter> =
            Box::new(RustrictFilter::new(self.config.replacements.clone()));
        let filter = Arc::new(Mutex::new(filter));

        // Create the app filter
        let app_filter = AppFilter::new(self.config.exclusions.apps.clone());
        let app_filter = Arc::new(Mutex::new(app_filter));

        // Determine input mode
        let requested_mode = match self.config.general.mode.as_str() {
            "grab" => InputMode::Grab,
            "listen" => InputMode::Listen,
            _ => InputMode::Grab, // "auto" tries grab first
        };
        let is_auto = self.config.general.mode == "auto";

        // Start config watcher in a separate thread
        let config_path = self.config_path.clone();
        let filter_clone = filter.clone();
        let app_filter_clone = app_filter.clone();
        thread::spawn(move || {
            config_watch_loop(config_path, filter_clone, app_filter_clone);
        });

        log::info!(
            "Keycen enabled, mode: {}",
            if is_auto {
                "auto"
            } else {
                &self.config.general.mode
            }
        );

        if self.daemon_mode {
            // Daemon mode: no tray, just run input listener directly
            self.start_input_directly(filter, app_filter, requested_mode, is_auto);
        } else {
            // Normal mode: start input in a thread, show tray icon on main thread
            self.start_with_tray(filter, app_filter, requested_mode, is_auto);
        }
    }

    fn start_input_directly(
        &self,
        filter: Arc<Mutex<Box<dyn ProfanityFilter>>>,
        app_filter: Arc<Mutex<AppFilter>>,
        requested_mode: InputMode,
        is_auto: bool,
    ) {
        if requested_mode == InputMode::Grab {
            log::info!("Attempting grab mode...");
            let filter_clone = filter.clone();
            let app_filter_clone = app_filter.clone();

            match input::grab::start(filter_clone, app_filter_clone) {
                Ok(()) => return,
                Err(e) => {
                    if is_auto {
                        log::warn!("Grab mode failed ({e:?}), falling back to listen mode");
                    } else {
                        log::error!("Grab mode failed: {e:?}");
                        return;
                    }
                }
            }
        }

        log::info!("Starting listen mode...");
        if let Err(e) = input::listen::start(filter, app_filter) {
            log::error!("Listen mode failed: {e:?}");
        }
    }

    fn start_with_tray(
        &self,
        filter: Arc<Mutex<Box<dyn ProfanityFilter>>>,
        app_filter: Arc<Mutex<AppFilter>>,
        requested_mode: InputMode,
        is_auto: bool,
    ) {
        let enabled = self.config.general.enabled;

        // Create tray icon on the main thread
        let (tray, tray_menu) = tray::create_tray(enabled);
        log::info!("System tray icon created");

        // Shared enabled state for tray toggle
        let filter_for_tray = filter.clone();

        // Start input listener in a separate thread
        let filter_clone = filter.clone();
        let app_filter_clone = app_filter.clone();
        let mode_str = if is_auto { "auto" } else { "" }.to_string();
        thread::spawn(move || {
            if requested_mode == InputMode::Grab {
                log::info!("Attempting grab mode...");
                let fc = filter_clone.clone();
                let afc = app_filter_clone.clone();

                match input::grab::start(fc, afc) {
                    Ok(()) => return,
                    Err(e) => {
                        if mode_str == "auto" {
                            log::warn!("Grab mode failed ({e:?}), falling back to listen mode");
                        } else {
                            log::error!("Grab mode failed: {e:?}");
                            return;
                        }
                    }
                }
            }

            log::info!("Starting listen mode...");
            if let Err(e) = input::listen::start(filter_clone, app_filter_clone) {
                log::error!("Listen mode failed: {e:?}");
            }
        });

        // Main thread: pump Windows messages + handle tray menu events
        let mut current_enabled = enabled;
        loop {
            // Pump Windows messages so the tray context menu works
            #[cfg(target_os = "windows")]
            {
                use windows::Win32::UI::WindowsAndMessaging::{
                    DispatchMessageW, PeekMessageW, TranslateMessage, MSG, PM_REMOVE,
                };
                unsafe {
                    let mut msg = MSG::default();
                    while PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).as_bool() {
                        let _ = TranslateMessage(&msg);
                        DispatchMessageW(&msg);
                    }
                }
            }

            if let Ok(event) = MenuEvent::receiver().try_recv() {
                if event.id == tray_menu.toggle_item.id() {
                    current_enabled = !current_enabled;
                    if let Ok(mut f) = filter_for_tray.lock() {
                        f.set_enabled(current_enabled);
                    }
                    tray::update_tray_state(&tray, &tray_menu, current_enabled);
                    log::info!(
                        "Keycen {}",
                        if current_enabled {
                            "enabled"
                        } else {
                            "disabled"
                        }
                    );
                } else if event.id == tray_menu.reload_item.id() {
                    log::info!("Manual config reload requested");
                    match AppConfig::load(&self.config_path) {
                        Ok(new_config) => {
                            if let Ok(mut f) = filter_for_tray.lock() {
                                f.set_enabled(new_config.general.enabled);
                            }
                            if let Ok(mut af) = app_filter.lock() {
                                af.update_exclusions(new_config.exclusions.apps);
                            }
                            current_enabled = new_config.general.enabled;
                            tray::update_tray_state(&tray, &tray_menu, current_enabled);
                            log::info!("Config reloaded successfully");
                        }
                        Err(e) => {
                            log::error!("Failed to reload config: {e}");
                        }
                    }
                } else if event.id == tray_menu.quit_item.id() {
                    log::info!("Quit requested from tray menu");
                    std::process::exit(0);
                }
            }
            // Small sleep to avoid busy-waiting
            thread::sleep(Duration::from_millis(50));
        }
    }
}

fn config_watch_loop(
    config_path: std::path::PathBuf,
    filter: Arc<Mutex<Box<dyn ProfanityFilter>>>,
    app_filter: Arc<Mutex<AppFilter>>,
) {
    let watcher = match ConfigWatcher::new(config_path.clone()) {
        Ok(w) => w,
        Err(e) => {
            log::warn!("Failed to start config watcher: {e:?}");
            return;
        }
    };

    loop {
        if watcher.check_for_changes() {
            log::info!("Config file changed, reloading...");
            match AppConfig::load(&config_path) {
                Ok(new_config) => {
                    // Update filter: enabled state + replacement map
                    if let Ok(mut f) = filter.lock() {
                        f.set_enabled(new_config.general.enabled);
                        f.update_replacements(new_config.replacements.clone());
                    }
                    // Update app exclusions
                    if let Ok(mut af) = app_filter.lock() {
                        af.update_exclusions(new_config.exclusions.apps);
                    }
                    log::info!("Config reloaded successfully");
                }
                Err(e) => {
                    log::error!("Failed to reload config: {e}");
                }
            }
        }
        thread::sleep(Duration::from_secs(1));
    }
}

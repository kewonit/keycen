use tray_icon::menu::{Menu, MenuItem, PredefinedMenuItem};

pub struct TrayMenu {
    pub menu: Menu,
    pub toggle_item: MenuItem,
    pub reload_item: MenuItem,
    pub quit_item: MenuItem,
}

impl TrayMenu {
    pub fn new(enabled: bool) -> Self {
        let toggle_text = if enabled {
            "Disable Keycen"
        } else {
            "Enable Keycen"
        };

        let toggle_item = MenuItem::new(toggle_text, true, None);
        let reload_item = MenuItem::new("Reload Config", true, None);
        let quit_item = MenuItem::new("Quit", true, None);

        let menu = Menu::new();
        let _ = menu.append(&toggle_item);
        let _ = menu.append(&PredefinedMenuItem::separator());
        let _ = menu.append(&reload_item);
        let _ = menu.append(&PredefinedMenuItem::separator());
        let _ = menu.append(&quit_item);

        TrayMenu {
            menu,
            toggle_item,
            reload_item,
            quit_item,
        }
    }

    pub fn update_toggle_text(&self, enabled: bool) {
        let text = if enabled {
            "Disable Keycen"
        } else {
            "Enable Keycen"
        };
        self.toggle_item.set_text(text);
    }
}

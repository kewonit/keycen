pub mod menu;

use menu::TrayMenu;
use tray_icon::{Icon, TrayIcon, TrayIconBuilder};

/// Create an icon from RGBA data (simple colored square)
fn create_icon(enabled: bool) -> Icon {
    let size = 32u32;
    let mut rgba = Vec::with_capacity((size * size * 4) as usize);

    let (r, g, b) = if enabled {
        (76u8, 175u8, 80u8) // Green = active
    } else {
        (244u8, 67u8, 54u8) // Red = disabled
    };

    for _ in 0..(size * size) {
        rgba.push(r);
        rgba.push(g);
        rgba.push(b);
        rgba.push(255); // alpha
    }

    Icon::from_rgba(rgba, size, size).expect("Failed to create tray icon")
}

/// Build and return the tray icon
pub fn create_tray(enabled: bool) -> (TrayIcon, TrayMenu) {
    let icon = create_icon(enabled);
    let tray_menu = TrayMenu::new(enabled);

    let tooltip = if enabled {
        "Keycen - Active"
    } else {
        "Keycen - Disabled"
    };

    let tray = TrayIconBuilder::new()
        .with_icon(icon)
        .with_tooltip(tooltip)
        .with_menu(Box::new(tray_menu.menu.clone()))
        .build()
        .expect("Failed to create tray icon");

    (tray, tray_menu)
}

/// Update tray icon to reflect enabled/disabled state
pub fn update_tray_state(tray: &TrayIcon, tray_menu: &TrayMenu, enabled: bool) {
    let icon = create_icon(enabled);
    let tooltip = if enabled {
        "Keycen - Active"
    } else {
        "Keycen - Disabled"
    };

    let _ = tray.set_icon(Some(icon));
    let _ = tray.set_tooltip(Some(tooltip));
    tray_menu.update_toggle_text(enabled);
}

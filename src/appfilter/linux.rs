use super::ActiveWindowDetector;

pub struct LinuxDetector;

impl LinuxDetector {
    pub fn new() -> Self {
        LinuxDetector
    }
}

impl ActiveWindowDetector for LinuxDetector {
    fn get_focused_process_name(&self) -> Option<String> {
        // Try X11 first
        if let Some(name) = get_focused_via_x11() {
            return Some(name);
        }

        // Wayland fallback: limited — Wayland doesn't expose focused window info easily
        log::debug!("X11 detection failed, Wayland environments may not support app exclusion");
        None
    }
}

fn get_focused_via_x11() -> Option<String> {
    use x11rb::connection::Connection;
    use x11rb::protocol::xproto::{AtomEnum, ConnectionExt};

    let (conn, _screen) = x11rb::connect(None).ok()?;

    // Get the focused window
    let focus = conn.get_input_focus().ok()?.reply().ok()?;
    let window = focus.focus;

    // Get _NET_WM_PID property
    let pid_atom = conn
        .intern_atom(false, b"_NET_WM_PID")
        .ok()?
        .reply()
        .ok()?
        .atom;

    // Walk up the window tree to find a window with _NET_WM_PID
    let pid = get_window_pid(&conn, window, pid_atom)?;

    // Read process name from /proc
    let comm = std::fs::read_to_string(format!("/proc/{}/comm", pid)).ok()?;
    Some(comm.trim().to_string())
}

fn get_window_pid(
    conn: &impl x11rb::protocol::xproto::ConnectionExt,
    mut window: u32,
    pid_atom: u32,
) -> Option<u32> {
    for _ in 0..10 {
        let prop = conn
            .get_property(false, window, pid_atom, AtomEnum::CARDINAL, 0, 1)
            .ok()?
            .reply()
            .ok()?;

        if prop.value_len > 0 && prop.value.len() >= 4 {
            let pid =
                u32::from_ne_bytes([prop.value[0], prop.value[1], prop.value[2], prop.value[3]]);
            if pid > 0 {
                return Some(pid);
            }
        }

        // Try parent
        let tree = conn.query_tree(window).ok()?.reply().ok()?;
        if tree.parent == 0 || tree.parent == tree.root {
            break;
        }
        window = tree.parent;
    }
    None
}

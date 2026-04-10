pub mod grab_corrector;
pub mod listen_corrector;

use rdev::{simulate, EventType, Key, SimulateError};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

/// Global flag to prevent self-correction loops
/// When true, the input listener should ignore events
pub static IS_SIMULATING: AtomicBool = AtomicBool::new(false);

/// Flag indicating a correction is in progress (spawned thread is actively typing)
/// The grab callback uses this to pass through keys without buffering,
/// preventing interleaving of user keystrokes with correction output
pub static CORRECTION_IN_PROGRESS: AtomicBool = AtomicBool::new(false);

/// Delay between simulated keystrokes (ms)
/// Keep this minimal to avoid interleaving with user typing
const SIMULATE_DELAY_MS: u64 = 1;

/// Maximum word length we'll attempt to correct (safety limit)
const MAX_CORRECTION_LENGTH: usize = 50;

/// Send a single simulated key event
fn send_key(event_type: EventType) -> Result<(), SimulateError> {
    simulate(&event_type)?;
    thread::sleep(Duration::from_millis(SIMULATE_DELAY_MS));
    Ok(())
}

/// Send backspace N times
fn send_backspaces(count: usize) -> Result<(), SimulateError> {
    for _ in 0..count {
        send_key(EventType::KeyPress(Key::Backspace))?;
        send_key(EventType::KeyRelease(Key::Backspace))?;
    }
    Ok(())
}

/// Type a string character by character
fn type_string(text: &str) -> Result<(), SimulateError> {
    for ch in text.chars() {
        if let Some(key) = char_to_key(ch) {
            let needs_shift = ch.is_uppercase() || is_shifted_char(ch);
            if needs_shift {
                send_key(EventType::KeyPress(Key::ShiftLeft))?;
            }
            send_key(EventType::KeyPress(key))?;
            send_key(EventType::KeyRelease(key))?;
            if needs_shift {
                send_key(EventType::KeyRelease(Key::ShiftLeft))?;
            }
        }
    }
    Ok(())
}

/// Map a character to an rdev Key
fn char_to_key(ch: char) -> Option<Key> {
    match ch.to_lowercase().next()? {
        'a' => Some(Key::KeyA),
        'b' => Some(Key::KeyB),
        'c' => Some(Key::KeyC),
        'd' => Some(Key::KeyD),
        'e' => Some(Key::KeyE),
        'f' => Some(Key::KeyF),
        'g' => Some(Key::KeyG),
        'h' => Some(Key::KeyH),
        'i' => Some(Key::KeyI),
        'j' => Some(Key::KeyJ),
        'k' => Some(Key::KeyK),
        'l' => Some(Key::KeyL),
        'm' => Some(Key::KeyM),
        'n' => Some(Key::KeyN),
        'o' => Some(Key::KeyO),
        'p' => Some(Key::KeyP),
        'q' => Some(Key::KeyQ),
        'r' => Some(Key::KeyR),
        's' => Some(Key::KeyS),
        't' => Some(Key::KeyT),
        'u' => Some(Key::KeyU),
        'v' => Some(Key::KeyV),
        'w' => Some(Key::KeyW),
        'x' => Some(Key::KeyX),
        'y' => Some(Key::KeyY),
        'z' => Some(Key::KeyZ),
        '0' => Some(Key::Num0),
        '1' => Some(Key::Num1),
        '2' => Some(Key::Num2),
        '3' => Some(Key::Num3),
        '4' => Some(Key::Num4),
        '5' => Some(Key::Num5),
        '6' => Some(Key::Num6),
        '7' => Some(Key::Num7),
        '8' => Some(Key::Num8),
        '9' => Some(Key::Num9),
        ' ' => Some(Key::Space),
        '\n' => Some(Key::Return),
        _ => None,
    }
}

/// Check if a character requires Shift to type
fn is_shifted_char(ch: char) -> bool {
    matches!(
        ch,
        '!' | '@' | '#' | '$' | '%' | '^' | '&' | '*' | '(' | ')'
    )
}

/// Execute a correction with the simulation guard active
pub fn execute_with_guard<F>(f: F)
where
    F: FnOnce(),
{
    CORRECTION_IN_PROGRESS.store(true, Ordering::SeqCst);
    IS_SIMULATING.store(true, Ordering::SeqCst);
    f();
    // Small delay to ensure all simulated events are processed before we resume listening
    thread::sleep(Duration::from_millis(5));
    IS_SIMULATING.store(false, Ordering::SeqCst);
    CORRECTION_IN_PROGRESS.store(false, Ordering::SeqCst);
}

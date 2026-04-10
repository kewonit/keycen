use super::{
    char_to_key, execute_with_guard, send_backspaces, send_key, type_string, MAX_CORRECTION_LENGTH,
};
use rdev::EventType;

/// Correct a word in listen mode
/// In listen mode, the word AND the boundary character have already been delivered to the app
/// We need to: backspace the word + boundary, then type replacement + boundary
pub fn correct(word: &str, replacement: &str, boundary: char) {
    if word.len() > MAX_CORRECTION_LENGTH || replacement.len() > MAX_CORRECTION_LENGTH {
        log::warn!(
            "Skipping correction: word ({}) or replacement ({}) exceeds max length",
            word.len(),
            replacement.len()
        );
        return;
    }

    log::info!("Listen correction: '{word}' -> '{replacement}' (boundary: {boundary:?})");

    execute_with_guard(|| {
        // Erase the typed word + boundary character
        let erase_count = word.chars().count() + 1; // +1 for boundary
        if let Err(e) = send_backspaces(erase_count) {
            log::error!("Failed to send backspaces: {e:?}");
            return;
        }

        // Type the replacement
        if let Err(e) = type_string(replacement) {
            log::error!("Failed to type replacement: {e:?}");
            return;
        }

        // Type the boundary character back
        if let Some(key) = char_to_key(boundary) {
            let _ = send_key(EventType::KeyPress(key));
            let _ = send_key(EventType::KeyRelease(key));
        }
    });
}

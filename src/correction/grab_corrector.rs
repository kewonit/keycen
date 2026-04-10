use super::{execute_with_guard, send_backspaces, type_string, MAX_CORRECTION_LENGTH};

/// Correct a word in grab mode
/// In grab mode, the word has already been typed (each character was passed through)
/// The boundary character (space, enter, etc.) has also been passed through by the callback
/// We need to: backspace the word + boundary, type replacement + boundary
pub fn correct(word: &str, replacement: &str, boundary: char) {
    if word.len() > MAX_CORRECTION_LENGTH || replacement.len() > MAX_CORRECTION_LENGTH {
        log::warn!(
            "Skipping correction: word ({}) or replacement ({}) exceeds max length",
            word.len(),
            replacement.len()
        );
        return;
    }

    log::info!("Grab correction: '{word}' -> '{replacement}'");

    execute_with_guard(|| {
        // Backspace word + the boundary character that was already typed
        if let Err(e) = send_backspaces(word.chars().count() + 1) {
            log::error!("Failed to send backspaces: {e:?}");
            return;
        }
        // Type replacement followed by the boundary character
        let mut output = replacement.to_string();
        output.push(boundary);
        if let Err(e) = type_string(&output) {
            log::error!("Failed to type replacement: {e:?}");
        }
    });
}

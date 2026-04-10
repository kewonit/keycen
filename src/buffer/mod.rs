pub mod classifier;

use classifier::{classify_key, KeyClass};
use rdev::Key;

/// Result of processing a key event
#[derive(Debug, PartialEq)]
pub enum BufferAction {
    /// Character added to buffer, no action needed yet
    Buffered,
    /// A complete word was detected at a word boundary
    /// Contains (the word, the boundary character that terminated it)
    WordComplete(String, char),
    /// Buffer was reset (cursor movement, paste, etc.)
    Reset,
    /// Key was ignored (modifier, function key, etc.)
    Ignored,
}

/// Accumulates typed characters into words and detects word boundaries
pub struct WordBuffer {
    buffer: String,
    ctrl_held: bool,
}

impl WordBuffer {
    pub fn new() -> Self {
        WordBuffer {
            buffer: String::with_capacity(64),
            ctrl_held: false,
        }
    }

    /// Process a key press event and return the resulting action
    pub fn process_key_press(&mut self, key: Key, name: Option<&str>) -> BufferAction {
        // Track Ctrl state
        match key {
            Key::ControlLeft | Key::ControlRight => {
                self.ctrl_held = true;
                return BufferAction::Ignored;
            }
            _ => {}
        }

        let class = classify_key(key, name, self.ctrl_held);

        match class {
            KeyClass::WordChar(ch) => {
                self.buffer.push(ch);
                BufferAction::Buffered
            }
            KeyClass::WordBoundary(boundary) => {
                if self.buffer.is_empty() {
                    BufferAction::Ignored
                } else {
                    let word = self.buffer.clone();
                    self.buffer.clear();
                    BufferAction::WordComplete(word, boundary)
                }
            }
            KeyClass::Backspace => {
                self.buffer.pop();
                BufferAction::Buffered
            }
            KeyClass::BufferReset | KeyClass::Paste => {
                self.buffer.clear();
                BufferAction::Reset
            }
            KeyClass::Modifier => BufferAction::Ignored,
            KeyClass::Ignore => BufferAction::Ignored,
        }
    }

    /// Process a key release event
    pub fn process_key_release(&mut self, key: Key) {
        match key {
            Key::ControlLeft | Key::ControlRight => {
                self.ctrl_held = false;
            }
            _ => {}
        }
    }

    /// Clear the buffer
    pub fn clear(&mut self) {
        self.buffer.clear();
    }
}

impl Default for WordBuffer {
    fn default() -> Self {
        Self::new()
    }
}

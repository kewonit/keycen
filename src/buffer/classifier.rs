use rdev::Key;

/// Classifies what a key event means for the word buffer
#[derive(Debug, PartialEq)]
pub enum KeyClass {
    /// A character that's part of a word (letter, digit, apostrophe, hyphen)
    WordChar(char),
    /// A word boundary (space, punctuation) — triggers word evaluation
    WordBoundary(char),
    /// Backspace — remove last char from buffer
    Backspace,
    /// A key that resets the buffer entirely (arrows, home, end, etc.)
    BufferReset,
    /// A modifier key (shift, ctrl, alt) — ignore, don't affect buffer
    Modifier,
    /// Paste operation detected (Ctrl+V) — reset buffer
    Paste,
    /// Unknown/unhandled — ignore
    Ignore,
}

/// Classify a key event into a buffer action
pub fn classify_key(key: Key, name: Option<&str>, ctrl_held: bool) -> KeyClass {
    // Check for paste operation (Ctrl+V)
    if ctrl_held {
        match key {
            Key::KeyV => return KeyClass::Paste,
            Key::KeyZ | Key::KeyX => return KeyClass::BufferReset,
            _ => return KeyClass::Ignore, // Other ctrl combos don't produce text
        }
    }

    match key {
        // Modifiers — ignore
        Key::ShiftLeft
        | Key::ShiftRight
        | Key::ControlLeft
        | Key::ControlRight
        | Key::Alt
        | Key::AltGr
        | Key::MetaLeft
        | Key::MetaRight
        | Key::CapsLock
        | Key::NumLock => KeyClass::Modifier,

        // Backspace
        Key::Backspace => KeyClass::Backspace,

        // Buffer reset triggers (cursor movement, focus changes)
        Key::UpArrow
        | Key::DownArrow
        | Key::LeftArrow
        | Key::RightArrow
        | Key::Home
        | Key::End
        | Key::PageUp
        | Key::PageDown
        | Key::Tab
        | Key::Escape
        | Key::Delete => KeyClass::BufferReset,

        // Enter — treated as word boundary
        Key::Return | Key::KpReturn => KeyClass::WordBoundary('\n'),

        // Space — word boundary
        Key::Space => KeyClass::WordBoundary(' '),

        // Function keys, special keys — ignore
        Key::F1
        | Key::F2
        | Key::F3
        | Key::F4
        | Key::F5
        | Key::F6
        | Key::F7
        | Key::F8
        | Key::F9
        | Key::F10
        | Key::F11
        | Key::F12
        | Key::PrintScreen
        | Key::ScrollLock
        | Key::Pause
        | Key::Insert => KeyClass::Ignore,

        // All other keys: use the event name to determine character
        _ => {
            if let Some(name) = name {
                if let Some(ch) = name.chars().next() {
                    if name.len() <= 4 && ch.is_alphanumeric() {
                        KeyClass::WordChar(ch)
                    } else if is_word_boundary_char(ch) {
                        KeyClass::WordBoundary(ch)
                    } else if ch == '\'' || ch == '-' {
                        KeyClass::WordChar(ch)
                    } else {
                        KeyClass::WordBoundary(ch)
                    }
                } else {
                    KeyClass::Ignore
                }
            } else {
                KeyClass::Ignore
            }
        }
    }
}

fn is_word_boundary_char(c: char) -> bool {
    c == ' '
        || c == '\t'
        || c == '\n'
        || c == '\r'
        || c == '.'
        || c == ','
        || c == '!'
        || c == '?'
        || c == ';'
        || c == ':'
        || c == '"'
        || c == '('
        || c == ')'
        || c == '['
        || c == ']'
        || c == '{'
        || c == '}'
        || c == '/'
        || c == '\\'
        || c == '|'
        || c == '~'
        || c == '`'
        || c == '@'
        || c == '#'
        || c == '$'
        || c == '%'
        || c == '^'
        || c == '&'
        || c == '*'
        || c == '+'
        || c == '='
        || c == '<'
        || c == '>'
}

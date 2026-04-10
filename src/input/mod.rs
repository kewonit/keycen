pub mod grab;
pub mod listen;

/// The mode the input listener is operating in
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputMode {
    /// Intercept keystrokes before delivery (best UX, requires permissions)
    Grab,
    /// Observe keystrokes after delivery (fallback, uses backspace-replace)
    Listen,
}

impl std::fmt::Display for InputMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputMode::Grab => write!(f, "grab"),
            InputMode::Listen => write!(f, "listen"),
        }
    }
}

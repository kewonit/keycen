pub mod rustrict_filter;

/// Result of checking a word
#[derive(Debug)]
pub enum FilterResult {
    /// Word is clean — no action needed
    Clean,
    /// Word is profane — contains the replacement string
    Profane(String),
}

/// Trait for profanity filter implementations
pub trait ProfanityFilter: Send + Sync {
    /// Check a word and return the filter result
    fn check(&self, word: &str) -> FilterResult;

    /// Update the filter's enabled state
    fn set_enabled(&mut self, enabled: bool);

    /// Check if the filter is enabled
    #[allow(dead_code)]
    fn is_enabled(&self) -> bool;

    /// Update the replacement map (e.g., after config hot-reload)
    fn update_replacements(&mut self, replacements: std::collections::HashMap<String, String>);
}

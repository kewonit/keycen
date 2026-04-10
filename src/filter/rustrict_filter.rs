use super::{FilterResult, ProfanityFilter};
use rustrict::{CensorStr, Type};
use std::collections::{HashMap, HashSet};

/// Minimum word length to check for profanity (avoids false positives on short words)
const MIN_WORD_LENGTH: usize = 4;

/// Common English words that rustrict over-flags as inappropriate.
/// These are words with legitimate everyday use that should not be corrected.
const FALSE_POSITIVE_ALLOWLIST: &[&str] = &[
    "suck", "sucks", "sucked", "sucking", "screw", "screwed", "screwing", "cock", "cocktail",
    "cockpit", "cocky", "boob", "boobs", "booby", "balls", "baller", "nuts", "nutty", "bang",
    "bangs", "banging", "banger", "blow", "blows", "blowing", "blown", "come", "comes", "coming",
    "hard", "harder", "hardest", "hoe", "hoes", "hooker", "strip", "strips", "stripper", "weed",
    "weeds", "crack", "cracker", "crackers", "cracking", "spank", "spanked", "kinky", "horny",
    "sexy", "erect", "erected", "organ", "organs", "rubber", "thong", "virgin", "anal",
];

pub struct RustrictFilter {
    enabled: bool,
    replacement_map: HashMap<String, String>,
    /// Set of safe words (replacement values) that should never be flagged
    safe_words: HashSet<String>,
}

impl RustrictFilter {
    pub fn new(replacement_map: HashMap<String, String>) -> Self {
        let safe_words = Self::build_safe_words(&replacement_map);
        RustrictFilter {
            enabled: true,
            replacement_map,
            safe_words,
        }
    }

    /// Build the safe-word set from replacement values
    fn build_safe_words(replacement_map: &HashMap<String, String>) -> HashSet<String> {
        replacement_map.values().map(|v| v.to_lowercase()).collect()
    }
}

impl ProfanityFilter for RustrictFilter {
    fn check(&self, word: &str) -> FilterResult {
        if !self.enabled {
            return FilterResult::Clean;
        }

        // Skip words that are too short
        if word.len() < MIN_WORD_LENGTH {
            return FilterResult::Clean;
        }

        // Skip words that are our own replacement words (prevents re-correction loops)
        let lower = word.to_lowercase();
        if self.safe_words.contains(&lower) {
            return FilterResult::Clean;
        }

        // Skip common false positives (everyday English words rustrict over-flags)
        if FALSE_POSITIVE_ALLOWLIST.iter().any(|&w| w == lower) {
            return FilterResult::Clean;
        }

        // Use rustrict to check if the word is inappropriate
        if word.is(Type::INAPPROPRIATE) {
            // Check custom replacement map first
            if let Some(replacement) = self.replacement_map.get(&lower) {
                // Preserve original casing pattern for replacement
                let replacement = match_case(word, replacement);
                FilterResult::Profane(replacement)
            } else {
                // Fall back to rustrict's built-in censoring (asterisks)
                let censored: String = word.censor();
                FilterResult::Profane(censored)
            }
        } else {
            FilterResult::Clean
        }
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn update_replacements(&mut self, replacements: HashMap<String, String>) {
        self.safe_words = Self::build_safe_words(&replacements);
        self.replacement_map = replacements;
    }
}

/// Attempt to match the case pattern of the original word in the replacement
/// e.g., "BAD" -> "GOOD", "Bad" -> "Good", "bad" -> "good"
fn match_case(original: &str, replacement: &str) -> String {
    if original.chars().all(|c| c.is_uppercase()) {
        // ALL CAPS
        replacement.to_uppercase()
    } else if original.chars().next().is_some_and(|c| c.is_uppercase()) {
        // Title Case
        let mut chars = replacement.chars();
        match chars.next() {
            Some(first) => {
                let upper: String = first.to_uppercase().collect();
                upper + chars.as_str()
            }
            None => replacement.to_string(),
        }
    } else {
        // lowercase
        replacement.to_lowercase()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_case_lowercase() {
        assert_eq!(match_case("hello", "world"), "world");
    }

    #[test]
    fn test_match_case_uppercase() {
        assert_eq!(match_case("HELLO", "world"), "WORLD");
    }

    #[test]
    fn test_match_case_titlecase() {
        assert_eq!(match_case("Hello", "world"), "World");
    }
}

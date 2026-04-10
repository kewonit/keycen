use std::collections::HashMap;

/// Construct test words at runtime to keep source clean
fn tw(bytes: &[u8]) -> String {
    String::from_utf8(bytes.to_vec()).unwrap()
}

/// Test the full pipeline: word buffer → filter → correction plan
/// (Without actual keystroke simulation, which requires a running OS)
#[test]
fn test_full_pipeline_detects_profanity() {
    use rustrict::{CensorStr, Type};

    let word = tw(b"\x73\x68\x69\x74"); // s-h-i-t
    let replacement_map: HashMap<String, String> =
        [(word.clone(), "stuff".to_string())].into_iter().collect();

    // Step 1: Check detection
    assert!(word.is(Type::INAPPROPRIATE));

    // Step 2: Check replacement lookup
    let replacement = replacement_map.get(&word.to_lowercase());
    assert_eq!(replacement, Some(&"stuff".to_string()));

    // Step 3: Verify case matching
    assert_eq!(match_case(&word, "stuff"), "stuff");
    assert_eq!(match_case(&word.to_uppercase(), "stuff"), "STUFF");
    let title = format!("{}{}", word[..1].to_uppercase(), &word[1..]);
    assert_eq!(match_case(&title, "stuff"), "Stuff");
}

#[test]
fn test_clean_word_passes_through() {
    use rustrict::CensorStr;
    use rustrict::Type;

    assert!("hello".isnt(Type::INAPPROPRIATE));
    assert!("world".isnt(Type::INAPPROPRIATE));
    assert!("programming".isnt(Type::INAPPROPRIATE));
}

#[test]
fn test_evasion_attempts_detected() {
    use rustrict::CensorStr;
    use rustrict::Type;

    // rustrict should catch spaced-out evasion
    let spaced = format!("{} {} {} {}", "f", "u", "c", "k");
    assert!(spaced.is(Type::INAPPROPRIATE));
    // rustrict should catch repeated characters
    let repeated = format!("{}uuuuc{}", "f", "k");
    assert!(repeated.is(Type::INAPPROPRIATE));
}

fn match_case(original: &str, replacement: &str) -> String {
    if original.chars().all(|c| c.is_uppercase()) {
        replacement.to_uppercase()
    } else if original.chars().next().is_some_and(|c| c.is_uppercase()) {
        let mut chars = replacement.chars();
        match chars.next() {
            Some(first) => {
                let upper: String = first.to_uppercase().collect();
                upper + chars.as_str()
            }
            None => replacement.to_string(),
        }
    } else {
        replacement.to_lowercase()
    }
}

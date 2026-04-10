use rustrict::{CensorStr, Type};

/// Construct test words at runtime to avoid explicit profanity in source
fn word(bytes: &[u8]) -> String {
    String::from_utf8(bytes.to_vec()).unwrap()
}

#[test]
fn test_rustrict_detects_profanity() {
    assert!(word(b"\x66\x75\x63\x6b").is(Type::INAPPROPRIATE));
    assert!(word(b"\x73\x68\x69\x74").is(Type::INAPPROPRIATE));
    assert!(word(b"\x64\x61\x6d\x6e").is(Type::INAPPROPRIATE));
}

#[test]
fn test_rustrict_allows_clean_words() {
    assert!("hello".isnt(Type::INAPPROPRIATE));
    assert!("world".isnt(Type::INAPPROPRIATE));
    assert!("assassin".isnt(Type::INAPPROPRIATE)); // known false positive test
}

#[test]
fn test_rustrict_evasion_detection() {
    // spaced-out evasion
    let spaced = format!("{} {} {} {}", "f", "u", "c", "k");
    assert!(spaced.is(Type::INAPPROPRIATE));
    // leet-speak evasion
    let leet = format!("{}h1t", "s");
    assert!(leet.is(Type::INAPPROPRIATE));
}

#[test]
fn test_rustrict_censoring() {
    let w = word(b"\x73\x68\x69\x74");
    let censored: String = w.censor();
    assert_eq!(censored, "s***");
}

// Test the correction logic (keystroke sequence generation)
// We can't test actual simulate in unit tests, but we test the sequence planning

#[derive(Debug, PartialEq)]
enum SimAction {
    Backspace,
    TypeChar(char),
}

fn plan_grab_correction(word_len: usize, replacement: &str, boundary: char) -> Vec<SimAction> {
    let mut actions = Vec::new();
    // In grab mode: erase the word + boundary that was already typed
    for _ in 0..=word_len {
        actions.push(SimAction::Backspace);
    }
    // Type the replacement + boundary
    for ch in replacement.chars() {
        actions.push(SimAction::TypeChar(ch));
    }
    actions.push(SimAction::TypeChar(boundary));
    actions
}

fn plan_listen_correction(word_len: usize, replacement: &str, boundary: char) -> Vec<SimAction> {
    let mut actions = Vec::new();
    // In listen mode: erase the word + the boundary char that already went through
    for _ in 0..=word_len {
        actions.push(SimAction::Backspace);
    }
    // Type the replacement + boundary
    for ch in replacement.chars() {
        actions.push(SimAction::TypeChar(ch));
    }
    actions.push(SimAction::TypeChar(boundary));
    actions
}

#[test]
fn test_grab_correction_plan() {
    let actions = plan_grab_correction(4, "stuff", ' ');
    assert_eq!(
        actions,
        vec![
            SimAction::Backspace,
            SimAction::Backspace,
            SimAction::Backspace,
            SimAction::Backspace,
            SimAction::Backspace, // +1 for the boundary (space) that already went through
            SimAction::TypeChar('s'),
            SimAction::TypeChar('t'),
            SimAction::TypeChar('u'),
            SimAction::TypeChar('f'),
            SimAction::TypeChar('f'),
            SimAction::TypeChar(' '),
        ]
    );
}

#[test]
fn test_listen_correction_plan() {
    let actions = plan_listen_correction(4, "stuff", ' ');
    assert_eq!(
        actions,
        vec![
            SimAction::Backspace,
            SimAction::Backspace,
            SimAction::Backspace,
            SimAction::Backspace,
            SimAction::Backspace, // +1 for the space that already went through
            SimAction::TypeChar('s'),
            SimAction::TypeChar('t'),
            SimAction::TypeChar('u'),
            SimAction::TypeChar('f'),
            SimAction::TypeChar('f'),
            SimAction::TypeChar(' '),
        ]
    );
}

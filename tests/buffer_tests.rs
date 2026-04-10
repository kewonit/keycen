// Tests for classification logic
mod buffer_logic {
    fn is_word_char(c: char) -> bool {
        c.is_alphanumeric() || c == '\'' || c == '-'
    }

    fn is_word_boundary(c: char) -> bool {
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
    }

    #[test]
    fn test_word_char_classification() {
        assert!(is_word_char('a'));
        assert!(is_word_char('Z'));
        assert!(is_word_char('5'));
        assert!(is_word_char('\'')); // don't → word char
        assert!(is_word_char('-')); // well-known → word char
        assert!(!is_word_char(' '));
        assert!(!is_word_char('.'));
        assert!(!is_word_char('!'));
    }

    #[test]
    fn test_word_boundary_classification() {
        assert!(is_word_boundary(' '));
        assert!(is_word_boundary('.'));
        assert!(is_word_boundary('!'));
        assert!(is_word_boundary('?'));
        assert!(!is_word_boundary('a'));
        assert!(!is_word_boundary('1'));
    }
}

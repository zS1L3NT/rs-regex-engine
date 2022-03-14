mod slashs {
    use super::super::*;
    use crate::lex_invalid;

    #[test]
    fn invalid_start() {
        lex_invalid!("abc/", "Expected RegExp to start with a </>");
    }

    #[test]
    fn invalid_end() {
        lex_invalid!("/abc", "Expected RegExp to end with a </>");
    }
}

mod length_check {
    use super::super::*;
    use crate::lex_valid;

    #[test]
    fn empty() {
        lex_valid!("//", 0);
    }

    #[test]
    fn literals() {
        lex_valid!("/abc/", 3);
    }

    #[test]
    fn anchors() {
        lex_valid!("/^abc$/", 5);
    }

    #[test]
    fn range() {
        lex_valid!("/abc{2,5}/", 4);
    }

    #[test]
    fn groups_and_brackets() {
        lex_valid!("/a(b|c)[^d]/", 9);
    }

    #[test]
    fn escapes() {
        lex_valid!("/\\ba\\b/", 3);
    }
}

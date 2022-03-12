mod slashs {
    use super::super::*;
    use crate::invalid;

    #[test]
    fn invalid_start() {
        invalid!("abc/", "Expected RegExp to start with a </>");
    }

    #[test]
    fn invalid_end() {
        invalid!("/abc", "Expected RegExp to end with a </>");
    }
}

mod length_check {
    use super::super::*;
    use crate::valid;

    #[test]
    fn empty() {
        valid!("//", 0);
    }

    #[test]
    fn literals() {
        valid!("/abc/", 3);
    }

    #[test]
    fn anchors() {
        valid!("/^abc$/", 5);
    }

    #[test]
    fn range() {
        valid!("/abc{2,5}/", 4);
    }

    #[test]
    fn groups_and_brackets() {
        valid!("/a(b|c)[^d]/", 9);
    }

    #[test]
    fn escapes() {
        valid!("/\\ba\\b/", 3);
    }
}

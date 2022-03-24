mod asterisk {
    use super::super::*;
    use crate::parse_valid;

    #[test]
    fn with_literal() {
        parse_valid!("/a*/", Single(Char('a', ZeroOrMore)));
    }
}

mod plus {
    use super::super::*;
    use crate::parse_valid;

    #[test]
    fn with_literal() {
        parse_valid!("/a+/", Single(Char('a', OneOrMore)));
    }
}

mod question_mark {
    use super::super::*;
    use crate::parse_valid;

    #[test]
    fn with_literal() {
        parse_valid!("/a?/", Single(Char('a', ZeroOrOne)));
    }
}

mod count {
    use super::super::*;
    use crate::parse_valid;

    #[test]
    fn as_literals() {
        parse_valid!(
            "/a{}/",
            Multiple(AND(vec![
                Single(Char('a', One)),
                Single(Char('{', One)),
                Single(Char('}', One))
            ]))
        );
    }

    #[test]
    fn with_literal() {
        parse_valid!("/a{1}/", Single(Char('a', Count(1))));
    }
}

mod range {
    use super::super::*;
    use crate::parse_valid;

    #[test]
    fn as_literals() {
        parse_valid!(
            "/a{,}/",
            Multiple(AND(vec![
                Single(Char('a', One)),
                Single(Char('{', One)),
                Single(Char(',', One)),
                Single(Char('}', One))
            ]))
        );
    }

    #[test]
    fn to_unlimited() {
        parse_valid!("/a{1,}/", Single(Char('a', Range(1, Option::None))));
    }

    #[test]
    fn fake() {
        parse_valid!(
            "/a{,1}/",
            Multiple(AND(vec![
                Single(Char('a', One)),
                Single(Char('{', One)),
                Single(Char(',', One)),
                Single(Char('1', One)),
                Single(Char('}', One))
            ]))
        );
    }

    #[test]
    fn full() {
        parse_valid!("/a{1,2}/", Single(Char('a', Range(1, Some(2)))));
    }

    #[test]
    fn nested_brackets() {
        parse_valid!(
            "/a{{1,}}/",
            Multiple(AND(vec![
                Single(Char('a', One)),
                Single(Char('{', Range(1, Option::None))),
                Single(Char('}', One))
            ]))
        );
    }
}

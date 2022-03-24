mod asterisk {
    use super::super::*;
    use crate::parse_valid;

    #[test]
    fn with_literal() {
        parse_valid!("/a*/", Single(Char('a', ZeroOrMore(2), 1)));
    }
}

mod plus {
    use super::super::*;
    use crate::parse_valid;

    #[test]
    fn with_literal() {
        parse_valid!("/a+/", Single(Char('a', OneOrMore(2), 1)));
    }
}

mod question_mark {
    use super::super::*;
    use crate::parse_valid;

    #[test]
    fn with_literal() {
        parse_valid!("/a?/", Single(Char('a', ZeroOrOne(2), 1)));
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
                Single(Char('a', One, 1)),
                Single(Char('{', One, 2)),
                Single(Char('}', One, 3))
            ]))
        );
    }

    #[test]
    fn with_literal() {
        parse_valid!("/a{1}/", Single(Char('a', Count(1, 2), 1)));
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
                Single(Char('a', One, 1)),
                Single(Char('{', One, 2)),
                Single(Char(',', One, 3)),
                Single(Char('}', One, 4))
            ]))
        );
    }

    #[test]
    fn to_unlimited() {
        parse_valid!("/a{1,}/", Single(Char('a', Range(1, Option::None, 2), 1)));
    }

    #[test]
    fn fake() {
        parse_valid!(
            "/a{,1}/",
            Multiple(AND(vec![
                Single(Char('a', One, 1)),
                Single(Char('{', One, 2)),
                Single(Char(',', One, 3)),
                Single(Char('1', One, 4)),
                Single(Char('}', One, 5))
            ]))
        );
    }

    #[test]
    fn full() {
        parse_valid!("/a{1,2}/", Single(Char('a', Range(1, Some(2), 2), 1)));
    }

    #[test]
    fn nested_brackets() {
        parse_valid!(
            "/a{{1,}}/",
            Multiple(AND(vec![
                Single(Char('a', One, 1)),
                Single(Char('{', Range(1, Option::None, 3), 2)),
                Single(Char('}', One, 7))
            ]))
        );
    }
}

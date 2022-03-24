mod asterisk {
    use super::super::*;
    use crate::lex_valid;

    #[test]
    fn with_literal() {
        lex_valid!("/a*/", vec![Literal('a', 1), Quantity(ZeroOrMore, 2)]);
    }
}

mod plus {
    use super::super::*;
    use crate::lex_valid;

    #[test]
    fn with_literal() {
        lex_valid!("/a+/", vec![Literal('a', 1), Quantity(OneOrMore, 2)]);
    }
}

mod question_mark {
    use super::super::*;
    use crate::lex_valid;

    #[test]
    fn with_literal() {
        lex_valid!("/a?/", vec![Literal('a', 1), Quantity(ZeroOrOne, 2)]);
    }
}

mod count {
    use super::super::*;
    use crate::lex_valid;

    #[test]
    fn as_literals() {
        lex_valid!("/a{}/", 3);
    }

    #[test]
    fn with_literal() {
        lex_valid!("/a{1}/", vec![Literal('a', 1), Quantity(Count(1), 2)]);
    }
}

mod range {
    use super::super::*;
    use crate::lex_valid;

    #[test]
    fn as_literals() {
        lex_valid!("/a{,}/", 4);
    }

    #[test]
    fn to_unlimited() {
        lex_valid!(
            "/a{1,}/",
            vec![Literal('a', 1), Quantity(Range(1, None), 2)]
        );
    }

    #[test]
    fn fake() {
        lex_valid!(
            "/a{,1}/",
            vec![
                Literal('a', 1),
                Literal('{', 2),
                Literal(',', 3),
                Literal('1', 4),
                Literal('}', 5)
            ]
        );
    }

    #[test]
    fn full() {
        lex_valid!(
            "/a{1,2}/",
            vec![Literal('a', 1), Quantity(Range(1, Some(2)), 2)]
        );
    }

    #[test]
    fn nested_brackets() {
        lex_valid!("/a{{1,}}/", 4);
    }
}

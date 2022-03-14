mod asterisk {
    use super::super::*;
    use crate::lex_valid;

    #[test]
    fn with_literal() {
        lex_valid!("/a*/", vec![Literal('a', 0), Quantifier(ZeroOrMore, 1)]);
    }
}

mod plus {
    use super::super::*;
    use crate::lex_valid;

    #[test]
    fn with_literal() {
        lex_valid!("/a+/", vec![Literal('a', 0), Quantifier(OneOrMore, 1)]);
    }
}

mod question_mark {
    use super::super::*;
    use crate::lex_valid;

    #[test]
    fn with_literal() {
        lex_valid!("/a?/", vec![Literal('a', 0), Quantifier(ZeroOrOne, 1)]);
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
        lex_valid!("/a{1}/", vec![Literal('a', 0), Quantifier(Count(1), 1)]);
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
            vec![Literal('a', 0), Quantifier(Range(1, None), 1)]
        );
    }

    #[test]
    fn fake() {
        lex_valid!(
            "/a{,1}/",
            vec![
                Literal('a', 0),
                Literal('{', 1),
                Literal(',', 2),
                Literal('1', 3),
                Literal('}', 4)
            ]
        );
    }

    #[test]
    fn full() {
        lex_valid!(
            "/a{1,2}/",
            vec![Literal('a', 0), Quantifier(Range(1, Some(2)), 1)]
        );
    }

    #[test]
    fn nested_brackets() {
        lex_valid!("/a{{1,}}/", 4);
    }
}

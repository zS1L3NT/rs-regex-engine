mod invalid {
    use super::super::*;
    use crate::lex_invalid;

    #[test]
    fn start() {
        lex_invalid!("/(/", "Incomplete group structure");
    }

    #[test]
    fn end() {
        lex_invalid!("/)/", "Unmatched parenthesis");
    }

    #[test]
    fn start_with_question_mark() {
        lex_invalid!("/(?/", "Incomplete group structure");
    }
}

mod capturing {
    use super::super::*;
    use crate::lex_valid;

    #[test]
    fn empty() {
        lex_valid!("/()/", vec![OpenGroup(Capturing, 0), CloseGroup(1)]);
    }

    #[test]
    fn with_literal() {
        lex_valid!(
            "/(a)/",
            vec![OpenGroup(Capturing, 0), Literal('a', 1), CloseGroup(2)]
        );
    }
}

mod non_capturing {
    use super::super::*;
    use crate::{lex_invalid, lex_valid};

    #[test]
    fn invalid() {
        lex_invalid!("/(?:/", "Incomplete group structure");
    }

    #[test]
    fn empty() {
        lex_valid!("/(?:)/", vec![OpenGroup(NonCapturing, 0), CloseGroup(3)]);
    }

    #[test]
    fn as_literals() {
        lex_valid!(
            "/(:?)/",
            vec![
                OpenGroup(Capturing, 0),
                Literal(':', 1),
                Quantifier(ZeroOrOne, 2),
                CloseGroup(3)
            ]
        );
    }

    #[test]
    fn with_literal() {
        lex_valid!(
            "/(?:x)/",
            vec![OpenGroup(NonCapturing, 0), Literal('x', 3), CloseGroup(4)]
        );
    }
}

mod positive_lookahead {
    use super::super::*;
    use crate::{lex_invalid, lex_valid};

    #[test]
    fn invalid() {
        lex_invalid!("/(?=/", "Incomplete group structure");
    }

    #[test]
    fn empty() {
        lex_valid!(
            "/(?=)/",
            vec![OpenGroup(PositiveLookAhead, 0), CloseGroup(3)]
        );
    }

    #[test]
    fn as_literals() {
        lex_valid!(
            "/(=?)/",
            vec![
                OpenGroup(Capturing, 0),
                Literal('=', 1),
                Quantifier(ZeroOrOne, 2),
                CloseGroup(3)
            ]
        );
    }

    #[test]
    fn with_literal() {
        lex_valid!(
            "/(?=x)/",
            vec![
                OpenGroup(PositiveLookAhead, 0),
                Literal('x', 3),
                CloseGroup(4)
            ]
        );
    }
}

mod positive_lookbehind {
    use super::super::*;
    use crate::{lex_invalid, lex_valid};

    #[test]
    fn invalid() {
        lex_invalid!("/(?</", "Incomplete group structure");
        lex_invalid!("/(?<=/", "Incomplete group structure");
    }

    #[test]
    fn empty() {
        lex_valid!(
            "/(?<=)/",
            vec![OpenGroup(PositiveLookBehind, 0), CloseGroup(4)]
        );
    }

    #[test]
    fn with_literal() {
        lex_valid!(
            "/(?<=x)/",
            vec![
                OpenGroup(PositiveLookBehind, 0),
                Literal('x', 4),
                CloseGroup(5)
            ]
        );
    }

    #[test]
    fn as_literals() {
        lex_valid!(
            "/(?=<x)/",
            vec![
                OpenGroup(PositiveLookAhead, 0),
                Literal('<', 3),
                Literal('x', 4),
                CloseGroup(5)
            ]
        );
    }
}

mod negative_lookahead {
    use super::super::*;
    use crate::{lex_invalid, lex_valid};

    #[test]
    fn invalid() {
        lex_invalid!("/(?!/", "Incomplete group structure");
    }

    #[test]
    fn empty() {
        lex_valid!(
            "/(?!)/",
            vec![OpenGroup(NegativeLookAhead, 0), CloseGroup(3)]
        );
    }

    #[test]
    fn as_literals() {
        lex_valid!(
            "/(!?)/",
            vec![
                OpenGroup(Capturing, 0),
                Literal('!', 1),
                Quantifier(ZeroOrOne, 2),
                CloseGroup(3)
            ]
        );
    }

    #[test]
    fn with_literal() {
        lex_valid!(
            "/(?!x)/",
            vec![
                OpenGroup(NegativeLookAhead, 0),
                Literal('x', 3),
                CloseGroup(4)
            ]
        );
    }
}

mod negative_lookbehind {
    use super::super::*;
    use crate::{lex_invalid, lex_valid};

    #[test]
    fn invalid() {
        lex_invalid!("/(?</", "Incomplete group structure");
        lex_invalid!("/(?<!/", "Incomplete group structure");
    }

    #[test]
    fn empty() {
        lex_valid!(
            "/(?<!)/",
            vec![OpenGroup(NegativeLookBehind, 0), CloseGroup(4)]
        );
    }

    #[test]
    fn with_literal() {
        lex_valid!(
            "/(?<!x)/",
            vec![
                OpenGroup(NegativeLookBehind, 0),
                Literal('x', 4),
                CloseGroup(5)
            ]
        );
    }
    #[test]
    fn as_literals() {
        lex_valid!(
            "/(?!<x)/",
            vec![
                OpenGroup(NegativeLookAhead, 0),
                Literal('<', 3),
                Literal('x', 4),
                CloseGroup(5)
            ]
        );
    }
}

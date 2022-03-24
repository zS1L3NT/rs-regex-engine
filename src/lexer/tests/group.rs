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
        lex_valid!("/()/", vec![OpenGroup(Capturing, 1), CloseGroup]);
    }

    #[test]
    fn with_literal() {
        lex_valid!(
            "/(a)/",
            vec![OpenGroup(Capturing, 1), Literal('a', 2), CloseGroup]
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
        lex_valid!("/(?:)/", vec![OpenGroup(NonCapturing, 1), CloseGroup]);
    }

    #[test]
    fn as_literals() {
        lex_valid!(
            "/(:?)/",
            vec![
                OpenGroup(Capturing, 1),
                Literal(':', 2),
                Quantity(ZeroOrOne, 3),
                CloseGroup
            ]
        );
    }

    #[test]
    fn with_literal() {
        lex_valid!(
            "/(?:x)/",
            vec![OpenGroup(NonCapturing, 1), Literal('x', 4), CloseGroup]
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
            vec![OpenGroup(PositiveLookAhead, 1), CloseGroup]
        );
    }

    #[test]
    fn as_literals() {
        lex_valid!(
            "/(=?)/",
            vec![
                OpenGroup(Capturing, 1),
                Literal('=', 2),
                Quantity(ZeroOrOne, 3),
                CloseGroup
            ]
        );
    }

    #[test]
    fn with_literal() {
        lex_valid!(
            "/(?=x)/",
            vec![
                OpenGroup(PositiveLookAhead, 1),
                Literal('x', 4),
                CloseGroup
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
            vec![OpenGroup(PositiveLookBehind, 1), CloseGroup]
        );
    }

    #[test]
    fn with_literal() {
        lex_valid!(
            "/(?<=x)/",
            vec![
                OpenGroup(PositiveLookBehind, 1),
                Literal('x', 5),
                CloseGroup
            ]
        );
    }

    #[test]
    fn as_literals() {
        lex_valid!(
            "/(?=<x)/",
            vec![
                OpenGroup(PositiveLookAhead, 1),
                Literal('<', 4),
                Literal('x', 5),
                CloseGroup
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
            vec![OpenGroup(NegativeLookAhead, 1), CloseGroup]
        );
    }

    #[test]
    fn as_literals() {
        lex_valid!(
            "/(!?)/",
            vec![
                OpenGroup(Capturing, 1),
                Literal('!', 2),
                Quantity(ZeroOrOne, 3),
                CloseGroup
            ]
        );
    }

    #[test]
    fn with_literal() {
        lex_valid!(
            "/(?!x)/",
            vec![
                OpenGroup(NegativeLookAhead, 1),
                Literal('x', 4),
                CloseGroup
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
            vec![OpenGroup(NegativeLookBehind, 1), CloseGroup]
        );
    }

    #[test]
    fn with_literal() {
        lex_valid!(
            "/(?<!x)/",
            vec![
                OpenGroup(NegativeLookBehind, 1),
                Literal('x', 5),
                CloseGroup
            ]
        );
    }
    #[test]
    fn as_literals() {
        lex_valid!(
            "/(?!<x)/",
            vec![
                OpenGroup(NegativeLookAhead, 1),
                Literal('<', 4),
                Literal('x', 5),
                CloseGroup
            ]
        );
    }
}

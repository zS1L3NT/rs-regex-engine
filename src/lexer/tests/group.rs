mod invalid {
    use super::super::*;
    use crate::invalid;

    #[test]
    fn start() {
        invalid!("/(/", "Incomplete group structure");
    }

    #[test]
    fn end() {
        invalid!("/)/", "Unmatched parenthesis");
    }

    #[test]
    fn start_with_question_mark() {
        invalid!("/(?/", "Incomplete group structure");
    }
}

mod capturing {
    use super::super::*;
    use crate::valid;

    #[test]
    fn empty() {
        valid!("/()/", vec![OpenGroup(Capturing, 0), CloseGroup(1)]);
    }

    #[test]
    fn with_literal() {
        valid!(
            "/(a)/",
            vec![OpenGroup(Capturing, 0), Literal('a', 1), CloseGroup(2)]
        );
    }
}

mod non_capturing {
    use super::super::*;
    use crate::{invalid, valid};

    #[test]
    fn invalid() {
        invalid!("/(?:/", "Incomplete group structure");
    }

    #[test]
    fn empty() {
        valid!("/(?:)/", vec![OpenGroup(NonCapturing, 0), CloseGroup(3)]);
    }

    #[test]
    fn as_literals() {
        valid!(
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
        valid!(
            "/(?:x)/",
            vec![OpenGroup(NonCapturing, 0), Literal('x', 3), CloseGroup(4)]
        );
    }
}

mod positive_lookahead {
    use super::super::*;
    use crate::{invalid, valid};

    #[test]
    fn invalid() {
        invalid!("/(?=/", "Incomplete group structure");
    }

    #[test]
    fn empty() {
        valid!(
            "/(?=)/",
            vec![OpenGroup(PositiveLookAhead, 0), CloseGroup(3)]
        );
    }

    #[test]
    fn as_literals() {
        valid!(
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
        valid!(
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
    use crate::{invalid, valid};

    #[test]
    fn invalid() {
        invalid!("/(?</", "Incomplete group structure");
        invalid!("/(?<=/", "Incomplete group structure");
    }

    #[test]
    fn empty() {
        valid!(
            "/(?<=)/",
            vec![OpenGroup(PositiveLookBehind, 0), CloseGroup(4)]
        );
    }

    #[test]
    fn with_literal() {
        valid!(
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
        valid!(
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
    use crate::{invalid, valid};

    #[test]
    fn invalid() {
        invalid!("/(?!/", "Incomplete group structure");
    }

    #[test]
    fn empty() {
        valid!(
            "/(?!)/",
            vec![OpenGroup(NegativeLookAhead, 0), CloseGroup(3)]
        );
    }

    #[test]
    fn as_literals() {
        valid!(
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
        valid!(
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
    use crate::{invalid, valid};

    #[test]
    fn invalid() {
        invalid!("/(?</", "Incomplete group structure");
        invalid!("/(?<!/", "Incomplete group structure");
    }

    #[test]
    fn empty() {
        valid!(
            "/(?<!)/",
            vec![OpenGroup(NegativeLookBehind, 0), CloseGroup(4)]
        );
    }

    #[test]
    fn with_literal() {
        valid!(
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
        valid!(
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

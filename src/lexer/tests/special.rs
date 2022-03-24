mod whitespace {
    use super::super::*;
    use crate::lex_valid;

    #[test]
    fn literal() {
        lex_valid!("/s/", vec![Literal('s', 1)]);
    }

    #[test]
    fn special() {
        lex_valid!("/\\s/", vec![Special(Whitespace, 1)]);
    }

    #[test]
    fn escaped() {
        lex_valid!("/\\\\s/", vec![Literal('\\', 1), Literal('s', 3)]);
    }
}

mod nonwhitespace {
    use super::super::*;
    use crate::lex_valid;

    #[test]
    fn literal() {
        lex_valid!("/S/", vec![Literal('S', 1)]);
    }

    #[test]
    fn special() {
        lex_valid!("/\\S/", vec![Special(NonWhitespace, 1)]);
    }

    #[test]
    fn escaped() {
        lex_valid!("/\\\\S/", vec![Literal('\\', 1), Literal('S', 3)]);
    }
}

mod digit {
    use super::super::*;
    use crate::lex_valid;

    #[test]
    fn literal() {
        lex_valid!("/d/", vec![Literal('d', 1)]);
    }

    #[test]
    fn special() {
        lex_valid!("/\\d/", vec![Special(Digit, 1)]);
    }

    #[test]
    fn escaped() {
        lex_valid!("/\\\\d/", vec![Literal('\\', 1), Literal('d', 3)]);
    }
}

mod nondigit {
    use super::super::*;
    use crate::lex_valid;

    #[test]
    fn literal() {
        lex_valid!("/D/", vec![Literal('D', 1)]);
    }

    #[test]
    fn special() {
        lex_valid!("/\\D/", vec![Special(NonDigit, 1)]);
    }

    #[test]
    fn escaped() {
        lex_valid!("/\\\\D/", vec![Literal('\\', 1), Literal('D', 3)]);
    }
}

mod word {
    use super::super::*;
    use crate::lex_valid;

    #[test]
    fn literal() {
        lex_valid!("/w/", vec![Literal('w', 1)]);
    }

    #[test]
    fn special() {
        lex_valid!("/\\w/", vec![Special(Word, 1)]);
    }

    #[test]
    fn escaped() {
        lex_valid!("/\\\\w/", vec![Literal('\\', 1), Literal('w', 3)]);
    }
}

mod nonword {
    use super::super::*;
    use crate::lex_valid;

    #[test]
    fn literal() {
        lex_valid!("/W/", vec![Literal('W', 1)]);
    }

    #[test]
    fn special() {
        lex_valid!("/\\W/", vec![Special(NonWord, 1)]);
    }

    #[test]
    fn escaped() {
        lex_valid!("/\\\\W/", vec![Literal('\\', 1), Literal('W', 3)]);
    }
}

mod boundary {
    use super::super::*;
    use crate::lex_valid;

    #[test]
    fn literal() {
        lex_valid!("/b/", vec![Literal('b', 1)]);
    }

    #[test]
    fn special() {
        lex_valid!("/\\b/", vec![Special(Boundary, 1)]);
    }

    #[test]
    fn escaped() {
        lex_valid!("/\\\\b/", vec![Literal('\\', 1), Literal('b', 3)]);
    }
}

mod nonboundary {
    use super::super::*;
    use crate::lex_valid;

    #[test]
    fn literal() {
        lex_valid!("/B/", vec![Literal('B', 1)]);
    }

    #[test]
    fn special() {
        lex_valid!("/\\B/", vec![Special(NonBoundary, 1)]);
    }

    #[test]
    fn escaped() {
        lex_valid!("/\\\\B/", vec![Literal('\\', 1), Literal('B', 3)]);
    }
}

mod linebreak {
    use super::super::*;
    use crate::lex_valid;

    #[test]
    fn literal() {
        lex_valid!("/n/", vec![Literal('n', 1)]);
    }

    #[test]
    fn special() {
        lex_valid!("/\\n/", vec![Special(LineBreak, 1)]);
    }

    #[test]
    fn escaped() {
        lex_valid!("/\\\\n/", vec![Literal('\\', 1), Literal('n', 3)]);
    }
}

mod carriagereturn {
    use super::super::*;
    use crate::lex_valid;

    #[test]
    fn literal() {
        lex_valid!("/r/", vec![Literal('r', 1)]);
    }

    #[test]
    fn special() {
        lex_valid!("/\\r/", vec![Special(CarriageReturn, 1)]);
    }

    #[test]
    fn escaped() {
        lex_valid!("/\\\\r/", vec![Literal('\\', 1), Literal('r', 3)]);
    }
}

mod tab {
    use super::super::*;
    use crate::lex_valid;

    #[test]
    fn literal() {
        lex_valid!("/t/", vec![Literal('t', 1)]);
    }

    #[test]
    fn special() {
        lex_valid!("/\\t/", vec![Special(Tab, 1)]);
    }

    #[test]
    fn escaped() {
        lex_valid!("/\\\\t/", vec![Literal('\\', 1), Literal('t', 3)]);
    }
}

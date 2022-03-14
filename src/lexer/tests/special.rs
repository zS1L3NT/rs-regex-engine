mod whitespace {
    use super::super::*;
    use crate::lex_valid;

    #[test]
    fn literal() {
        lex_valid!("/s/", vec![Literal('s', 0)]);
    }

    #[test]
    fn special() {
        lex_valid!("/\\s/", vec![Special(Whitespace, 0)]);
    }

    #[test]
    fn escaped() {
        lex_valid!("/\\\\s/", vec![Literal('\\', 0), Literal('s', 2)]);
    }
}

mod nonwhitespace {
    use super::super::*;
    use crate::lex_valid;

    #[test]
    fn literal() {
        lex_valid!("/S/", vec![Literal('S', 0)]);
    }

    #[test]
    fn special() {
        lex_valid!("/\\S/", vec![Special(NonWhitespace, 0)]);
    }

    #[test]
    fn escaped() {
        lex_valid!("/\\\\S/", vec![Literal('\\', 0), Literal('S', 2)]);
    }
}

mod digit {
    use super::super::*;
    use crate::lex_valid;

    #[test]
    fn literal() {
        lex_valid!("/d/", vec![Literal('d', 0)]);
    }

    #[test]
    fn special() {
        lex_valid!("/\\d/", vec![Special(Digit, 0)]);
    }

    #[test]
    fn escaped() {
        lex_valid!("/\\\\d/", vec![Literal('\\', 0), Literal('d', 2)]);
    }
}

mod nondigit {
    use super::super::*;
    use crate::lex_valid;

    #[test]
    fn literal() {
        lex_valid!("/D/", vec![Literal('D', 0)]);
    }

    #[test]
    fn special() {
        lex_valid!("/\\D/", vec![Special(NonDigit, 0)]);
    }

    #[test]
    fn escaped() {
        lex_valid!("/\\\\D/", vec![Literal('\\', 0), Literal('D', 2)]);
    }
}

mod word {
    use super::super::*;
    use crate::lex_valid;

    #[test]
    fn literal() {
        lex_valid!("/w/", vec![Literal('w', 0)]);
    }

    #[test]
    fn special() {
        lex_valid!("/\\w/", vec![Special(Word, 0)]);
    }

    #[test]
    fn escaped() {
        lex_valid!("/\\\\w/", vec![Literal('\\', 0), Literal('w', 2)]);
    }
}

mod nonword {
    use super::super::*;
    use crate::lex_valid;

    #[test]
    fn literal() {
        lex_valid!("/W/", vec![Literal('W', 0)]);
    }

    #[test]
    fn special() {
        lex_valid!("/\\W/", vec![Special(NonWord, 0)]);
    }

    #[test]
    fn escaped() {
        lex_valid!("/\\\\W/", vec![Literal('\\', 0), Literal('W', 2)]);
    }
}

mod boundary {
    use super::super::*;
    use crate::lex_valid;

    #[test]
    fn literal() {
        lex_valid!("/b/", vec![Literal('b', 0)]);
    }

    #[test]
    fn special() {
        lex_valid!("/\\b/", vec![Special(Boundary, 0)]);
    }

    #[test]
    fn escaped() {
        lex_valid!("/\\\\b/", vec![Literal('\\', 0), Literal('b', 2)]);
    }
}

mod nonboundary {
    use super::super::*;
    use crate::lex_valid;

    #[test]
    fn literal() {
        lex_valid!("/B/", vec![Literal('B', 0)]);
    }

    #[test]
    fn special() {
        lex_valid!("/\\B/", vec![Special(NonBoundary, 0)]);
    }

    #[test]
    fn escaped() {
        lex_valid!("/\\\\B/", vec![Literal('\\', 0), Literal('B', 2)]);
    }
}

mod linebreak {
    use super::super::*;
    use crate::lex_valid;

    #[test]
    fn literal() {
        lex_valid!("/n/", vec![Literal('n', 0)]);
    }

    #[test]
    fn special() {
        lex_valid!("/\\n/", vec![Special(LineBreak, 0)]);
    }

    #[test]
    fn escaped() {
        lex_valid!("/\\\\n/", vec![Literal('\\', 0), Literal('n', 2)]);
    }
}

mod carriagereturn {
    use super::super::*;
    use crate::lex_valid;

    #[test]
    fn literal() {
        lex_valid!("/r/", vec![Literal('r', 0)]);
    }

    #[test]
    fn special() {
        lex_valid!("/\\r/", vec![Special(CarriageReturn, 0)]);
    }

    #[test]
    fn escaped() {
        lex_valid!("/\\\\r/", vec![Literal('\\', 0), Literal('r', 2)]);
    }
}

mod tab {
    use super::super::*;
    use crate::lex_valid;

    #[test]
    fn literal() {
        lex_valid!("/t/", vec![Literal('t', 0)]);
    }

    #[test]
    fn special() {
        lex_valid!("/\\t/", vec![Special(Tab, 0)]);
    }

    #[test]
    fn escaped() {
        lex_valid!("/\\\\t/", vec![Literal('\\', 0), Literal('t', 2)]);
    }
}

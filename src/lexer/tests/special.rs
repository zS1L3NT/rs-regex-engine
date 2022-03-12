mod whitespace {
    use super::super::*;
    use crate::valid;

    #[test]
    fn literal() {
        valid!("/s/", vec![Literal('s', 0)]);
    }

    #[test]
    fn special() {
        valid!("/\\s/", vec![Special(Whitespace, 0)]);
    }

    #[test]
    fn escaped() {
        valid!("/\\\\s/", vec![Literal('\\', 0), Literal('s', 2)]);
    }
}

mod nonwhitespace {
    use super::super::*;
    use crate::valid;

    #[test]
    fn literal() {
        valid!("/S/", vec![Literal('S', 0)]);
    }

    #[test]
    fn special() {
        valid!("/\\S/", vec![Special(NonWhitespace, 0)]);
    }

    #[test]
    fn escaped() {
        valid!("/\\\\S/", vec![Literal('\\', 0), Literal('S', 2)]);
    }
}

mod digit {
    use super::super::*;
    use crate::valid;

    #[test]
    fn literal() {
        valid!("/d/", vec![Literal('d', 0)]);
    }

    #[test]
    fn special() {
        valid!("/\\d/", vec![Special(Digit, 0)]);
    }

    #[test]
    fn escaped() {
        valid!("/\\\\d/", vec![Literal('\\', 0), Literal('d', 2)]);
    }
}

mod nondigit {
    use super::super::*;
    use crate::valid;

    #[test]
    fn literal() {
        valid!("/D/", vec![Literal('D', 0)]);
    }

    #[test]
    fn special() {
        valid!("/\\D/", vec![Special(NonDigit, 0)]);
    }

    #[test]
    fn escaped() {
        valid!("/\\\\D/", vec![Literal('\\', 0), Literal('D', 2)]);
    }
}

mod word {
    use super::super::*;
    use crate::valid;

    #[test]
    fn literal() {
        valid!("/w/", vec![Literal('w', 0)]);
    }

    #[test]
    fn special() {
        valid!("/\\w/", vec![Special(Word, 0)]);
    }

    #[test]
    fn escaped() {
        valid!("/\\\\w/", vec![Literal('\\', 0), Literal('w', 2)]);
    }
}

mod nonword {
    use super::super::*;
    use crate::valid;

    #[test]
    fn literal() {
        valid!("/W/", vec![Literal('W', 0)]);
    }

    #[test]
    fn special() {
        valid!("/\\W/", vec![Special(NonWord, 0)]);
    }

    #[test]
    fn escaped() {
        valid!("/\\\\W/", vec![Literal('\\', 0), Literal('W', 2)]);
    }
}

mod boundary {
    use super::super::*;
    use crate::valid;

    #[test]
    fn literal() {
        valid!("/b/", vec![Literal('b', 0)]);
    }

    #[test]
    fn special() {
        valid!("/\\b/", vec![Special(Boundary, 0)]);
    }

    #[test]
    fn escaped() {
        valid!("/\\\\b/", vec![Literal('\\', 0), Literal('b', 2)]);
    }
}

mod nonboundary {
    use super::super::*;
    use crate::valid;

    #[test]
    fn literal() {
        valid!("/B/", vec![Literal('B', 0)]);
    }

    #[test]
    fn special() {
        valid!("/\\B/", vec![Special(NonBoundary, 0)]);
    }

    #[test]
    fn escaped() {
        valid!("/\\\\B/", vec![Literal('\\', 0), Literal('B', 2)]);
    }
}

mod linebreak {
    use super::super::*;
    use crate::valid;

    #[test]
    fn literal() {
        valid!("/n/", vec![Literal('n', 0)]);
    }

    #[test]
    fn special() {
        valid!("/\\n/", vec![Special(LineBreak, 0)]);
    }

    #[test]
    fn escaped() {
        valid!("/\\\\n/", vec![Literal('\\', 0), Literal('n', 2)]);
    }
}

mod carriagereturn {
    use super::super::*;
    use crate::valid;

    #[test]
    fn literal() {
        valid!("/r/", vec![Literal('r', 0)]);
    }

    #[test]
    fn special() {
        valid!("/\\r/", vec![Special(CarriageReturn, 0)]);
    }

    #[test]
    fn escaped() {
        valid!("/\\\\r/", vec![Literal('\\', 0), Literal('r', 2)]);
    }
}

mod tab {
    use super::super::*;
    use crate::valid;

    #[test]
    fn literal() {
        valid!("/t/", vec![Literal('t', 0)]);
    }

    #[test]
    fn special() {
        valid!("/\\t/", vec![Special(Tab, 0)]);
    }

    #[test]
    fn escaped() {
        valid!("/\\\\t/", vec![Literal('\\', 0), Literal('t', 2)]);
    }
}

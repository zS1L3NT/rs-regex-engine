mod capturing {
    use super::super::*;
    use crate::parse_valid;

    #[test]
    fn empty() {
        parse_valid!("/()/", Group(Box::new(Capturing(Empty, One, 1))));
    }

    #[test]
    fn with_literal() {
        parse_valid!(
            "/(a)/",
            Group(Box::new(Capturing(Single(Char('a', One, 2)), One, 1)))
        );
    }

    #[test]
    fn with_literal_quantified() {
        parse_valid!(
            "/(a+)?/",
            Group(Box::new(Capturing(
                Single(Char('a', OneOrMore(3), 2)),
                ZeroOrOne(5),
                1
            )))
        );
    }

    #[test]
    fn with_brackets() {
        parse_valid!(
            "/(a[bc])/",
            Group(Box::new(Capturing(
                Multiple(AND(vec![
                    Single(Char('a', One, 2)),
                    Multiple(OR(
                        vec![Single(Char('b', One, 4)), Single(Char('c', One, 5))],
                        One,
                        3
                    ))
                ])),
                One,
                1
            )))
        );
    }
}

mod non_capturing {
    use super::super::*;
    use crate::parse_valid;

    #[test]
    fn empty() {
        parse_valid!("/(?:)/", Group(Box::new(NonCapturing(Empty, One, 1))));
    }

    #[test]
    fn with_literal() {
        parse_valid!(
            "/(?:a)/",
            Group(Box::new(NonCapturing(Single(Char('a', One, 4)), One, 1)))
        );
    }

    #[test]
    fn with_literal_quantified() {
        parse_valid!(
            "/(?:a+)?/",
            Group(Box::new(NonCapturing(
                Single(Char('a', OneOrMore(5), 4)),
                ZeroOrOne(7),
                1
            )))
        );
    }

    #[test]
    fn with_brackets() {
        parse_valid!(
            "/(?:a[bc])/",
            Group(Box::new(NonCapturing(
                Multiple(AND(vec![
                    Single(Char('a', One, 4)),
                    Multiple(OR(
                        vec![Single(Char('b', One, 6)), Single(Char('c', One, 7))],
                        One,
                        5
                    ))
                ])),
                One,
                1
            )))
        );
    }
}

mod positive_lookahead {
    use super::super::*;
    use crate::parse_valid;

    #[test]
    fn empty() {
        parse_valid!("/(?=)/", Group(Box::new(PositiveLookAhead(Empty, One, 1))));
    }

    #[test]
    fn with_literal() {
        parse_valid!(
            "/(?=a)/",
            Group(Box::new(PositiveLookAhead(
                Single(Char('a', One, 4)),
                One,
                1
            )))
        );
    }

    #[test]
    fn with_literal_quantified() {
        parse_valid!(
            "/(?=a+)?/",
            Group(Box::new(PositiveLookAhead(
                Single(Char('a', OneOrMore(5), 4)),
                ZeroOrOne(7),
                1
            )))
        );
    }

    #[test]
    fn with_brackets() {
        parse_valid!(
            "/(?=a[bc])/",
            Group(Box::new(PositiveLookAhead(
                Multiple(AND(vec![
                    Single(Char('a', One, 4)),
                    Multiple(OR(
                        vec![Single(Char('b', One, 6)), Single(Char('c', One, 7))],
                        One,
                        5
                    ))
                ])),
                One,
                1
            )))
        );
    }
}

mod positive_lookbehind {
    use super::super::*;
    use crate::parse_valid;

    #[test]
    fn empty() {
        parse_valid!(
            "/(?<=)/",
            Group(Box::new(PositiveLookBehind(Empty, One, 1)))
        );
    }

    #[test]
    fn with_literal() {
        parse_valid!(
            "/(?<=a)/",
            Group(Box::new(PositiveLookBehind(
                Single(Char('a', One, 5)),
                One,
                1
            )))
        );
    }

    #[test]
    fn with_literal_quantified() {
        parse_valid!(
            "/(?<=a+)?/",
            Group(Box::new(PositiveLookBehind(
                Single(Char('a', OneOrMore(6), 5)),
                ZeroOrOne(8),
                1
            )))
        );
    }

    #[test]
    fn with_brackets() {
        parse_valid!(
            "/(?<=a[bc])/",
            Group(Box::new(PositiveLookBehind(
                Multiple(AND(vec![
                    Single(Char('a', One, 5)),
                    Multiple(OR(
                        vec![Single(Char('b', One, 7)), Single(Char('c', One, 8))],
                        One,
                        6
                    ))
                ])),
                One,
                1
            )))
        );
    }
}
mod negative_lookahead {
    use super::super::*;
    use crate::parse_valid;

    #[test]
    fn empty() {
        parse_valid!("/(?!)/", Group(Box::new(NegativeLookAhead(Empty, One, 1))));
    }

    #[test]
    fn with_literal() {
        parse_valid!(
            "/(?!a)/",
            Group(Box::new(NegativeLookAhead(
                Single(Char('a', One, 4)),
                One,
                1
            )))
        );
    }

    #[test]
    fn with_literal_quantified() {
        parse_valid!(
            "/(?!a+)?/",
            Group(Box::new(NegativeLookAhead(
                Single(Char('a', OneOrMore(5), 4)),
                ZeroOrOne(7),
                1
            )))
        );
    }

    #[test]
    fn with_brackets() {
        parse_valid!(
            "/(?!a[bc])/",
            Group(Box::new(NegativeLookAhead(
                Multiple(AND(vec![
                    Single(Char('a', One, 4)),
                    Multiple(OR(
                        vec![Single(Char('b', One, 6)), Single(Char('c', One, 7))],
                        One,
                        5
                    ))
                ])),
                One,
                1
            )))
        );
    }
}

mod negative_lookbehind {
    use super::super::*;
    use crate::parse_valid;

    #[test]
    fn empty() {
        parse_valid!(
            "/(?<!)/",
            Group(Box::new(NegativeLookBehind(Empty, One, 1)))
        );
    }

    #[test]
    fn with_literal() {
        parse_valid!(
            "/(?<!a)/",
            Group(Box::new(NegativeLookBehind(
                Single(Char('a', One, 5)),
                One,
                1
            )))
        );
    }

    #[test]
    fn with_literal_quantified() {
        parse_valid!(
            "/(?<!a+)?/",
            Group(Box::new(NegativeLookBehind(
                Single(Char('a', OneOrMore(6), 5)),
                ZeroOrOne(8),
                1
            )))
        );
    }

    #[test]
    fn with_brackets() {
        parse_valid!(
            "/(?<!a[bc])/",
            Group(Box::new(NegativeLookBehind(
                Multiple(AND(vec![
                    Single(Char('a', One, 5)),
                    Multiple(OR(
                        vec![Single(Char('b', One, 7)), Single(Char('c', One, 8))],
                        One,
                        6
                    ))
                ])),
                One,
                1
            )))
        );
    }
}

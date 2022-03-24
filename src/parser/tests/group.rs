mod capturing {
    use super::super::*;
    use crate::parse_valid;

    #[test]
    fn empty() {
        parse_valid!("/()/", Group(Box::new(Capturing(Empty, One))));
    }

    #[test]
    fn with_literal() {
        parse_valid!(
            "/(a)/",
            Group(Box::new(Capturing(Single(Char('a', One)), One)))
        );
    }

    #[test]
    fn with_literal_quantified() {
        parse_valid!(
            "/(a+)?/",
            Group(Box::new(Capturing(Single(Char('a', OneOrMore)), ZeroOrOne)))
        );
    }

    #[test]
    fn with_brackets() {
        parse_valid!(
            "/(a[bc])/",
            Group(Box::new(Capturing(
                Multiple(AND(vec![
                    Single(Char('a', One)),
                    Multiple(OR(
                        vec![Single(Char('b', One)), Single(Char('c', One))],
                        One
                    ))
                ])),
                One
            )))
        );
    }
}

mod non_capturing {
    use super::super::*;
    use crate::parse_valid;

    #[test]
    fn empty() {
        parse_valid!("/(^)/", Group(Box::new(NonCapturing(Empty, One))));
    }

    #[test]
    fn with_literal() {
        parse_valid!(
            "/(^a)/",
            Group(Box::new(NonCapturing(Single(Char('a', One)), One)))
        );
    }

    #[test]
    fn with_literal_quantified() {
        parse_valid!(
            "/(^a+)?/",
            Group(Box::new(NonCapturing(
                Single(Char('a', OneOrMore)),
                ZeroOrOne
            )))
        );
    }

    #[test]
    fn with_brackets() {
        parse_valid!(
            "/(^a[bc])/",
            Group(Box::new(NonCapturing(
                Multiple(AND(vec![
                    Single(Char('a', One)),
                    Multiple(OR(
                        vec![Single(Char('b', One)), Single(Char('c', One))],
                        One
                    ))
                ])),
                One
            )))
        );
    }
}

mod positive_lookahead {
    use super::super::*;
    use crate::parse_valid;

    #[test]
    fn empty() {
        parse_valid!("/(?=)/", Group(Box::new(PositiveLookAhead(Empty, One))));
    }

    #[test]
    fn with_literal() {
        parse_valid!(
            "/(?=a)/",
            Group(Box::new(PositiveLookAhead(Single(Char('a', One)), One)))
        );
    }

    #[test]
    fn with_literal_quantified() {
        parse_valid!(
            "/(?=a+)?/",
            Group(Box::new(PositiveLookAhead(
                Single(Char('a', OneOrMore)),
                ZeroOrOne
            )))
        );
    }

    #[test]
    fn with_brackets() {
        parse_valid!(
            "/(?=a[bc])/",
            Group(Box::new(PositiveLookAhead(
                Multiple(AND(vec![
                    Single(Char('a', One)),
                    Multiple(OR(
                        vec![Single(Char('b', One)), Single(Char('c', One))],
                        One
                    ))
                ])),
                One
            )))
        );
    }
}

mod positive_lookbehind {
    use super::super::*;
    use crate::parse_valid;

    #[test]
    fn empty() {
        parse_valid!("/(?<=)/", Group(Box::new(PositiveLookBehind(Empty, One))));
    }

    #[test]
    fn with_literal() {
        parse_valid!(
            "/(?<=a)/",
            Group(Box::new(PositiveLookBehind(Single(Char('a', One)), One)))
        );
    }

    #[test]
    fn with_literal_quantified() {
        parse_valid!(
            "/(?<=a+)?/",
            Group(Box::new(PositiveLookBehind(
                Single(Char('a', OneOrMore)),
                ZeroOrOne
            )))
        );
    }

    #[test]
    fn with_brackets() {
        parse_valid!(
            "/(?<=a[bc])/",
            Group(Box::new(PositiveLookBehind(
                Multiple(AND(vec![
                    Single(Char('a', One)),
                    Multiple(OR(
                        vec![Single(Char('b', One)), Single(Char('c', One))],
                        One
                    ))
                ])),
                One
            )))
        );
    }
}
mod negative_lookahead {
    use super::super::*;
    use crate::parse_valid;

    #[test]
    fn empty() {
        parse_valid!("/(?!)/", Group(Box::new(NegativeLookAhead(Empty, One))));
    }

    #[test]
    fn with_literal() {
        parse_valid!(
            "/(?!a)/",
            Group(Box::new(NegativeLookAhead(Single(Char('a', One)), One)))
        );
    }

    #[test]
    fn with_literal_quantified() {
        parse_valid!(
            "/(?!a+)?/",
            Group(Box::new(NegativeLookAhead(
                Single(Char('a', OneOrMore)),
                ZeroOrOne
            )))
        );
    }

    #[test]
    fn with_brackets() {
        parse_valid!(
            "/(?!a[bc])/",
            Group(Box::new(NegativeLookAhead(
                Multiple(AND(vec![
                    Single(Char('a', One)),
                    Multiple(OR(
                        vec![Single(Char('b', One)), Single(Char('c', One))],
                        One
                    ))
                ])),
                One
            )))
        );
    }
}

mod negative_lookbehind {
    use super::super::*;
    use crate::parse_valid;

    #[test]
    fn empty() {
        parse_valid!("/(?<!)/", Group(Box::new(NegativeLookBehind(Empty, One))));
    }

    #[test]
    fn with_literal() {
        parse_valid!(
            "/(?<!a)/",
            Group(Box::new(NegativeLookBehind(Single(Char('a', One)), One)))
        );
    }

    #[test]
    fn with_literal_quantified() {
        parse_valid!(
            "/(?<!a+)?/",
            Group(Box::new(NegativeLookBehind(
                Single(Char('a', OneOrMore)),
                ZeroOrOne
            )))
        );
    }

    #[test]
    fn with_brackets() {
        parse_valid!(
            "/(?<!a[bc])/",
            Group(Box::new(NegativeLookBehind(
                Multiple(AND(vec![
                    Single(Char('a', One)),
                    Multiple(OR(
                        vec![Single(Char('b', One)), Single(Char('c', One))],
                        One
                    ))
                ])),
                One
            )))
        );
    }
}

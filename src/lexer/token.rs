pub type Pos = usize;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Literal(char, Pos),
    Special(Special, Pos),
    Quantity(Quantity, Pos),
    AnchorStart(Pos),
    AnchorEnd(Pos),
    OpenGroup(OpenGroup, Pos),
    CloseGroup(Pos),
    OpenBracket(OpenBracket, Pos),
    CloseBracket(Pos),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Special {
    Whitespace,
    NonWhitespace,
    Word,
    NonWord,
    Digit,
    NonDigit,
    Boundary,
    NonBoundary,
    LineBreak,
    CarriageReturn,
    Tab,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Quantity {
    OneOrMore,
    ZeroOrMore,
    ZeroOrOne,
    Count(usize),
    Range(usize, Option<usize>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum OpenGroup {
    Capturing,
    NonCapturing,
    PositiveLookAhead,
    PositiveLookBehind,
    NegativeLookAhead,
    NegativeLookBehind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OpenBracket {
    NonNegated,
    Negated,
}

type Pos = usize;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Literal(char, Pos),
    Special(Special, Pos),
    Quantifier(Quantifier, Pos),
    AssertStart(Pos),
    AssertEnd(Pos),
    OpenGroup(OpenGroup, Pos),
    CloseGroup(Pos),
    OpenBracket(OpenBracket, Pos),
    CloseBracket(Pos),
    Pipe(Pos),
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
pub enum Quantifier {
    Plus,
    Asterisk,
    QuestionMark,
    Count(usize),
    Range(Option<usize>, Option<usize>),
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

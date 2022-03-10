type Pos = usize;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Literal(char, Pos),
    Special(Special),
    Quantifier(Quantifier),
    AssertStart(Pos),
    AssertEnd(Pos),
    OpenGroup(OpenGroup),
    CloseGroup(Pos),
    OpenBracket(OpenBracket),
    CloseBracket(Pos),
    Pipe(Pos),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Special {
    Whitespace(Pos),
    NonWhitespace(Pos),
    Word(Pos),
    NonWord(Pos),
    Digit(Pos),
    NonDigit(Pos),
    Boundary(Pos),
    NonBoundary(Pos),
    LineBreak(Pos),
    CarriageReturn(Pos),
    Tab(Pos),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Quantifier {
    Plus(Pos),
    Asterisk(Pos),
    QuestionMark(Pos),
    Count(usize, Pos),
    Range(Option<usize>, Option<usize>, Pos),
}

#[derive(Debug, Clone, PartialEq)]
pub enum OpenGroup {
    Capturing(Pos),
    NonCapturing(Pos),
    PositiveLookAhead(Pos),
    PositiveLookBehind(Pos),
    NegativeLookAhead(Pos),
    NegativeLookBehind(Pos),
}

#[derive(Debug, Clone, PartialEq)]
pub enum OpenBracket {
    NonNegated(Pos),
    Negated(Pos),
}

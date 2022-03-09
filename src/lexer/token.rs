type Pos = usize;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Literal(char, Pos),
    Special(char, Pos),
    Quantifier(Quantifier),
    AssertStart(Pos),
    AssertEnd(Pos),
    OpenGroup(OpenGroup),
    CloseGroup(Pos),
    OpenBracket(bool, Pos),
    CloseBracket(Pos),
    Pipe(Pos),
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
pub enum Quantifier {
    Plus(Pos),
    Asterisk(Pos),
    QuestionMark(Pos),
    Count(usize, Pos),
    Range(Option<usize>, Option<usize>, Pos),
}

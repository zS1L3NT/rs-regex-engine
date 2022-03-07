type Pos = usize;

#[derive(Debug, Clone)]
pub enum Token {
    Literal(char, Pos, Option<Quantifier>),
    Special(char, Pos, Option<Quantifier>),
    AssertStart(Pos),
    AssertEnd(Pos),
    DisableCapture(Pos),
    GroupName(String, Pos),
    OpenGroup(Pos),
    CloseGroup(Pos),
    OpenBracket(bool, Pos),
    CloseBracket(Pos),
    LookAhead(bool, Pos),
    LookBehind(bool, Pos),
}

#[derive(Debug, Clone)]
pub enum Quantifier {
    Plus(Pos),
    QuestionMark(Pos),
    Count(usize, Pos),
    Range(usize, usize, Pos),
}

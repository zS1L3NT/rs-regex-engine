use std::fmt::Debug;

pub type Pos = usize;

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Empty,
    Single(Single),
    Multiple(Multiple),
    Group(Box<Group>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Single {
    Char(char, Quantity, Pos),
    Whitespace(Quantity, Pos),
    NonWhitespace(Quantity, Pos),
    Word(Quantity, Pos),
    NonWord(Quantity, Pos),
    Digit(Quantity, Pos),
    NonDigit(Quantity, Pos),
    Boundary(Quantity, Pos),
    NonBoundary(Quantity, Pos),
    LineBreak(Quantity, Pos),
    CarriageReturn(Quantity, Pos),
    Tab(Quantity, Pos),
    AnchorStart(Quantity, Pos),
    AnchorEnd(Quantity, Pos),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Multiple {
    AND(Vec<Node>),
    OR(Vec<Node>, Quantity, Pos),
    NOR(Vec<Node>, Quantity, Pos),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Group {
    Capturing(Node, Quantity, Pos),
    NonCapturing(Node, Quantity, Pos),
    PositiveLookAhead(Node, Quantity, Pos),
    PositiveLookBehind(Node, Quantity, Pos),
    NegativeLookAhead(Node, Quantity, Pos),
    NegativeLookBehind(Node, Quantity, Pos),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Quantity {
    One,
    OneOrMore(Pos),
    ZeroOrMore(Pos),
    ZeroOrOne(Pos),
    Count(usize, Pos),
    Range(usize, Option<usize>, Pos),
}

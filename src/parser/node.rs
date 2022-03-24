use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Empty,
    Single(Single),
    Multiple(Multiple),
    Group(Box<Group>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Single {
    Char(char, Quantity),
    Whitespace(Quantity),
    NonWhitespace(Quantity),
    Word(Quantity),
    NonWord(Quantity),
    Digit(Quantity),
    NonDigit(Quantity),
    Boundary(Quantity),
    NonBoundary(Quantity),
    LineBreak(Quantity),
    CarriageReturn(Quantity),
    Tab(Quantity),
    AnchorStart(Quantity),
    AnchorEnd(Quantity),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Multiple {
    AND(Vec<Node>),
    OR(Vec<Node>, Quantity),
    NOR(Vec<Node>, Quantity),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Group {
    Capturing(Node, Quantity),
    NonCapturing(Node, Quantity),
    PositiveLookAhead(Node, Quantity),
    PositiveLookBehind(Node, Quantity),
    NegativeLookAhead(Node, Quantity),
    NegativeLookBehind(Node, Quantity),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Quantity {
    One,
    OneOrMore,
    ZeroOrMore,
    ZeroOrOne,
    Count(usize),
    Range(usize, Option<usize>),
}

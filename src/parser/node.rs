use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Single(Single),
    Multiple(Multiple),
    Group(Box<Group>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Single {
    Char(char, Option<Quantity>),
    Whitespace(Option<Quantity>),
    NonWhitespace(Option<Quantity>),
    Word(Option<Quantity>),
    NonWord(Option<Quantity>),
    Digit(Option<Quantity>),
    NonDigit(Option<Quantity>),
    Boundary(Option<Quantity>),
    NonBoundary(Option<Quantity>),
    LineBreak(Option<Quantity>),
    CarriageReturn(Option<Quantity>),
    Tab(Option<Quantity>),
    AnchorStart(Option<Quantity>),
    AnchorEnd(Option<Quantity>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Multiple {
    AND(Vec<Node>, Option<Quantity>),
    OR(Vec<Node>, Option<Quantity>),
    NOR(Vec<Node>, Option<Quantity>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Group {
    Capturing(Node, Option<Quantity>),
    NonCapturing(Node, Option<Quantity>),
    PositiveLookAhead(Node, Option<Quantity>),
    PositiveLookBehind(Node, Option<Quantity>),
    NegativeLookAhead(Node, Option<Quantity>),
    NegativeLookBehind(Node, Option<Quantity>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Quantity {
    OneOrMore,
    ZeroOrMore,
    ZeroOrOne,
    Count(usize),
    Range(usize, Option<usize>),
}

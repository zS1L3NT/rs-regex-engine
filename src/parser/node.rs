use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Single(Single),
    Multi(Multi),
    Group(Group),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Single {
    value: SingleValue,
    quantity: Quantity,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SingleValue {
    Char(char),
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
    AnchorStart,
    AnchorEnd,
}

impl Single {
    pub fn new(value: SingleValue, quantity: Quantity) -> Self {
        Self { value, quantity }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Multi {
    variant: MultiVariant,
    values: Vec<Node>,
    quantity: Quantity,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MultiVariant {
    AND,
    OR,
    NAND,
}

impl Multi {
    pub fn new(variant: MultiVariant, values: Vec<Node>, quantity: Quantity) -> Self {
        Self {
            variant,
            values,
            quantity,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Group {
    variant: GroupVariant,
    value: Box<Node>,
    quantity: Quantity,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GroupVariant {
    Capturing,
    NonCapturing,
    PositiveLookAhead,
    PositiveLookBehind,
    NegativeLookAhead,
    NegativeLookBehind,
}

impl Group {
    pub fn new(variant: GroupVariant, value: Box<Node>, quantity: Quantity) -> Self {
        Self {
            variant,
            value,
            quantity,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Quantity {
    OneOrMore,
    ZeroOrMore,
    ZeroOrOne,
    Count(usize),
    Range(usize, Option<usize>),
}

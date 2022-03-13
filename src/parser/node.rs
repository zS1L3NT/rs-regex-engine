use std::fmt::Debug;

trait Node: NodeClone + Debug {}

trait NodeClone {
    fn clone_box(&self) -> Box<dyn Node>;
}

impl<T: 'static + Node + Clone> NodeClone for T {
    fn clone_box(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Node> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Debug, Clone)]
pub enum Quantity {
    OneOrMore,
    ZeroOrMore,
    ZeroOrOne,
    Count(usize),
    Range(usize, Option<usize>),
}

#[derive(Debug, Clone)]
pub struct SingleNode {
    value: SingleValue,
    quantity: Quantity,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct MultiNode {
    variant: MultiVariant,
    values: Vec<Box<dyn Node>>,
    quantity: Quantity,
}

#[derive(Debug, Clone)]
pub enum MultiVariant {
    AND,
    OR,
    NAND,
}

#[derive(Debug, Clone)]
pub struct GroupNode {
    variant: GroupVariant,
    quantity: Quantity,
}

#[derive(Debug, Clone)]
pub enum GroupVariant {
    Capturing,
    NonCapturing,
    PositiveLookAhead,
    PositiveLookBehind,
    NegativeLookAhead,
    NegativeLookBehind,
}

impl Node for SingleNode {}
impl Node for MultiNode {}
impl Node for GroupNode {}

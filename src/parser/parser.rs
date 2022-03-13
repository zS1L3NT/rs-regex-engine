use super::{super::lexer::Token, *};

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }

    fn parse() -> Option<Node> {
        None
    }
}

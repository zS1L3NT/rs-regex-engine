use super::{
    evaluator::Evaluator,
    lexer::Lexer,
    parser::{Node, Parser},
    Error,
};

pub struct RegExp {
    pub node: Node,
}

impl RegExp {
    fn new(node: Node) -> Self {
        Self { node }
    }

    pub fn matches_string(&self, string: &str) -> Option<Vec<String>> {
        Evaluator::new(self.node.clone(), string).evaluate()
    }
}

pub trait ToRegExp {
    fn to_regexp(&self) -> Result<RegExp, Error>;
}

impl ToRegExp for String {
    fn to_regexp(&self) -> Result<RegExp, Error> {
        let tokens = Lexer::new(self.clone()).lex()?;
        let node = Parser::new(tokens).parse()?;
        Ok(RegExp::new(node))
    }
}

pub trait MatchesRegExp {
    fn matches_regexp(&self, regexp: RegExp) -> bool {
        true
    }
}

impl MatchesRegExp for String {
    fn matches_regexp(&self, regexp: RegExp) -> bool {
        true
    }
}

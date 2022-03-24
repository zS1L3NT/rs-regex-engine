mod anchor;
mod bracket;
mod group;
mod quantifier;

pub use super::{
    super::lexer::Lexer,
    super::Opsult,
    node::{
        Group::{self, *},
        Multiple::{self, *},
        Node::{self, *},
        Quantity::{self, *},
        Single::{self, *},
    },
    Parser,
};

#[macro_export]
macro_rules! parse_valid {
    ($a:literal, $node:expr) => {{
        let tokens = Lexer::new($a.to_string()).lex().unwrap();
        let node = Parser::new(tokens).parse();
        let value = if let Ok(value) = node {
            value
        } else {
            println!("Value was not Some, {:?}", node);
            Single(Char('_', One, 0))
        };
        assert_eq!(value, $node);
    }};
}

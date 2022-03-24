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
        let value = if let Opsult::Some(value) = node {
            value
        } else {
            println!("Value was not Some, {:?}", node);
            Single(Char('_', ZeroOrOne))
        };
        assert_eq!(value, $node);
    }};
}

#[macro_export]
macro_rules! parse_invalid {
    ($a:literal, $msg:literal) => {{
        let tokens = Lexer::new($a.to_string()).lex().unwrap();
        let msg_starts_with = if let Opsult::Err(err) = Parser::new(tokens).parse() {
            err.msg.starts_with($msg)
        } else {
            println!("\"{}\" was not Err", $a);
            false
        };
        assert!(msg_starts_with);
    }};
}

mod anchors;
mod brackets;
mod escape;
mod group;
mod quantifiers;
mod regex;
mod special;

pub use super::{
    token::{
        OpenBracket::{self, *},
        OpenGroup::{self, *},
        Quantity::{self, *},
        Special::{self, *},
        Token::{self, *},
    },
    Lexer,
};

#[macro_export]
macro_rules! lex_valid {
    ($a:literal, $count:literal) => {
        let count = if let Ok(tokens) = Lexer::new($a.to_string()).lex() {
            tokens.len()
        } else {
            println!("\"{}\" was not Ok", $a);
            0
        };
        assert_eq!(count, $count);
    };
    ($a:literal, $tokens:expr) => {
        let tokens = if let Ok(tokens) = Lexer::new($a.to_string()).lex() {
            tokens
        } else {
            println!("\"{}\" was not Ok", $a);
            vec![]
        };
        assert_eq!(tokens, $tokens);
    };
}

#[macro_export]
macro_rules! lex_invalid {
    ($a:literal, $msg:literal) => {
        let msg_starts_with = if let Err(err) = Lexer::new($a.to_string()).lex() {
            err.msg.starts_with($msg)
        } else {
            println!("\"{}\" was not Err", $a);
            false
        };
        assert!(msg_starts_with);
    };
}

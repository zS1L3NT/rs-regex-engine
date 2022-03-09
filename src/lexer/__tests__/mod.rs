#[cfg(test)]
mod anchors;

#[cfg(test)]
mod brackets;

#[cfg(test)]
mod escape;

#[cfg(test)]
mod group;

#[cfg(test)]
mod quantifiers;

#[cfg(test)]
mod regex;

#[cfg(test)]
mod special;

pub use super::{
    token::{OpenBracket, OpenGroup, Quantifier, Special, Token},
    Lexer,
};
pub use OpenBracket::*;
pub use OpenGroup::*;
pub use Quantifier::*;
pub use Special::*;
pub use Token::*;

#[macro_export]
macro_rules! valid {
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
macro_rules! invalid {
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

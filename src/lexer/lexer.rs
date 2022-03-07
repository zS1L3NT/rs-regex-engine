use crate::error::Error;

use super::Token;

pub struct Lexer {
    string: String,
}

impl Lexer {
    pub fn new(string: String) -> Self {
        Self { string }
    }

    pub fn lex(&self) -> Result<Vec<Token>, Error> {
        let mut string = self.string.clone();
        let mut tokens = vec![];

        

        Ok(tokens)
    }
}

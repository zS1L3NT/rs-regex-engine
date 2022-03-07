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

        if string.starts_with("/") {
            string.remove(0);
        } else {
            return Err(Error::new(
                "Expected RegExp to start with a </>".to_string(),
                0,
            ));
        }

        if string.ends_with("/") {
            string.remove(string.len() - 1);
        } else {
            return Err(Error::new(
                "Expected RegExp to end with a </>".to_string(),
                string.len() - 1,
            ));
        }

        Ok(tokens)
    }
}

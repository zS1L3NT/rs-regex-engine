use crate::{
    lexer::{Pos, Token},
    Error,
    Opsult::{self, Err as _Err, Some as _Some},
};

pub struct Lexer {
    chars: Vec<char>,
    tokens: Vec<Token>,
    pos: Pos,
}

impl Lexer {
    pub fn new(string: String) -> Self {
        Self {
            chars: string.chars().collect(),
            tokens: vec![],
            pos: 0,
        }
    }

    pub fn lex(&mut self) -> Opsult<Vec<Token>, Error> {
        if let Some(first_char) = self.chars.get(0) {
            if *first_char != '/' {
                return _Err(Error::new(
                    "Expected RegExp to start with a </>".to_string(),
                    0,
                ));
            }
        }

        if let Some(last_char) = self.chars.last() {
            if *last_char != '/' {
                return _Err(Error::new(
                    "Expected RegExp to end with a </>".to_string(),
                    self.chars.len() - 1,
                ));
            }
        }

        self.chars.remove(0);
        self.chars.remove(self.chars.len() - 1);

        while self.chars.len() > 0 {
            if self.lex_anchors() {
                continue;
            }

            if self.lex_brackets() {
                continue;
            }

            if let Some(char) = self.chars.first() {
                self.tokens.push(Token::Literal(*char, self.pos));
                self.chars.remove(0);
                self.pos += 1;
                continue;
            } else {
                break;
            }
        }

        _Some(vec![])
    }

    fn lex_anchors(&mut self) -> bool {
        match self.chars.first() {
            Some('^') => {
                self.tokens.push(Token::AnchorStart(self.pos));
                self.chars.remove(0);
                self.pos += 1;
                true
            }
            Some('$') => {
                let token = Token::AnchorEnd(self.pos);
                self.chars.remove(0);
                self.pos += 1;
                true
            }
            _ => false,
        }
    }

    fn lex_brackets(&self) -> bool {
        false
    }
}

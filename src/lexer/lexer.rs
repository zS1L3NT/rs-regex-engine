use super::{super::Error, OpenBracket, Pos, Token};

pub struct Lexer {
    chars: Vec<char>,
    tokens: Vec<Token>,
    pos: Pos,

    bracket_depth: i32,
}

impl Lexer {
    pub fn new(string: String) -> Self {
        Self {
            chars: string.chars().collect(),
            tokens: vec![],
            pos: 0,
            bracket_depth: 0,
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, Error> {
        if let Some(first_char) = self.chars.get(0) {
            if *first_char != '/' {
                return Err(Error::new(
                    "Expected RegExp to start with a </>".to_string(),
                    0,
                ));
            }
        }

        if let Some(last_char) = self.chars.last() {
            if *last_char != '/' {
                return Err(Error::new(
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

            match self.lex_brackets() {
                Ok(result) => {
                    if result {
                        continue;
                    }
                }
                Err(err) => return Err(err),
            }

            if self.lex_groups() {
                continue;
            }

            if self.lex_quantifier() {
                continue;
            }

            if self.lex_special() {
                continue;
            }

            match self.lex_escaped() {
                Ok(result) => {
                    if result {
                        continue;
                    }
                }
                Err(err) => return Err(err),
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

        Ok(self.tokens.clone())
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
                self.tokens.push(Token::AnchorEnd(self.pos));
                self.chars.remove(0);
                self.pos += 1;
                true
            }
            _ => false,
        }
    }

    fn lex_brackets(&mut self) -> Result<bool, Error> {
        match self.chars.first() {
            Some('[') => {
                self.bracket_depth += 1;
                if let Some('^') = self.chars.get(1) {
                    self.tokens
                        .push(Token::OpenBracket(OpenBracket::Negated, self.pos));
                    self.chars.drain(0..2);
                    self.pos += 2;
                } else {
                    self.tokens
                        .push(Token::OpenBracket(OpenBracket::NonNegated, self.pos));
                    self.chars.remove(0);
                    self.pos += 1;
                }
                if self.chars.len() == 0 {
                    Err(Error::new(
                        "Character class missing closing bracket".to_string(),
                        self.pos,
                    ))
                } else {
                    Ok(true)
                }
            }
            Some(']') => {
                if self.bracket_depth == 0 {
                    self.tokens.push(Token::Literal(']', self.pos));
                } else {
                    self.bracket_depth -= 1;
                    self.tokens.push(Token::CloseBracket(self.pos));
                }
                self.chars.remove(0);
                self.pos += 1;
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    fn lex_groups(&mut self) -> bool {
        false
    }

    fn lex_quantifier(&mut self) -> bool {
        false
    }

    fn lex_special(&mut self) -> bool {
        false
    }

    fn lex_escaped(&mut self) -> Result<bool, Error> {
        if let Some('\\') = self.chars.first() {
            match self.chars.get(1) {
                Some(char) => {
                    self.tokens.push(Token::Literal(*char, self.pos));
                    self.chars.drain(0..2);
                    self.pos += 2;
                    Ok(true)
                }
                None => Err(Error::new(
                    "Pattern may not end with a trailing backslash".to_string(),
                    self.pos,
                )),
            }
        } else {
            Ok(false)
        }
    }
}

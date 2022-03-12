use super::{super::Error, OpenBracket, OpenGroup, Pos, Quantifier, Special, Token};

pub struct Lexer {
    chars: Vec<char>,
    tokens: Vec<Token>,
    pos: Pos,

    bracket_depth: i32,
    group_depth: i32,
}

impl Lexer {
    pub fn new(string: String) -> Self {
        Self {
            chars: string.chars().collect(),
            tokens: vec![],
            pos: 0,

            bracket_depth: 0,
            group_depth: 0,
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

            match self.lex_groups() {
                Ok(result) => {
                    if result {
                        continue;
                    }
                }
                Err(err) => return Err(err),
            }

            if self.lex_quantifier() {
                continue;
            }

            match self.lex_special() {
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

    fn lex_groups(&mut self) -> Result<bool, Error> {
        match self.chars.first() {
            Some('(') => {
                self.group_depth += 1;
                let result = match self.chars.get(1) {
                    Some('?') => match self.chars.get(2) {
                        Some(':') => {
                            self.tokens
                                .push(Token::OpenGroup(OpenGroup::NonCapturing, self.pos));
                            self.chars.drain(0..3);
                            self.pos += 3;
                            Ok(true)
                        }
                        Some('=') => {
                            self.tokens
                                .push(Token::OpenGroup(OpenGroup::PositiveLookAhead, self.pos));
                            self.chars.drain(0..3);
                            self.pos += 3;
                            Ok(true)
                        }
                        Some('!') => {
                            self.tokens
                                .push(Token::OpenGroup(OpenGroup::NegativeLookAhead, self.pos));
                            self.chars.drain(0..3);
                            self.pos += 3;
                            Ok(true)
                        }
                        Some('<') => match self.chars.get(3) {
                            Some('=') => {
                                self.tokens.push(Token::OpenGroup(
                                    OpenGroup::PositiveLookBehind,
                                    self.pos,
                                ));
                                self.chars.drain(0..4);
                                self.pos += 4;
                                Ok(true)
                            }
                            Some('!') => {
                                self.tokens.push(Token::OpenGroup(
                                    OpenGroup::NegativeLookBehind,
                                    self.pos,
                                ));
                                self.chars.drain(0..4);
                                self.pos += 4;
                                Ok(true)
                            }
                            Some(_) => Err(Error::new(
                                "The preceding token is not quantifiable".to_string(),
                                self.pos,
                            )),
                            None => Err(Error::new(
                                "Incomplete group structure".to_string(),
                                self.pos,
                            )),
                        },
                        Some(_) => Err(Error::new(
                            "The preceding token is not quantifiable".to_string(),
                            self.pos,
                        )),
                        None => Err(Error::new(
                            "Incomplete group structure".to_string(),
                            self.pos,
                        )),
                    },
                    Some(_) => {
                        self.tokens
                            .push(Token::OpenGroup(OpenGroup::Capturing, self.pos));
                        self.chars.remove(0);
                        self.pos += 1;
                        Ok(true)
                    }
                    None => Err(Error::new(
                        "Incomplete group structure".to_string(),
                        self.pos,
                    )),
                };

                if let Ok(_) = result {
                    if self.chars.len() == 0 {
                        Err(Error::new(
                            "Incomplete group structure".to_string(),
                            self.pos,
                        ))
                    } else {
                        result
                    }
                } else {
                    result
                }
            }
            Some(')') => {
                if self.group_depth == 0 {
                    return Err(Error::new("Unmatched parenthesis".to_string(), self.pos));
                }
                self.group_depth -= 1;
                self.tokens.push(Token::CloseGroup(self.pos));
                self.chars.remove(0);
                self.pos += 1;
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    fn lex_quantifier(&mut self) -> bool {
        match self.chars.first() {
            Some('+') => {
                self.tokens
                    .push(Token::Quantifier(Quantifier::OneOrMore, self.pos));
                self.chars.remove(0);
                self.pos += 1;
                true
            }
            Some('*') => {
                self.tokens
                    .push(Token::Quantifier(Quantifier::ZeroOrMore, self.pos));
                self.chars.remove(0);
                self.pos += 1;
                true
            }
            Some('?') => {
                self.tokens
                    .push(Token::Quantifier(Quantifier::ZeroOrOne, self.pos));
                self.chars.remove(0);
                self.pos += 1;
                true
            }
            Some('{') => {
                let mut data = String::new();
                for char in &self.chars[1..] {
                    if char == &'}' {
                        if data.len() == 0 {
                            return false;
                        } else if data.contains(',') {
                            let mut ranges = data.split(',').into_iter();

                            let start = if let Ok(start) = ranges.next().unwrap().parse::<usize>() {
                                start
                            } else {
                                return false;
                            };

                            let end = if let Some(end) = ranges.next() {
                                if let Ok(end) = end.parse::<usize>() {
                                    Some(end)
                                } else {
                                    None
                                }
                            } else {
                                None
                            };

                            self.tokens
                                .push(Token::Quantifier(Quantifier::Range(start, end), self.pos))
                        } else {
                            self.tokens.push(Token::Quantifier(
                                Quantifier::Count(data.parse().unwrap()),
                                self.pos,
                            ))
                        }

                        let forward = data.len() + 2;
                        self.chars.drain(0..forward);
                        self.pos += forward;
                        return true;
                    }

                    if char.is_numeric() || (char == &',' && !data.contains(',')) {
                        data.push(*char);
                        continue;
                    }

                    return false;
                }

                false
            }
            _ => false,
        }
    }

    fn lex_special(&mut self) -> Result<bool, Error> {
        if let Some('\\') = self.chars.first() {
            match self.chars.get(1) {
                Some('s') => {
                    self.tokens
                        .push(Token::Special(Special::Whitespace, self.pos));
                    self.chars.drain(0..2);
                    self.pos += 2;
                    Ok(true)
                }
                Some('S') => {
                    self.tokens
                        .push(Token::Special(Special::NonWhitespace, self.pos));
                    self.chars.drain(0..2);
                    self.pos += 2;
                    Ok(true)
                }
                Some('d') => {
                    self.tokens.push(Token::Special(Special::Digit, self.pos));
                    self.chars.drain(0..2);
                    self.pos += 2;
                    Ok(true)
                }
                Some('D') => {
                    self.tokens
                        .push(Token::Special(Special::NonDigit, self.pos));
                    self.chars.drain(0..2);
                    self.pos += 2;
                    Ok(true)
                }
                Some('w') => {
                    self.tokens.push(Token::Special(Special::Word, self.pos));
                    self.chars.drain(0..2);
                    self.pos += 2;
                    Ok(true)
                }
                Some('W') => {
                    self.tokens.push(Token::Special(Special::NonWord, self.pos));
                    self.chars.drain(0..2);
                    self.pos += 2;
                    Ok(true)
                }
                Some('b') => {
                    self.tokens
                        .push(Token::Special(Special::Boundary, self.pos));
                    self.chars.drain(0..2);
                    self.pos += 2;
                    Ok(true)
                }
                Some('B') => {
                    self.tokens
                        .push(Token::Special(Special::NonBoundary, self.pos));
                    self.chars.drain(0..2);
                    self.pos += 2;
                    Ok(true)
                }
                Some('n') => {
                    self.tokens
                        .push(Token::Special(Special::LineBreak, self.pos));
                    self.chars.drain(0..2);
                    self.pos += 2;
                    Ok(true)
                }
                Some('t') => {
                    self.tokens.push(Token::Special(Special::Tab, self.pos));
                    self.chars.drain(0..2);
                    self.pos += 2;
                    Ok(true)
                }
                Some('r') => {
                    self.tokens
                        .push(Token::Special(Special::CarriageReturn, self.pos));
                    self.chars.drain(0..2);
                    self.pos += 2;
                    Ok(true)
                }
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

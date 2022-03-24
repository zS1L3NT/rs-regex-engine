use super::{
    super::{lexer::Token, Error, Opsult},
    *,
};

pub struct Parser {
    tokens: Vec<Token>,
}

fn convert_quantity(quantity: &crate::lexer::Quantity, pos: Pos) -> Quantity {
    match quantity {
        crate::lexer::Quantity::ZeroOrOne => Quantity::ZeroOrOne(pos),
        crate::lexer::Quantity::ZeroOrMore => Quantity::ZeroOrMore(pos),
        crate::lexer::Quantity::OneOrMore => Quantity::OneOrMore(pos),
        crate::lexer::Quantity::Count(count) => Quantity::Count(*count, pos),
        crate::lexer::Quantity::Range(start, end) => Quantity::Range(*start, *end, pos),
    }
}

fn convert_special(special: crate::lexer::Special) -> Special {
    match special {
        crate::lexer::Special::Whitespace => Special::Whitespace,
        crate::lexer::Special::NonWhitespace => Special::NonWhitespace,
        crate::lexer::Special::Word => Special::Word,
        crate::lexer::Special::NonWord => Special::NonWord,
        crate::lexer::Special::Digit => Special::Digit,
        crate::lexer::Special::NonDigit => Special::NonDigit,
        crate::lexer::Special::Boundary => Special::Boundary,
        crate::lexer::Special::NonBoundary => Special::NonBoundary,
        crate::lexer::Special::LineBreak => Special::LineBreak,
        crate::lexer::Special::CarriageReturn => Special::CarriageReturn,
        crate::lexer::Special::Tab => Special::Tab,
    }
}

fn convert_group(
    group: crate::lexer::OpenGroup,
    node: Node,
    quantity: Quantity,
    pos: Pos,
) -> Group {
    match group {
        crate::lexer::OpenGroup::Capturing => Group::Capturing(node, quantity, pos),
        crate::lexer::OpenGroup::NonCapturing => Group::NonCapturing(node, quantity, pos),
        crate::lexer::OpenGroup::PositiveLookAhead => Group::PositiveLookAhead(node, quantity, pos),
        crate::lexer::OpenGroup::PositiveLookBehind => {
            Group::PositiveLookBehind(node, quantity, pos)
        }
        crate::lexer::OpenGroup::NegativeLookAhead => Group::NegativeLookAhead(node, quantity, pos),
        crate::lexer::OpenGroup::NegativeLookBehind => {
            Group::NegativeLookBehind(node, quantity, pos)
        }
    }
}

fn convert_bracket(
    bracket: crate::lexer::OpenBracket,
    nodes: Vec<Node>,
    quantity: Quantity,
    pos: Pos,
) -> Multiple {
    match bracket {
        crate::lexer::OpenBracket::Negated => Multiple::NOR(nodes, quantity, pos),
        crate::lexer::OpenBracket::NonNegated => Multiple::OR(nodes, quantity, pos),
    }
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }

    pub fn parse(&mut self) -> Opsult<Node, Error> {
        if self.tokens.len() == 0 {
            return Opsult::None;
        }

        let mut nodes = Vec::new();

        while self.tokens.len() > 0 {
            match self.parse_node() {
                Opsult::Some(node) => nodes.push(node),
                Opsult::Err(err) => return Opsult::Err(err),
                Opsult::None => break,
            }
        }

        match nodes.len() {
            0 => Opsult::None,
            1 => Opsult::Some(nodes.first().unwrap().to_owned()),
            _ => Opsult::Some(Node::Multiple(Multiple::AND(nodes))),
        }
    }

    fn parse_node(&mut self) -> Opsult<Node, Error> {
        if let Some(token) = self.tokens.first() {
            Opsult::Some(match token.to_owned() {
                Token::Literal(char, pos) => {
                    self.tokens.remove(0);
                    if let Some(Token::Quantity(quantity, _pos)) = self.tokens.first() {
                        let quantity = convert_quantity(quantity, *_pos);
                        self.tokens.remove(0);
                        Node::Single(Single::Char(char, quantity, pos))
                    } else {
                        Node::Single(Single::Char(char, Quantity::One, pos))
                    }
                }
                Token::Special(special, pos) => {
                    self.tokens.remove(0);
                    if let Some(Token::Quantity(quantity, _pos)) = self.tokens.first() {
                        let quantity = convert_quantity(quantity, *_pos);
                        self.tokens.remove(0);
                        Node::Single(Single::Special(convert_special(special), quantity, pos))
                    } else {
                        Node::Single(Single::Special(
                            convert_special(special),
                            Quantity::One,
                            pos,
                        ))
                    }
                }
                Token::AnchorStart(pos) => {
                    self.tokens.remove(0);
                    if let Some(Token::Quantity(quantity, _pos)) = self.tokens.first() {
                        let quantity = convert_quantity(quantity, *_pos);
                        self.tokens.remove(0);
                        Node::Single(Single::AnchorStart(quantity, pos))
                    } else {
                        Node::Single(Single::AnchorStart(Quantity::One, pos))
                    }
                }
                Token::AnchorEnd(pos) => {
                    self.tokens.remove(0);
                    if let Some(Token::Quantity(quantity, _pos)) = self.tokens.first() {
                        let quantity = convert_quantity(quantity, *_pos);
                        self.tokens.remove(0);
                        Node::Single(Single::AnchorEnd(quantity, pos))
                    } else {
                        Node::Single(Single::AnchorEnd(Quantity::One, pos))
                    }
                }
                Token::OpenGroup(opengroup, pos) => {
                    self.tokens.remove(0);
                    let mut closed = false;
                    let mut nodes = Vec::new();
                    while let Some(token) = self.tokens.first() {
                        let token = token.to_owned();
                        if let Token::CloseGroup = token {
                            self.tokens.remove(0);
                            closed = true;
                            break;
                        }
                        if let Opsult::Some(node) = self.parse_node() {
                            nodes.push(node);
                            continue;
                        }
                        return Opsult::Err(Error::new(
                            format!("Could not parse node at token {:?}", token),
                            pos,
                        ));
                    }

                    if !closed {
                        return Opsult::Err(Error::new(
                            "Did not find end of group".to_string(),
                            pos,
                        ));
                    } else {
                        Node::Group(Box::new(convert_group(
                            opengroup,
                            match nodes.len() {
                                0 => Node::Empty,
                                1 => nodes.first().unwrap().to_owned(),
                                _ => Node::Multiple(Multiple::AND(nodes)),
                            },
                            if let Some(Token::Quantity(quantity, _pos)) = self.tokens.first() {
                                let quantity = convert_quantity(quantity, *_pos);
                                self.tokens.remove(0);
                                quantity
                            } else {
                                Quantity::One
                            },
                            pos,
                        )))
                    }
                }
                Token::OpenBracket(openbracket, pos) => {
                    self.tokens.remove(0);
                    let mut closed = false;
                    let mut nodes = Vec::new();
                    while let Some(token) = self.tokens.first() {
                        let token = token.to_owned();
                        if let Token::CloseBracket = token {
                            self.tokens.remove(0);
                            closed = true;
                            break;
                        }
                        if let Opsult::Some(node) = self.parse_node() {
                            nodes.push(node);
                            continue;
                        }
                        return Opsult::Err(Error::new(
                            format!("Could not parse node at token {:?}", token),
                            pos,
                        ));
                    }

                    if !closed {
                        return Opsult::Err(Error::new(
                            "Did not find end of bracket".to_string(),
                            pos,
                        ));
                    } else {
                        Node::Multiple(convert_bracket(
                            openbracket,
                            nodes,
                            if let Some(Token::Quantity(quantity, _pos)) = self.tokens.first() {
                                let quantity = convert_quantity(quantity, *_pos);
                                self.tokens.remove(0);
                                quantity
                            } else {
                                Quantity::One
                            },
                            pos,
                        ))
                    }
                }
                _ => return Opsult::Err(Error::new("How can this be???".to_string(), 0)),
            })
        } else {
            Opsult::None
        }
    }
}

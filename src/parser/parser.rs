use super::{super::lexer::Token, super::Opsult, *};

pub struct Parser {
    tokens: Vec<Token>,
}

fn convert_quantity(quantity: &crate::lexer::Quantity) -> Quantity {
    match quantity {
        crate::lexer::Quantity::ZeroOrOne => Quantity::ZeroOrOne,
        crate::lexer::Quantity::ZeroOrMore => Quantity::ZeroOrMore,
        crate::lexer::Quantity::OneOrMore => Quantity::OneOrMore,
        crate::lexer::Quantity::Count(count) => Quantity::Count(*count),
        crate::lexer::Quantity::Range(start, end) => Quantity::Range(*start, *end),
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

fn convert_group(group: crate::lexer::OpenGroup, node: Node, quantity: Quantity) -> Group {
    match group {
        crate::lexer::OpenGroup::Capturing => Group::Capturing(node, quantity),
        crate::lexer::OpenGroup::NonCapturing => Group::NonCapturing(node, quantity),
        crate::lexer::OpenGroup::PositiveLookAhead => Group::PositiveLookAhead(node, quantity),
        crate::lexer::OpenGroup::PositiveLookBehind => Group::PositiveLookBehind(node, quantity),
        crate::lexer::OpenGroup::NegativeLookAhead => Group::NegativeLookAhead(node, quantity),
        crate::lexer::OpenGroup::NegativeLookBehind => Group::NegativeLookBehind(node, quantity),
    }
}

fn convert_bracket(
    bracket: crate::lexer::OpenBracket,
    nodes: Vec<Node>,
    quantity: Quantity,
) -> Multiple {
    match bracket {
        crate::lexer::OpenBracket::Negated => Multiple::NOR(nodes, quantity),
        crate::lexer::OpenBracket::NonNegated => Multiple::OR(nodes, quantity),
    }
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }

    pub fn parse(&mut self) -> Opsult<Node, String> {
        if self.tokens.len() == 0 {
            return Opsult::None;
        }

        let mut nodes = Vec::new();

        while self.tokens.len() > 0 {
            match self.parse_node() {
                Opsult::Some(node) => nodes.push(node),
                Opsult::Err(err) => return Opsult::Err(err),
                Opsult::None => break
            }
        }

        match nodes.len() {
            0 => Opsult::None,
            1 => Opsult::Some(nodes.first().unwrap().to_owned()),
            _ => Opsult::Some(Node::Multiple(Multiple::AND(nodes)))
        }
    }

    fn parse_node(&mut self) -> Opsult<Node, String> {
        if let Some(token) = self.tokens.first() {
            Opsult::Some(match token.to_owned() {
                Token::Literal(char, _) => {
                    self.tokens.remove(0);
                    if let Some(Token::Quantity(quantity, _)) = self.tokens.first() {
                        let quantity = convert_quantity(quantity);
                        self.tokens.remove(0);
                        Node::Single(Single::Char(char, quantity))
                    } else {
                        Node::Single(Single::Char(char, Quantity::One))
                    }
                }
                Token::Special(special, _) => {
                    self.tokens.remove(0);
                    if let Some(Token::Quantity(quantity, _)) = self.tokens.first() {
                        let quantity = convert_quantity(quantity);
                        self.tokens.remove(0);
                        Node::Single(Single::Special(convert_special(special), quantity))
                    } else {
                        Node::Single(Single::Special(convert_special(special), Quantity::One))
                    }
                }
                Token::AnchorStart(_) => {
                    self.tokens.remove(0);
                    if let Some(Token::Quantity(quantity, _)) = self.tokens.first() {
                        let quantity = convert_quantity(quantity);
                        self.tokens.remove(0);
                        Node::Single(Single::AnchorStart(quantity))
                    } else {
                        Node::Single(Single::AnchorStart(Quantity::One))
                    }
                }
                Token::AnchorEnd(_) => {
                    self.tokens.remove(0);
                    if let Some(Token::Quantity(quantity, _)) = self.tokens.first() {
                        let quantity = convert_quantity(quantity);
                        self.tokens.remove(0);
                        Node::Single(Single::AnchorEnd(quantity))
                    } else {
                        Node::Single(Single::AnchorEnd(Quantity::One))
                    }
                }
                Token::OpenGroup(opengroup, _) => {
                    self.tokens.remove(0);
                    let mut closed = false;
                    let mut nodes = Vec::new();
                    while let Some(token) = self.tokens.first() {
                        let token = token.to_owned();
                        if let Token::CloseGroup(_) = token {
                            self.tokens.remove(0);
                            closed = true;
                            break;
                        }
                        if let Opsult::Some(node) = self.parse_node() {
                            nodes.push(node);
                            continue;
                        }
                        return Opsult::Err(format!("Could not parse node at token {:?}", token));
                    }

                    if !closed {
                        return Opsult::Err("Did not find end of group".to_string());
                    } else {
                        Node::Group(Box::new(convert_group(
                            opengroup,
                            match nodes.len() {
                                0 => Node::Empty,
                                1 => nodes.first().unwrap().to_owned(),
                                _ => Node::Multiple(Multiple::AND(nodes))
                            },
                            if let Some(Token::Quantity(quantity, _)) = self.tokens.first() {
                                let quantity = convert_quantity(quantity);
                                self.tokens.remove(0);
                                quantity
                            } else {
                                Quantity::One
                            },
                        )))
                    }
                }
                Token::OpenBracket(openbracket, _) => {
                    self.tokens.remove(0);
                    let mut closed = false;
                    let mut nodes = Vec::new();
                    while let Some(token) = self.tokens.first() {
                        let token = token.to_owned();
                        if let Token::CloseBracket(_) = token {
                            self.tokens.remove(0);
                            closed = true;
                            break;
                        }
                        if let Opsult::Some(node) = self.parse_node() {
                            nodes.push(node);
                            continue;
                        }
                        return Opsult::Err(format!("Could not parse node at token {:?}", token));
                    }

                    if !closed {
                        return Opsult::Err("Did not find end of bracket".to_string());
                    } else {
                        Node::Multiple(convert_bracket(
                            openbracket,
                            nodes,
                            if let Some(Token::Quantity(quantity, _)) = self.tokens.first() {
                                let quantity = convert_quantity(quantity);
                                self.tokens.remove(0);
                                quantity
                            } else {
                                Quantity::One
                            },
                        ))
                    }
                }
                _ => return Opsult::Err("".to_string()),
            })
        } else {
            Opsult::None
        }
    }
}

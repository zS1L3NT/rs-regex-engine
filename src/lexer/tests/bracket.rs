use super::*;
use crate::{lex_invalid, lex_valid};

#[test]
fn nonnegated_invalid() {
    lex_invalid!("/[/", "Character class missing closing bracket");
}

#[test]
fn nonnegated_empty() {
    lex_valid!("/[]/", 2);
}

#[test]
fn nonnegated_with_literal() {
    lex_valid!(
        "/[a]/",
        vec![OpenBracket(NonNegated, 1), Literal('a', 2), CloseBracket]
    );
}

#[test]
fn negated_invalid() {
    lex_invalid!("/[^/", "Character class missing closing bracket");
}

#[test]
fn negated_empty() {
    lex_valid!("/[^]/", vec![OpenBracket(Negated, 1), CloseBracket]);
}

#[test]
fn negated_with_literal() {
    lex_valid!(
        "/[^a]/",
        vec![OpenBracket(Negated, 1), Literal('a', 3), CloseBracket]
    );
}

#[test]
fn literal_end() {
    lex_valid!("/]/", vec![Literal(']', 1)]);
}

#[test]
fn empty_with_literal_end() {
    lex_valid!(
        "/[]]/",
        vec![OpenBracket(NonNegated, 1), CloseBracket, Literal(']', 3)]
    );
}

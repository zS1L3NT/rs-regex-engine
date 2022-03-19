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
        vec![OpenBracket(NonNegated, 0), Literal('a', 1), CloseBracket(2)]
    );
}

#[test]
fn negated_invalid() {
    lex_invalid!("/[^/", "Character class missing closing bracket");
}

#[test]
fn negated_empty() {
    lex_valid!("/[^]/", vec![OpenBracket(Negated, 0), CloseBracket(2)]);
}

#[test]
fn negated_with_literal() {
    lex_valid!(
        "/[^a]/",
        vec![OpenBracket(Negated, 0), Literal('a', 2), CloseBracket(3)]
    );
}

#[test]
fn literal_end() {
    lex_valid!("/]/", vec![Literal(']', 0)]);
}

#[test]
fn empty_with_literal_end() {
    lex_valid!(
        "/[]]/",
        vec![OpenBracket(NonNegated, 0), CloseBracket(1), Literal(']', 2)]
    );
}

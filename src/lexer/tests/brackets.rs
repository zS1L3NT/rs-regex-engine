use super::*;
use crate::{invalid, valid};

#[test]
fn nonnegated_invalid() {
    invalid!("/[/", "Character class missing closing bracket");
}

#[test]
fn nonnegated_empty() {
    valid!("/[]/", 2);
}

#[test]
fn nonnegated_with_literal() {
    valid!(
        "/[a]/",
        vec![OpenBracket(NonNegated, 0), Literal('a', 1), CloseBracket(2)]
    );
}

#[test]
fn negated_invalid() {
    invalid!("/[^/", "Character class missing closing bracket");
}

#[test]
fn negated_empty() {
    valid!("/[^]/", vec![OpenBracket(Negated, 0), CloseBracket(2)]);
}

#[test]
fn negated_with_literal() {
    valid!(
        "/[^a]/",
        vec![OpenBracket(Negated, 0), Literal('a', 2), CloseBracket(3)]
    );
}

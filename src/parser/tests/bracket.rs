use super::*;
use crate::parse_valid;

#[test]
fn nonnegated_empty() {
    parse_valid!("/[]/", Multiple(OR(vec![], One, 1)));
}

#[test]
fn nonnegated_escaped() {
    parse_valid!(
        "/[\\[]/",
        Multiple(OR(vec![Single(Char('[', One, 2))], One, 1))
    );
}

#[test]
fn nonnegated_quantified() {
    parse_valid!(
        "/[a{1}]/",
        Multiple(OR(vec![Single(Char('a', Count(1, 3), 2))], One, 1))
    );
}

#[test]
fn negated_empty() {
    parse_valid!("/[^]/", Multiple(NOR(vec![], One, 1)));
}

#[test]
fn negated_escaped() {
    parse_valid!(
        "/[^\\[]/",
        Multiple(NOR(vec![Single(Char('[', One, 3))], One, 1))
    );
}

#[test]
fn negated_quantified() {
    parse_valid!(
        "/[^a{1}]/",
        Multiple(NOR(vec![Single(Char('a', Count(1, 4), 3))], One, 1))
    );
}

#[test]
fn nonnegated_with_literal() {
    parse_valid!(
        "/[]]/",
        Multiple(AND(vec![
            Multiple(OR(vec![], One, 1)),
            Single(Char(']', One, 3))
        ]))
    );
}

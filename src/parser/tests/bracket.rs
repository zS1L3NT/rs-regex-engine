use super::*;
use crate::parse_valid;

#[test]
fn nonnegated_empty() {
    parse_valid!("/[]/", Multiple(OR(vec![], One)));
}

#[test]
fn nonnegated_escaped() {
    parse_valid!("/[\\[]/", Multiple(OR(vec![Single(Char('[', One))], One)));
}

#[test]
fn nonnegated_quantified() {
    parse_valid!(
        "/[a{1}]/",
        Multiple(OR(vec![Single(Char('a', Count(1)))], One))
    );
}

#[test]
fn negated_empty() {
    parse_valid!("/[^]/", Multiple(NOR(vec![], One)));
}

#[test]
fn negated_escaped() {
    parse_valid!(
        "/[^\\[]/",
        Multiple(NOR(vec![Single(Char('[', One))], One))
    );
}

#[test]
fn negated_quantified() {
    parse_valid!(
        "/[^a{1}]/",
        Multiple(NOR(vec![Single(Char('a', Count(1)))], One))
    );
}

#[test]
fn nonnegated_with_literal() {
    parse_valid!(
        "/[]]/",
        Multiple(AND(vec![
            Multiple(OR(vec![], One)),
            Single(Char(']', One))
        ]))
    );
}

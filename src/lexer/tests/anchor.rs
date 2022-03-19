use super::*;
use crate::lex_valid;

#[test]
fn start() {
    lex_valid!("/^/", 1);
}

#[test]
fn end() {
    lex_valid!("/$/", 1);
}

#[test]
fn both() {
    lex_valid!("/^$/", 2);
}

#[test]
fn both_with_literal() {
    lex_valid!("/^a$/", vec![AnchorStart(0), Literal('a', 1), AnchorEnd(2)]);
}

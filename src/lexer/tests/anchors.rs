use super::*;
use crate::valid;

#[test]
fn start() {
    valid!("/^/", 1);
}

#[test]
fn end() {
    valid!("/$/", 1);
}

#[test]
fn both() {
    valid!("/^$/", 2);
}

#[test]
fn both_with_literal() {
    valid!("/^a$/", vec![AnchorStart(0), Literal('a', 1), AnchorEnd(2)]);
}

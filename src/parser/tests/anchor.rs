use super::*;
use crate::parse_valid;

#[test]
fn start() {
    parse_valid!("/^/", Single(AnchorStart(One, 1)));
}

#[test]
fn end() {
    parse_valid!("/$/", Single(AnchorEnd(One, 1)));
}

#[test]
fn both() {
    parse_valid!(
        "/^$/",
        Multiple(AND(vec![
            Single(AnchorStart(One, 1)),
            Single(AnchorEnd(One, 2))
        ]))
    );
}

#[test]
fn both_with_literal() {
    parse_valid!(
        "/^a$/",
        Multiple(AND(vec![
            Single(AnchorStart(One, 1)),
            Single(Char('a', One, 2)),
            Single(AnchorEnd(One, 3)),
        ]))
    );
}

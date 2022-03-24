use super::*;
use crate::parse_valid;

#[test]
fn start() {
    parse_valid!("/^/", Single(AnchorStart(One)));
}

#[test]
fn end() {
    parse_valid!("/$/", Single(AnchorEnd(One)));
}

#[test]
fn both() {
    parse_valid!(
        "/^$/",
        Multiple(AND(vec![
            Single(AnchorStart(One)),
            Single(AnchorEnd(One))
        ]))
    );
}

#[test]
fn both_with_literal() {
    parse_valid!(
        "/^a$/",
        Multiple(AND(vec![
            Single(AnchorStart(One)),
            Single(Char('a', One)),
            Single(AnchorEnd(One)),
        ]))
    );
}

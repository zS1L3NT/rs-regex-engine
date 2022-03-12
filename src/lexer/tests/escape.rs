use super::*;
use crate::{invalid, valid};

#[test]
fn period() {
    valid!("/\\./", vec![Literal('.', 0)]);
}

#[test]
fn plus() {
    valid!("/\\+/", vec![Literal('+', 0)]);
}

#[test]
fn asterisk() {
    valid!("/\\*/", vec![Literal('*', 0)]);
}

#[test]
fn question_mark() {
    valid!("/\\?/", vec![Literal('?', 0)]);
}

#[test]
fn caret() {
    valid!("/\\^/", vec![Literal('^', 0)]);
}

#[test]
fn dollar_sign() {
    valid!("/\\$/", vec![Literal('$', 0)]);
}

#[test]
fn open_group() {
    valid!("/\\(/", vec![Literal('(', 0)]);
}

#[test]
fn close_group() {
    valid!("/\\)/", vec![Literal(')', 0)]);
}

#[test]
fn open_bracket() {
    valid!("/\\[/", vec![Literal('[', 0)]);
}

#[test]
fn close_bracket() {
    valid!("/\\]/", vec![Literal(']', 0)]);
}

#[test]
fn open_range() {
    valid!("/\\{/", vec![Literal('{', 0)]);
}

#[test]
fn close_range() {
    valid!("/\\}/", vec![Literal('}', 0)]);
}

#[test]
fn pipe() {
    valid!("/\\|/", vec![Literal('|', 0)]);
}

#[test]
fn backslash() {
    valid!("/\\\\/", vec![Literal('\\', 0)]);
}

#[test]
fn invalid() {
    invalid!("/\\/", "Pattern may not end with a trailing backslash");
}

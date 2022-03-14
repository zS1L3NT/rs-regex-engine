use super::*;
use crate::{lex_invalid, lex_valid};

#[test]
fn period() {
    lex_valid!("/\\./", vec![Literal('.', 0)]);
}

#[test]
fn plus() {
    lex_valid!("/\\+/", vec![Literal('+', 0)]);
}

#[test]
fn asterisk() {
    lex_valid!("/\\*/", vec![Literal('*', 0)]);
}

#[test]
fn question_mark() {
    lex_valid!("/\\?/", vec![Literal('?', 0)]);
}

#[test]
fn caret() {
    lex_valid!("/\\^/", vec![Literal('^', 0)]);
}

#[test]
fn dollar_sign() {
    lex_valid!("/\\$/", vec![Literal('$', 0)]);
}

#[test]
fn open_group() {
    lex_valid!("/\\(/", vec![Literal('(', 0)]);
}

#[test]
fn close_group() {
    lex_valid!("/\\)/", vec![Literal(')', 0)]);
}

#[test]
fn open_bracket() {
    lex_valid!("/\\[/", vec![Literal('[', 0)]);
}

#[test]
fn close_bracket() {
    lex_valid!("/\\]/", vec![Literal(']', 0)]);
}

#[test]
fn open_range() {
    lex_valid!("/\\{/", vec![Literal('{', 0)]);
}

#[test]
fn close_range() {
    lex_valid!("/\\}/", vec![Literal('}', 0)]);
}

#[test]
fn pipe() {
    lex_valid!("/\\|/", vec![Literal('|', 0)]);
}

#[test]
fn backslash() {
    lex_valid!("/\\\\/", vec![Literal('\\', 0)]);
}

#[test]
fn invalid() {
    lex_invalid!("/\\/", "Pattern may not end with a trailing backslash");
}

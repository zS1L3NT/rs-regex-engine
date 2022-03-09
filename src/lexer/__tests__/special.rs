use super::*;
use crate::valid;

#[test]
fn special_whitespace() {
    valid!("/s/", vec![Literal('s', 0)]);
    valid!("/\\s/", vec![Special(Whitespace(0))]);
    valid!("/\\\\s/", vec![Literal('\\', 0), Literal('s', 1)]);
    valid!("/S/", vec![Literal('S', 0)]);
    valid!("/\\S/", vec![Special(NonWhitespace(0))]);
    valid!("/\\\\S/", vec![Literal('\\', 0), Literal('S', 1)]);
}

#[test]
fn special_digit() {
    valid!("/d/", vec![Literal('d', 0)]);
    valid!("/\\d/", vec![Special(Digit(0))]);
    valid!("/\\\\d/", vec![Literal('\\', 0), Literal('d', 1)]);
    valid!("/D/", vec![Literal('D', 0)]);
    valid!("/\\D/", vec![Special(NonDigit(0))]);
    valid!("/\\\\D/", vec![Literal('\\', 0), Literal('D', 1)]);
}

#[test]
fn special_word() {
    valid!("/w/", vec![Literal('w', 0)]);
    valid!("/\\w/", vec![Special(Word(0))]);
    valid!("/\\\\w/", vec![Literal('\\', 0), Literal('w', 1)]);
    valid!("/W/", vec![Literal('W', 0)]);
    valid!("/\\W/", vec![Special(NonWord(0))]);
    valid!("/\\\\W/", vec![Literal('\\', 0), Literal('W', 1)]);
}

#[test]
fn special_boundary() {
    valid!("/b/", vec![Literal('b', 0)]);
    valid!("/\\b/", vec![Special(Boundary(0))]);
    valid!("/\\\\b/", vec![Literal('\\', 0), Literal('b', 1)]);
    valid!("/B/", vec![Literal('B', 0)]);
    valid!("/\\B/", vec![Special(Boundary(0))]);
    valid!("/\\\\B/", vec![Literal('\\', 0), Literal('B', 1)]);
}

#[test]
fn special_linebreak() {
    valid!("/n/", vec![Literal('n', 0)]);
    valid!("/\\n/", vec![Special(LineBreak(0))]);
    valid!("/\\\\n/", vec![Literal('\\', 0), Literal('n', 1)]);
}

#[test]
fn special_carriagereturn() {
    valid!("/r/", vec![Literal('r', 0)]);
    valid!("/\\r/", vec![Special(CarriageReturn(0))]);
    valid!("/\\\\r/", vec![Literal('\\', 0), Literal('r', 1)]);
}

#[test]
fn special_tab() {
    valid!("/t/", vec![Literal('t', 0)]);
    valid!("/\\t/", vec![Special(Tab(0))]);
    valid!("/\\\\t/", vec![Literal('\\', 0), Literal('t', 1)]);
}

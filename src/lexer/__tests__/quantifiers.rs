use super::*;
use crate::{invalid, valid};

#[test]
fn quantifier_asterisk() {
    invalid!("/*/", "The preceding token is not quantifiable");
    valid!("/a*/", vec![Literal('a', 0), Quantifier(Asterisk(1))]);
}

#[test]
fn quantifier_plus() {
    invalid!("/+/", "The preceding token is not quantifiable");
    valid!("/a+/", vec![Literal('a', 0), Quantifier(Plus(1))]);
}

#[test]
fn quantifier_question_mark() {
    invalid!("/?/", "The preceding token is not quantifiable");
    valid!("/a?/", vec![Literal('a', 0), Quantifier(QuestionMark(1))]);
}

#[test]
fn quantifier_count() {
    invalid!("/{1}/", "The preceding token is not quantifiable");
    valid!("/a{}/", 3);
    valid!("/a{1}/", vec![Literal('a', 0), Quantifier(Count(1, 1))]);
}

#[test]
fn quantifier_range() {
    invalid!("/{1,2}/", "The preceding token is not quantifiable");
    valid!("/a{,}/", 4);
    valid!(
        "/a{1,}/",
        vec![Literal('a', 0), Quantifier(Range(Some(1), None, 1))]
    );
    valid!(
        "/a{1,2}/",
        vec![Literal('a', 0), Quantifier(Range(Some(1), Some(2), 1))]
    );
    valid!("/a{{1,}}/", 4);
    valid!("/a(bc){1,}/", 6);
}

use super::*;
use crate::{invalid, valid};

#[test]
fn brackets() {
    invalid!("/[/", "Character class missing closing bracket");
    valid!("/[]/", 2);
    valid!(
        "/[a(bc)]/",
        vec![
            OpenBracket(NonNegated, 0),
            Literal('a', 1),
            OpenGroup(Capturing, 2),
            Literal('b', 3),
            Literal('c', 4),
            CloseGroup(5),
            CloseBracket(6)
        ]
    );
}

#[test]
fn brackets_negation() {
    invalid!("/[^/", "Character class missing closing bracket");
    valid!("/[^]/", vec![OpenBracket(Negated, 0), CloseBracket(2)]);
    valid!(
        "/[^a(bc)]/",
        vec![
            OpenBracket(Negated, 0),
            Literal('a', 2),
            OpenGroup(Capturing, 3),
            Literal('b', 4),
            Literal('c', 5),
            CloseGroup(6),
            CloseBracket(7)
        ]
    );
}

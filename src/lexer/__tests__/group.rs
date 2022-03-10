use super::*;
use crate::{invalid, valid};

#[test]
fn groups() {
    invalid!("/(/", "Incomplete group structure");
    invalid!("/)/", "Unmatched parenthesis");
    invalid!("/(?/", "Incomplete group structure");
}

#[test]
fn group_capturing() {
    valid!("/()/", vec![OpenGroup(Capturing(0)), CloseGroup(1)]);
    valid!(
        "/(abc)/",
        vec![
            OpenGroup(Capturing(0)),
            Literal('a', 1),
            Literal('b', 2),
            Literal('c', 3),
            CloseGroup(4)
        ]
    );
}

#[test]
fn group_non_capturing() {
    invalid!("/(?:/", "Incomplete group structure");
    valid!("/(?:)/", vec![OpenGroup(NonCapturing(0)), CloseGroup(1)]);
    valid!(
        "/(:?)/",
        vec![
            OpenGroup(Capturing(0)),
            Literal(':', 1),
            Quantifier(QuestionMark(2)),
            CloseGroup(3)
        ]
    );
    valid!(
        "/(?:x)/",
        vec![OpenGroup(NonCapturing(0)), Literal('x', 3), CloseGroup(4)]
    );
}

#[test]
fn group_positive_lookahead() {
    invalid!("/(?=/", "Incomplete group structure");
    valid!(
        "/(?=)/",
        vec![OpenGroup(PositiveLookAhead(0)), CloseGroup(1)]
    );
    valid!(
        "/(=?)/",
        vec![
            OpenGroup(Capturing(0)),
            Literal('=', 1),
            Quantifier(QuestionMark(2)),
            CloseGroup(3)
        ]
    );
    valid!(
        "/(?=x)/",
        vec![
            OpenGroup(PositiveLookAhead(0)),
            Literal('x', 3),
            CloseGroup(4)
        ]
    );
}

#[test]
fn group_positive_lookbehind() {
    invalid!("/(?</", "Incomplete group structure");
    invalid!("/(?<=/", "Incomplete group structure");
    valid!(
        "/(?<=)/",
        vec![OpenGroup(PositiveLookBehind(0)), CloseGroup(1)]
    );
    valid!(
        "/(?<=x)/",
        vec![
            OpenGroup(PositiveLookBehind(0)),
            Literal('x', 4),
            CloseGroup(5)
        ]
    );
    valid!(
        "/(?=<x)/",
        vec![
            OpenGroup(PositiveLookAhead(0)),
            Literal('<', 3),
            Literal('x', 4),
            CloseGroup(5)
        ]
    );
}

#[test]
fn group_negative_lookahead() {
    invalid!("/(?!/", "Incomplete group structure");
    valid!(
        "/(?!)/",
        vec![OpenGroup(PositiveLookAhead(0)), CloseGroup(1)]
    );
    valid!(
        "/(!?)/",
        vec![
            OpenGroup(Capturing(0)),
            Literal('!', 1),
            Quantifier(QuestionMark(2)),
            CloseGroup(3)
        ]
    );
    valid!(
        "/(?!x)/",
        vec![
            OpenGroup(PositiveLookAhead(0)),
            Literal('x', 3),
            CloseGroup(4)
        ]
    );
}

#[test]
fn group_negative_lookbehind() {
    invalid!("/(?</", "Incomplete group structure");
    invalid!("/(?<!/", "Incomplete group structure");
    valid!(
        "/(?<!)/",
        vec![OpenGroup(NegativeLookBehind(0)), CloseGroup(1)]
    );
    valid!(
        "/(?<!x)/",
        vec![
            OpenGroup(NegativeLookBehind(0)),
            Literal('x', 4),
            CloseGroup(5)
        ]
    );
    valid!(
        "/(?!<x)/",
        vec![
            OpenGroup(NegativeLookAhead(0)),
            Literal('<', 3),
            Literal('x', 4),
            CloseGroup(5)
        ]
    );
}

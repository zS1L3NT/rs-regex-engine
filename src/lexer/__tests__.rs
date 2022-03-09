use crate::lexer::{OpenGroup, Quantifier, Token};

use super::Lexer;
use OpenGroup::*;
use Quantifier::*;
use Token::*;

macro_rules! valid {
    ($a:literal, $count:literal) => {
        let count = if let Ok(tokens) = Lexer::new($a.to_string()).lex() {
            tokens.len()
        } else {
            println!("\"{}\" was not Ok", $a);
            0
        };
        assert_eq!(count, $count);
    };
    ($a:literal, $tokens:expr) => {
        let tokens = if let Ok(tokens) = Lexer::new($a.to_string()).lex() {
            tokens
        } else {
            println!("\"{}\" was not Ok", $a);
            vec![]
        };
        assert_eq!(tokens, $tokens);
    };
}

macro_rules! invalid {
    ($a:literal, $msg:literal) => {
        let msg_starts_with = if let Err(err) = Lexer::new($a.to_string()).lex() {
            err.msg.starts_with($msg)
        } else {
            println!("\"{}\" was not Err", $a);
            false
        };
        assert!(msg_starts_with);
    };
}

#[test]
fn regex_start() {
    valid!("/abc/", 3);
    valid!("/bcd/", 3);
    invalid!("abc", "Expected RegExp to start with a </>");
    invalid!("bcd", "Expected RegExp to start with a </>");
}

#[test]
fn regex_end() {
    valid!("/abc/", 3);
    valid!("/bcd/", 3);
    invalid!("abc", "Expected RegExp to start with a </>");
    invalid!("bcd", "Expected RegExp to start with a </>");
}

#[test]
fn length_check() {
    valid!("//", 0);
    valid!("/abc/", 3);
    valid!("/^abc$/", 5);
    valid!("/abc{2,5}/", 3);
    valid!("/a(b|c)[^d]/", 9);
    valid!("/\\ba\\b", 3);
}

#[test]
fn anchors() {
    valid!("/^/", 1);
    valid!("/^$/", 2);
    valid!(
        "/^a$/",
        vec![AssertStart(0), Literal('a', 1), AssertEnd(2),]
    );
}

#[test]
fn quantifier_asterisk() {
    invalid!("*", "The preceding token is not quantifiable");
    valid!("/a*/", vec![Literal('a', 0), Quantifier(Asterisk(1))]);
}

#[test]
fn quantifier_plus() {
    invalid!("+", "The preceding token is not quantifiable");
    valid!("/a+/", vec![Literal('a', 0), Quantifier(Plus(1))]);
}

#[test]
fn quantifier_question_mark() {
    invalid!("?", "The preceding token is not quantifiable");
    valid!("/a?/", vec![Literal('a', 0), Quantifier(QuestionMark(1))]);
}

#[test]
fn quantifier_count() {
    invalid!("{1}", "The preceding token is not quantifiable");
    valid!("/a{}/", 3);
    valid!("/a{1}/", vec![Literal('a', 0), Quantifier(Count(1, 1))]);
}

#[test]
fn quantifier_range() {
    invalid!("{1,2}", "The preceding token is not quantifiable");
    valid!("/a{,}/", 4);
    valid!(
        "/a{1,}/",
        vec![Literal('a', 0), Quantifier(Range(Some(1), None, 1))]
    );
    valid!(
        "/a{1,2}/",
        vec![Literal('a', 0), 0, Quantifier(Range(Some(1), Some(2), 1))]
    );
    valid!("/a{{1,}}/", 4);
    valid!("/a(bc){1,}", 6);
}

#[test]
fn groups() {
    invalid!("(", "Incomplete group structure");
    invalid!(")", "Unmatched parenthesis");
    invalid!("(?", "Incomplete group structure");
}

#[test]
fn group_capturing() {
    valid!("()", vec![OpenGroup(Capturing(0)), CloseGroup(1)]);
    valid!(
        "(abc)",
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
    invalid!("(?:", "Incomplete group structure");
    valid!("(?:)", vec![OpenGroup(NonCapturing(0)), CloseGroup(1)]);
    valid!(
        "(:?)",
        vec![
            OpenGroup(Capturing(0)),
            Literal(":", 1),
            Quantifier(QuestionMark(2)),
            CloseGroup(3)
        ]
    );
    valid!(
        "(?:x)",
        vec![OpenGroup(NonCapturing(0)), Literal('x', 3), CloseGroup(4)]
    );
}

#[test]
fn group_positive_lookahead() {
    invalid!("(?=", "Incomplete group structure");
    valid!("(?=)", vec![OpenGroup(PositiveLookAhead(0)), CloseGroup(1)]);
    valid!(
        "(=?)",
        vec![
            OpenGroup(Capturing(0)),
            Literal("=", 1),
            Quantifier(QuestionMark(2)),
            CloseGroup(3)
        ]
    );
    valid!(
        "(?=x)",
        vec![
            OpenGroup(PositiveLookAhead(0)),
            Literal('x', 3),
            CloseGroup(4)
        ]
    );
}

#[test]
fn group_positive_lookbehind() {
    invalid!("(?<", "Incomplete group structure");
    invalid!("(?<=", "Incomplete group structure");
    valid!(
        "(?<=)",
        vec![OpenGroup(PositiveLookBehind(0)), CloseGroup(1)]
    );
    valid!(
        "(?<=x)",
        vec![
            OpenGroup(PositiveLookBehind(0)),
            Literal('x', 4),
            CloseGroup(5)
        ]
    );
    valid!(
        "(?=<x)",
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
    invalid!("(?!", "Incomplete group structure");
    valid!("(?!)", vec![OpenGroup(PositiveLookAhead(0)), CloseGroup(1)]);
    valid!(
        "(!?)",
        vec![
            OpenGroup(Capturing(0)),
            Literal("!", 1),
            Quantifier(QuestionMark(2)),
            CloseGroup(3)
        ]
    );
    valid!(
        "(?!x)",
        vec![
            OpenGroup(PositiveLookAhead(0)),
            Literal('x', 3),
            CloseGroup(4)
        ]
    );
}

#[test]
fn group_negative_lookbehind() {
    invalid!("(?<", "Incomplete group structure");
    invalid!("(?<!", "Incomplete group structure");
    valid!(
        "(?<!)",
        vec![OpenGroup(NegativeLookBehind(0)), CloseGroup(1)]
    );
    valid!(
        "(?<!x)",
        vec![
            OpenGroup(NegativeLookBehind(0)),
            Literal('x', 4),
            CloseGroup(5)
        ]
    );
    valid!(
        "(?!<x)",
        vec![
            OpenGroup(NegativeLookAhead(0)),
            Literal('<', 3),
            Literal('x', 4),
            CloseGroup(5)
        ]
    );
}

#[test]
fn brackets() {}

#[test]
fn brackets_negation() {}

#[test]
fn special() {}

#[test]
fn escape() {}

#[test]
fn test() {
    valid!(
        "/a(b|c)[^d]/",
        vec![
            Literal('a', 0),
            Capturing(1),
            Literal('b', 2),
            Pipe(3),
            Literal('c', 4),
            CloseGroup(5),
            OpenBracket(false, 6),
            Literal('d', 8),
            CloseBracket(9),
        ]
    );
    valid!(
        "/\\ba\\b",
        vec![Special('b', 0), Literal('a', 1), Special('b', 2)]
    );
}

use super::Lexer;

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
    equals!("/abc/", Ok);
    equals!("/bcd/", Ok);
    equals!("abc", Err);
    equals!("bcd", Err);
}

#[test]
fn regex_end() {
    equals!("/abc/", Ok);
    equals!("/bcd/", Ok);
    equals!("abc", Err);
    equals!("bcd", Err);
}

#[test]
fn length_check() {
    equals!("//", Err);
}

#[test]
fn valid_all_literals() {
    equals!("/abc/", Ok, 3);
    equals!("/abc/", Ok, 3);
}

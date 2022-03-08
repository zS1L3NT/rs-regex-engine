use super::Lexer;

macro_rules! equals {
    ($a:literal, Ok) => {
        let was_ok = if let Ok(_) = Lexer::new($a.to_string()).lex() {
            true
        } else {
            println!("\"{}\" was not Ok", $a);
            false
        };
        assert!(was_ok);
    };
    ($a:literal, Ok, $count:literal) => {
        let count = if let Ok(tokens) = Lexer::new($a.to_string()).lex() {
            tokens.len()
        } else {
            println!("\"{}\" was not Ok", $a);
            0
        };
        assert_eq!(count, $count);
    };
    ($a:literal, Ok, $tokens:expr) => {
        let tokens = if let Ok(tokens) = Lexer::new($a.to_string()).lex() {
            tokens
        } else {
            println!("\"{}\" was not Ok", $a);
            vec![]
        };
        assert_eq!(tokens, $tokens);
    };
    ($a:literal, Err) => {
        let was_err = if let Err(_) = Lexer::new($a.to_string()).lex() {
            true
        } else {
            println!("\"{}\" was not Err", $a);
            false
        };
        assert!(was_err);
    };
    ($a:literal, Err, $msg:literal) => {
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

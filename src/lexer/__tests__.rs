use super::Lexer;

macro_rules! equals {
    ($a:literal, Ok) => {
        assert!(if let Ok(_) = Lexer::new($a.to_string()).lex() {
            true
        } else {
            println!("\"{}\" was not Ok", $a);
            false
        })
    };
    ($a:literal, Ok, $count:literal) => {
        assert_eq!(
            if let Ok(tokens) = Lexer::new($a.to_string()).lex() {
                tokens.len()
            } else {
                println!("\"{}\" was not Ok", $a);
                0
            },
            $count
        )
    };
    ($a:literal, Ok, $tokens:expr) => {
        assert_eq!(
            if let Ok(tokens) = Lexer::new($a.to_string()).lex() {
                tokens
            } else {
                println!("\"{}\" was not Ok", $a);
                vec![]
            },
            $tokens
        )
    };
    ($a:literal, Err) => {
        assert!(if let Err(_) = Lexer::new($a.to_string()).lex() {
            true
        } else {
            println!("\"{}\" was not Err", $a);
            false
        })
    };
    ($a:literal, Err, $msg:literal) => {
        assert!(if let Err(err) = Lexer::new($a.to_string()).lex() {
            err.msg.starts_with($msg)
        } else {
            println!("\"{}\" was not Err", $a);
            false
        })
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

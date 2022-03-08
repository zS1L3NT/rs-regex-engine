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
    ($a:literal, Err) => {
        assert!(if let Err(_) = Lexer::new($a.to_string()).lex() {
            true
        } else {
            println!("\"{}\" was not Err", $a);
            false
        })
    };
}

#[test]
fn valid_regex_start() {
    equals!("/abc/", Ok);
    equals!("/bcd/", Ok);
}

#[test]
fn invalid_regex_start() {
    equals!("abc", Err);
    equals!("bcd", Err);
}

#[test]
fn valid_regex_end() {
    equals!("/abc/", Ok);
    equals!("/bcd/", Ok);
}

#[test]
fn invalid_regex_end() {
    equals!("abc", Err);
    equals!("bcd", Err);
}

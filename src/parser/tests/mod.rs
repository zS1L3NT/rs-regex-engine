
#[macro_export]
macro_rules! parse_valid {
    ($a:literal, $node:expr) => {
        let tokens = Lexer::new($a.to_string()).lex().unwrap();
        let value = if let Some(value) = Parser::new(tokens).parse() {
            value
        } else {
            println!("{} was not Some");
            None
        }

        assert_eq!(value, $node);
    };
}

#[macro_export]
macro_rules! parse_invalid {
    ($a:literal, $msg:literal) => {
        let tokens = Lexer::new($a.to_string()).lex().unwrap();
        let msg_starts_with = if let Err(err) = Parser::new(tokens).parse() {
            err.msg.starts_with($msg)
        } else {
            println!("\"{}\" was not Err", $a);
            false
        };
        assert!(msg_starts_with);
    }
}
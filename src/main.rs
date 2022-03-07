use regex_engine::{MatchesRegExp, ToRegExp};

fn main() {
    let string = String::new();
    let regexp = string.to_regexp().unwrap();
    regexp.matches_string(&string);
    string.matches_regexp(regexp);
}

use crate::regex::RegExp;

pub trait MatchesRegExp {
    fn matches_regexp(&self, regexp: RegExp) -> bool {
        true
    }
}

impl MatchesRegExp for String {
    fn matches_regexp(&self, regexp: RegExp) -> bool {
        true
    }
}

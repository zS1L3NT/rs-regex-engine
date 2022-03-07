use std::error::Error;

use crate::regex::RegExp;

pub trait ToRegExp {
    fn to_regexp(&self) -> Result<RegExp, Box<dyn Error>>;
}

impl ToRegExp for String {
    fn to_regexp(&self) -> Result<RegExp, Box<dyn Error>> {
        Ok(RegExp::new())
    }
}

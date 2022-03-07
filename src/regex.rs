pub struct RegExp {}

impl RegExp {
    pub fn new() -> Self {
        Self {}
    }

    pub fn matches_string(&self, string: &str) {
        println!("{}", string);
    }
}

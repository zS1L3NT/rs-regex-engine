use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Error {
    pub msg: String,
    pub pos: usize,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (position {})", self.msg, self.pos)
    }
}

impl Error {
    pub fn new(msg: String, pos: usize) -> Self {
        Self { msg, pos }
    }
}

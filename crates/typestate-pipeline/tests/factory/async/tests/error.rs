use core::fmt;

#[derive(Debug)]
pub enum BadInput {
    Empty,
}

impl fmt::Display for BadInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("empty")
    }
}
impl std::error::Error for BadInput {}

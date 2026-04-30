use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Bad(&'static str),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Bad(m) => write!(f, "bad: {m}"),
        }
    }
}
impl std::error::Error for AppError {}

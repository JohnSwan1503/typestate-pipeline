use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Empty(&'static str),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Empty(field) => write!(f, "{field} is empty"),
        }
    }
}
impl std::error::Error for AppError {}

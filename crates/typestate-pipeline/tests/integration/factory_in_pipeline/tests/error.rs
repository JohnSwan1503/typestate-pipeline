use std::fmt;

#[derive(Debug)]
pub enum SubmitError {
    Empty(&'static str),
}

impl fmt::Display for SubmitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SubmitError::Empty(field) => write!(f, "{field} is empty"),
        }
    }
}

impl std::error::Error for SubmitError {}

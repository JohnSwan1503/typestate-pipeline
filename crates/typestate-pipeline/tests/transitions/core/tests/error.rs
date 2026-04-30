use std::fmt;

#[derive(Debug)]
pub enum TestError {
    Invalid(&'static str),
}

impl fmt::Display for TestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TestError::Invalid(m) => write!(f, "invalid: {m}"),
        }
    }
}

impl std::error::Error for TestError {}

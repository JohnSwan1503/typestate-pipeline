use core::fmt;

#[derive(Debug)]
pub struct DummyError;

impl fmt::Display for DummyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("dummy")
    }
}
impl std::error::Error for DummyError {}

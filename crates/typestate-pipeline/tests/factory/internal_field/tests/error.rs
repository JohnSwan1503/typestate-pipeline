use core::fmt;

#[derive(Debug)]
pub enum AppError {}
impl fmt::Display for AppError {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}
impl std::error::Error for AppError {}

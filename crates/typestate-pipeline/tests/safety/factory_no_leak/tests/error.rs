#[derive(Debug)]
pub struct Reject;

impl std::fmt::Display for Reject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("rejected")
    }
}
impl std::error::Error for Reject {}

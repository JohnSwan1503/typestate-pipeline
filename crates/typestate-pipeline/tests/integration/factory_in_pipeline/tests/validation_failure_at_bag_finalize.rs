#[path = "shared.rs"]
mod shared;

use shared::{Server, SubmitError, User, drafting};

pub fn main() {
    let server = Server::default();
    let pipeline = drafting(
        &server,
        User {
            name: String::new(),
            email: "no-name@example.com".to_owned(),
            age: 0,
        },
    );

    match pipeline.submit() {
        Err(SubmitError::Empty(field)) => assert_eq!(field, "name"),
        Ok(_) => panic!("expected validation error"),
    }
}

use super::{BadInput, UserProfileFactory};

pub async fn main() {
    let bag = UserProfileFactory::new().name("Bob".to_owned()).await;
    let result = bag.email(String::new()).await;
    match result {
        Err(BadInput::Empty) => {}
        Ok(_) => panic!("expected BadInput::Empty"),
    }
}

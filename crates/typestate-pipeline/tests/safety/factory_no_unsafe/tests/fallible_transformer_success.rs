use super::ValidationError;
use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
#[factory(no_unsafe, error = ValidationError)]
struct ValidatedUser {
    #[field(required, setter = require_nonempty, fallible)]
    name: String,
}

fn require_nonempty(value: String) -> Result<String, ValidationError> {
    if value.is_empty() {
        Err(ValidationError("name is empty"))
    } else {
        Ok(value)
    }
}

pub fn main() {
    let bag = ValidatedUserFactory::new()
        .name("Carol".to_owned())
        .expect("non-empty");
    assert_eq!(bag.name(), "Carol");
    let u = bag.finalize();
    assert_eq!(u.name, "Carol");
}

use typestate_pipeline::TypestateFactory;

#[derive(Debug)]
struct ValidationError(&'static str);

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::error::Error for ValidationError {}

#[derive(TypestateFactory)]
#[factory(error = ValidationError)]
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
    match ValidatedUserFactory::new().name(String::new()) {
        Ok(_) => panic!("expected validation failure"),
        Err(ValidationError(msg)) => assert_eq!(msg, "name is empty"),
    }
}

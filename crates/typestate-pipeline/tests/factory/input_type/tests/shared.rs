use typestate_pipeline::TypestateFactory;

#[derive(Debug, TypestateFactory)]
pub struct Profile {
    #[field(required)]
    pub name: String,
    /// Worker name — `Option<String>` storage, but the user-facing
    /// setter takes plain `String` and the `wrap_some` transformer
    /// lifts.
    #[field(default = None, setter = wrap_some, input = String)]
    pub worker: Option<String>,
}

pub fn wrap_some(s: String) -> Option<String> {
    Some(s)
}

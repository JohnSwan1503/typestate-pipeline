use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
struct OneFlag {
    #[field(required)]
    name: String,
}

pub fn main() {
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    assert_send::<OneFlagFactory<typestate_pipeline::Yes>>();
    assert_sync::<OneFlagFactory<typestate_pipeline::Yes>>();
    assert_send::<OneFlagFactory<typestate_pipeline::No>>();
    assert_sync::<OneFlagFactory<typestate_pipeline::No>>();
}

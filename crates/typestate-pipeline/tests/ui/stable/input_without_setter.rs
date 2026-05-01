use typestate_pipeline::TypestateFactory;

// `input = T` means the setter takes `T` instead of the stored field type.
// Without `setter = <fn>` there's no transformer to bridge the two, so the
// macro rejects this combination at the derive site instead of producing
// a non-typechecking setter signature.
#[derive(TypestateFactory)]
struct Bag {
    #[field(required, input = String)]
    label: u32,
}

fn main() {}

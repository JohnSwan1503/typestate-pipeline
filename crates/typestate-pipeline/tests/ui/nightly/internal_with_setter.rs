use typestate_pipeline::TypestateFactory;

// `internal` means "set at construction, locked from then on" — pairing it
// with `setter = …` (or any other mutability-implying attr) is a contract
// violation that the macro rejects at parse time. Here we exercise the
// `setter` clash; the same rejection applies to `optional`, `default`,
// `overridable`, `removable`, `fallible`, `async_fn`, and `input`.
#[derive(TypestateFactory)]
struct Bag {
    #[field(internal, setter = trim)]
    name: String,
}

fn trim(s: String) -> String {
    s.trim().to_owned()
}

fn main() {}

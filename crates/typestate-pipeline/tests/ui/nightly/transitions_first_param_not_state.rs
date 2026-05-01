use typestate_pipeline::{transitions, Mode, Pipeline, Resolved};

struct Author<'a, S, M = Resolved>(Pipeline<'a, (), (), S, (), M>)
where
    M: Mode<'a, S, ()>;

struct A;
struct B;

#[transitions(error = ())]
impl<'a> Author<'a, A> {
    #[transition(into = B)]
    fn noop(thing: A) -> B {
        let _ = thing;
        B
    }
}

fn main() {}

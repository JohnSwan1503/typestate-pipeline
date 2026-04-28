use typestate_pipeline::{transitions, Mode, Pipeline, Resolved};

struct Author<'a, S, M = Resolved>(Pipeline<'a, (), (), S, (), M>)
where
    M: Mode<'a, S, ()>;

struct A;

#[transitions]
impl<'a> Author<'a, A> {
    #[transition(into = A)]
    fn noop(state: A) -> A {
        state
    }
}

fn main() {}

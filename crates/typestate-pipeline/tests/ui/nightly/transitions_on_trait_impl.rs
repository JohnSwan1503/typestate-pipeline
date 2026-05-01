use typestate_pipeline::{transitions, Mode, Pipeline, Resolved};

struct Author<'a, S, M = Resolved>(Pipeline<'a, (), (), S, (), M>)
where
    M: Mode<'a, S, ()>;

struct A;

trait Noop {
    fn noop(self);
}

#[transitions(error = ())]
impl<'a> Noop for Author<'a, A> {
    fn noop(self) {}
}

fn main() {}

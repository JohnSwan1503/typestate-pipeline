#[path = "shared.rs"]
mod shared;

use typestate_pipeline::{InFlight, Pipelined, Resolved};

use shared::{Author, Client, DummyError, Started};

pub fn main() {
    fn assert_pipelined<'a, T>()
    where
        T: Pipelined<'a, Ctx = Client, Error = DummyError, Tag = ()>,
    {
    }
    assert_pipelined::<Author<'_, Started, Resolved>>();
    assert_pipelined::<Author<'_, Started, InFlight>>();
}

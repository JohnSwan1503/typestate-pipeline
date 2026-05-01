use super::{Author, Finished, Started};
use typestate_pipeline::{InFlight, Pipelined, Resolved};

pub fn main() {
    // Resolved<NS> projects to NS-stated Resolved-mode carrier.
    fn assert_resolved_projection<'a, A>()
    where
        A: Pipelined<'a>,
        A::Resolved<Finished>: Pipelined<'a, State = Finished, Mode = Resolved>,
    {
    }
    assert_resolved_projection::<Author<'_, Started, Resolved>>();

    // InFlight<NS> projects similarly.
    fn assert_inflight_projection<'a, A>()
    where
        A: Pipelined<'a>,
        A::Ctx: Sync,
        A::Error: Send,
        A::InFlight<Finished>: Pipelined<'a, State = Finished, Mode = InFlight>,
    {
    }
    assert_inflight_projection::<Author<'_, Started, Resolved>>();
}

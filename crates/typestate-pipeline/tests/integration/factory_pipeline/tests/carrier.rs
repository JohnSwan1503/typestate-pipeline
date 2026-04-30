use typestate_pipeline::{
    InFlight, Pipeline, Resolved, TypestateFactory, pipelined, transitions,
};

use super::domain::Server;
use super::error::AppError;

pipelined!(pub Author, ctx = Server, error = AppError);

impl<'a, S: 'a> Author<'a, S, Resolved> {
    pub fn state(&self) -> &S {
        self.0.state()
    }

    pub fn into_state(self) -> S {
        self.0.into_state()
    }
}

// ---------------------------------------------------------------------------
// `DatasetData` — Pipeline-integrated bag. Lives next to `Author` so
// the derive's `pipeline(carrier = Author)` arm has direct access to
// the carrier's tuple-struct internals.
// ---------------------------------------------------------------------------

#[derive(Debug, TypestateFactory)]
#[factory(error = AppError, pipeline(carrier = Author))]
pub struct DatasetData {
    #[field(required)]
    pub name: String,
    #[field(required, removable)]
    pub network: String,
    #[field(default = 4, overridable)]
    pub parallelism: u16,
    #[field(required, setter = trim_label, fallible)]
    pub label: String,
}

pub fn trim_label(value: String) -> Result<String, AppError> {
    let trimmed = value.trim().to_owned();
    if trimmed.is_empty() {
        Err(AppError::Empty("label"))
    } else {
        Ok(trimmed)
    }
}

// ---------------------------------------------------------------------------
// Out-of-bag transition: finalize the bag and move to a follow-on
// phase. The transition is bounded on the *exact* flag tuple
// `(Yes, Yes, No, Yes)` so `parallelism` flows through the default
// branch. Tests that need the override path finalize directly.
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct Deployed {
    pub dataset: DatasetData,
    pub job_id: u64,
}

#[transitions(error = AppError)]
impl<'a>
    Author<
        'a,
        DatasetDataFactory<
            typestate_pipeline::Yes, // name
            typestate_pipeline::Yes, // network
            typestate_pipeline::No,  // parallelism — defaulted at finalize
            typestate_pipeline::Yes, // label
        >,
    >
{
    #[transition(into = Deployed)]
    pub async fn deploy(
        state: DatasetDataFactory<
            typestate_pipeline::Yes,
            typestate_pipeline::Yes,
            typestate_pipeline::No,
            typestate_pipeline::Yes,
        >,
        ctx: &Server,
    ) -> Result<Deployed, AppError> {
        let dataset = state.finalize();
        Ok(Deployed {
            dataset,
            job_id: ctx.allocate_id(),
        })
    }
}

// ---------------------------------------------------------------------------
// Test helpers — wrap the (private-elsewhere) Author construction.
// ---------------------------------------------------------------------------

pub fn empty_bag<'a>(server: &'a Server) -> Author<'a, DatasetDataFactory, Resolved> {
    Author(Pipeline::resolved(server, DatasetDataFactory::new()))
}

/// Hand-construct an `InFlight` carrier whose pending future yields a
/// fresh empty bag — used by the InFlight setter-chain test, since
/// none of this suite's transitions naturally open an InFlight chain.
pub fn empty_inflight_bag<'a>(server: &'a Server) -> Author<'a, DatasetDataFactory, InFlight> {
    Author(Pipeline::in_flight(
        server,
        Box::pin(async { Ok(DatasetDataFactory::new()) }),
    ))
}

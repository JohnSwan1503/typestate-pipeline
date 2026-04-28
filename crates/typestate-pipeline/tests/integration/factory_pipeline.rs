//! Pipeline-integrated TypestateFactory — `#[factory(pipeline(carrier = …))]`.
//!
//! With this attribute, the macro emits Resolved + InFlight method pairs on
//! the user's carrier (`Author<'a, Bag<…>, M>`) for every standalone
//! transition: setters, default helpers, removers (`drop_<field>`), and
//! overriders (`override_<field>`). The bag's transitions become Pipeline
//! transitions automatically — no `#[transitions]` boilerplate required for
//! field-level moves.
//!
//! A separate `#[transitions]` impl can still be written for *cross-bag*
//! transitions (e.g. finalize the bag and advance to a different phase).
//! The two macro families compose orthogonally.

use std::{
    fmt,
    sync::atomic::{AtomicU64, Ordering},
};

use typestate_pipeline::{
    pipelined, transitions, InFlight, Pipeline, Resolved, TypestateFactory,
};

// ---------------------------------------------------------------------------
// Domain
// ---------------------------------------------------------------------------

#[derive(Debug, Default)]
struct Server {
    next_id: AtomicU64,
}

impl Server {
    fn allocate_id(&self) -> u64 {
        self.next_id.fetch_add(1, Ordering::SeqCst) + 1
    }
}

#[derive(Debug)]
enum AppError {
    Empty(&'static str),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Empty(field) => write!(f, "{field} is empty"),
        }
    }
}
impl std::error::Error for AppError {}

// ---------------------------------------------------------------------------
// Bag with Pipeline integration enabled.
// ---------------------------------------------------------------------------

#[derive(Debug, TypestateFactory)]
#[factory(error = AppError, pipeline(carrier = Author))]
struct DatasetData {
    #[field(required)]
    name: String,
    #[field(required, removable)]
    network: String,
    #[field(default = 4, overridable)]
    parallelism: u16,
    #[field(required, setter = trim_label, fallible)]
    label: String,
}

fn trim_label(value: String) -> Result<String, AppError> {
    let trimmed = value.trim().to_owned();
    if trimmed.is_empty() {
        Err(AppError::Empty("label"))
    } else {
        Ok(trimmed)
    }
}

// ---------------------------------------------------------------------------
// Carrier
// ---------------------------------------------------------------------------

pipelined!(Author, ctx = Server, error = AppError);

impl<'a, S: 'a> Author<'a, S, Resolved> {
    fn state(&self) -> &S {
        self.0.state()
    }
}

// ---------------------------------------------------------------------------
// Out-of-bag transition: finalize and move to a follow-on Phase.
// ---------------------------------------------------------------------------

#[derive(Debug)]
struct Deployed {
    dataset: DatasetData,
    job_id: u64,
}

// The bag is `DatasetDataFactory<…>`. Once every required field is `Yes` (and
// optional fields are either `Yes` or have defaults), we can finalize.
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
// Tests
// ---------------------------------------------------------------------------

fn empty_bag<'a>(server: &'a Server) -> Author<'a, DatasetDataFactory, Resolved> {
    Author(Pipeline::resolved(server, DatasetDataFactory::new()))
}

#[tokio::test]
async fn pipeline_setters_chain_in_resolved_mode() {
    let server = Server::default();
    let pipeline = empty_bag(&server);

    // All setters are direct methods on the Author carrier — no
    // .into_future() in the middle, all sync.
    let bag_full = pipeline
        .name("ds-a".to_owned())
        .network("eth".to_owned())
        // skip parallelism — let it default at finalize
        .label("  primary  ".to_owned()) // fallible — returns Result<Author, AppError>
        .expect("label trim should succeed");

    let deployed = bag_full.deploy().await.expect("deploy");
    assert_eq!(deployed.state().dataset.name, "ds-a");
    assert_eq!(deployed.state().dataset.parallelism, 4); // default
    assert_eq!(deployed.state().dataset.label, "primary"); // trimmed
    assert_eq!(deployed.state().job_id, 1);
}

#[tokio::test]
async fn pipeline_setters_chain_through_inflight() {
    let server = Server::default();

    // No transition in this carrier opens an InFlight chain on its own
    // (the only async transition is `deploy`, which is terminal). Construct
    // an InFlight pipeline by hand so we can exercise the InFlight arms of
    // the bag's setters.
    let initial: Author<DatasetDataFactory, InFlight> = Author(Pipeline::in_flight(
        &server,
        Box::pin(async { Ok(DatasetDataFactory::new()) }),
    ));

    // On InFlight, the fallible setter folds its Result into the chained
    // pending — no Result at the call site, just a chained InFlight.
    let chain = initial
        .name("ds-b".to_owned())
        .network("eth".to_owned())
        .label("  alpha  ".to_owned()); // InFlight — Result is folded

    // Awaiting the InFlight chain hands back a Resolved bag.
    let resolved: Author<_, Resolved> = chain.await.expect("chain");

    let deployed = resolved.deploy().await.expect("deploy");
    assert_eq!(deployed.state().dataset.label, "alpha");
}

#[tokio::test]
async fn pipeline_drop_field_transitions_yes_to_no() {
    let server = Server::default();
    let pipeline = empty_bag(&server);

    // Set network, then drop it, then set it again — exercises the bag's
    // drop_<field> as a Pipeline transition.
    let pipeline = pipeline
        .name("ds-c".to_owned())
        .network("eth".to_owned())
        .drop_network()
        .network("op".to_owned())
        .label("primary".to_owned())
        .expect("label");

    let deployed = pipeline.deploy().await.expect("deploy");
    assert_eq!(deployed.state().dataset.network, "op");
}

#[tokio::test]
async fn pipeline_override_replaces_value() {
    let server = Server::default();
    let pipeline = empty_bag(&server);

    let pipeline = pipeline
        .name("ds-d".to_owned())
        .network("eth".to_owned())
        .with_parallelism(2)
        .override_parallelism(8); // replaces 2 with 8, stays in Yes

    // Override flipped parallelism's flag to Yes, but our deploy transition
    // is bounded on parallelism flag = No (uses default). To test override,
    // route through finalize directly. (The deploy transition's strict bound
    // on the flag is intentional — it shows you can declare exactly which
    // bag shapes a transition operates on.)
    let pipeline = pipeline
        .label("primary".to_owned())
        .expect("label");

    // Drop parallelism back to No (it has #[field(default = 4, overridable)]
    // — but is not removable, so we can't drop it). Instead, just finalize
    // directly to verify the override took effect.
    let dataset = pipeline.0.into_state().finalize();
    assert_eq!(dataset.parallelism, 8);
}

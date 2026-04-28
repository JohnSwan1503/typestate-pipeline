//! End-to-end demo of the dataset-authoring pipeline rebuilt on top of
//! `typestate-pipeline`. Mirrors the four example flows from the upstream
//! `amp-client-admin` crate's `examples/` directory, but the whole pipeline
//! is implemented in roughly two hundred lines of code (see
//! `src/dataset_authoring/`) instead of the ~5 modules / dozens of phase
//! impls / two-mode hand-rolled method pairs the upstream version uses.
//!
//! Each scenario below is a direct rewrite of one of the upstream examples;
//! comments call out the divergences (mostly cosmetic) where they exist.
//!
//! This example was built by Claude. Be nice!

use typestate_pipeline::dataset_authoring::{
    client::Client,
    primitives::{Name, Namespace, Reference, Version},
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::default();

    new_evm_rpc(&client).await?;
    new_derived(&client).await?;
    edit_existing_evm_rpc(&client).await?;
    edit_existing_derived(&client).await?;
    inspect_along_the_way(&client).await?;

    Ok(())
}

// ---------------------------------------------------------------------------
// Scenario 1 — author a fresh EVM-RPC dataset (raw kind).
//
// Upstream equivalent: `examples/new_raw_dataset_authoring.rs`. Single
// terminal `.await?` collapses the whole register → tag → deploy chain.
// ---------------------------------------------------------------------------

async fn new_evm_rpc(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let deployed = client
        .author()
        .new_evm_rpc("my_namespace".parse()?, "my_dataset".parse()?)
        .into_builder()
        .with_finalized_blocks_only(true)
        .with_start_block(42)
        .with_network("mainnet".parse()?)
        .with_default_tables()
        .register()
        .tag_version(Version::new(0, 1, 0))
        .with_verify(true)
        .with_parallelism(2)
        .deploy()
        .await?;

    println!("[new_evm_rpc] deployed {}", deployed.job_id());
    Ok(())
}

// ---------------------------------------------------------------------------
// Scenario 2 — author a fresh derived dataset (SQL-defined kind).
//
// Upstream equivalent: `examples/new_derived_dataset_authoring.rs`.
// ---------------------------------------------------------------------------

async fn new_derived(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let deployed = client
        .author()
        .new_derived("my_namespace".parse()?, "my_derived".parse()?)
        .into_builder()
        .add_dependency("alias".into(), "dep".into())
        .add_table("table_a".parse()?, "SELECT 1".parse()?)
        .register()
        .tag_version(Version::new(0, 1, 0))
        .with_verify(true)
        .with_parallelism(2)
        .deploy()
        .await?;

    println!("[new_derived] deployed {}", deployed.job_id());
    Ok(())
}

// ---------------------------------------------------------------------------
// Scenario 3 — edit an existing EVM-RPC dataset (re-register + bump_patch).
//
// Upstream equivalent: `examples/existing_raw_dataset_authoring.rs`. The
// upstream example uses `override_network`/`with_start_block` after the
// edit-existing fetch; here the fetched-empty bag means we just call
// `with_network`/`with_start_block` to set them for the first time. The
// shape of the chain is otherwise identical.
// ---------------------------------------------------------------------------

async fn edit_existing_evm_rpc(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    // Seed the in-memory client so `edit_existing_*` finds something.
    seed(
        client,
        "my_namespace",
        "my_dataset",
        "evm_rpc",
        Version::new(0, 1, 0),
    )
    .await;

    let reference = Reference {
        namespace: "my_namespace".parse()?,
        name: "my_dataset".parse()?,
        version: Version::new(0, 1, 0),
    };

    let deployed = client
        .author()
        .edit_existing_evm_rpc(&reference)
        .into_builder()
        .with_network("testnet".parse()?)
        .with_start_block(0)
        .with_finalized_blocks_only(false)
        .with_default_tables()
        .register()
        .bump_patch()
        // `with_worker` takes a `Name` and the bag's `setter = wrap_some`
        // transformer lifts it into the underlying `Option<Name>` storage.
        .with_worker("worker-2".parse()?)
        .with_verify(true)
        .with_parallelism(4)
        .deploy()
        .await?;

    println!("[edit_existing_evm_rpc] deployed {}", deployed.job_id());
    Ok(())
}

// ---------------------------------------------------------------------------
// Scenario 4 — edit an existing derived dataset (re-register + bump_patch).
//
// Upstream equivalent: `examples/existing_derived_dataset_authoring.rs`.
// ---------------------------------------------------------------------------

async fn edit_existing_derived(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    seed(
        client,
        "my_namespace",
        "transactions_opt",
        "derived",
        Version::new(0, 1, 0),
    )
    .await;

    let reference = Reference {
        namespace: "my_namespace".parse()?,
        name: "transactions_opt".parse()?,
        version: Version::new(0, 1, 0),
    };

    let deployed = client
        .author()
        .edit_existing_derived(&reference)
        .into_builder()
        .add_table("transactions_deadbeef".parse()?, "SELECT *".parse()?)
        .register()
        .bump_patch()
        .with_parallelism(10)
        .deploy()
        .await?;

    println!("[edit_existing_derived] deployed {}", deployed.job_id());
    Ok(())
}

// ---------------------------------------------------------------------------
// Scenario 5 — pause mid-pipeline to inspect via carrier-arm getters.
//
// Not a port of any upstream amp example. Shows off two features in
// combination:
//
//   1. The auto-generated pipeline-arm getters that
//      `TypestateFactory`'s `pipeline(carrier = …)` emits:
//      `pub fn <field>(&self) -> &T` on the user's carrier
//      (Resolved-only), with the same per-field bounds as the standalone
//      bag's getter.
//   2. The `.inspect(|c| …)` combinator emitted by `pipelined!` /
//      `impl_pipelined!`: a chain-preserving "tap" that hands the
//      closure a `&Self` (Resolved arm) or a `&Carrier<S, Resolved>`
//      (InFlight arm — closure runs *after* the pending future
//      resolves). Either way the chain continues unchanged in the same
//      mode, so a print-statement peek doesn't break the fluent shape.
//
// Two pause shapes are exercised side-by-side, both via `.inspect(...)`:
//
// - **InFlight pause** straight after the async `tag_version(...)`
//   transition. No `.await?` is needed at the inspection point — the
//   InFlight `inspect` defers the closure into the chain's pending
//   future, so the print fires once the chain is finally awaited.
// - **Resolved pause** after the sync `with_verify` / `with_parallelism`
//   chain. Sync transitions preserve mode, so the chain is still
//   Resolved and the closure runs synchronously at that point.
//
// The whole flow folds into a single terminal `.await?`.
// ---------------------------------------------------------------------------

async fn inspect_along_the_way(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let deployed = client
        .author()
        .new_evm_rpc("inspect_ns".parse()?, "inspect_ds".parse()?)
        .into_builder()
        .with_finalized_blocks_only(true)
        .with_start_block(0)
        .with_network("mainnet".parse()?)
        .with_default_tables()
        .register()
        .tag_version(Version::new(1, 0, 0))
        // InFlight `inspect`: deferred until the pending future resolves.
        // The closure receives a `&Author<…, JobConfig<No, No, No>, Resolved>`
        // — internal getters (namespace / name / manifest_hash / version)
        // are unconditionally callable here; deploy-param getters are
        // still gated and unreachable.
        .inspect(|author| {
            println!(
                "[inspect] just tagged: {}/{} @ {} (manifest = {})",
                author.namespace(),
                author.name(),
                author.version(),
                author.manifest_hash(),
            );
        })
        .with_verify(true)
        .with_parallelism(4)
        // Resolved `inspect`: at this point the InFlight inspect above
        // has already deferred its closure into the pending future, but
        // the chain is *still* InFlight (sync transitions preserve mode
        // for both arms), so this `inspect` is also the InFlight arm —
        // it defers a second closure. Both fire in declaration order
        // when the chain is awaited. The closure now sees `parallelism`
        // and `verify` flipped to `Yes`, so their gated getters are
        // callable.
        .inspect(|author| {
            println!(
                "[inspect] configured: parallelism={}, verify={}",
                author.parallelism(),
                author.verify(),
            );
        })
        .deploy()
        .await?;

    println!("[inspect] deployed {}", deployed.job_id());
    Ok(())
}

// ---------------------------------------------------------------------------
// Test helper: pre-populate the in-memory client so the `edit_existing_*`
// fetch has something to find.
// ---------------------------------------------------------------------------

async fn seed(client: &Client, namespace: &str, name: &str, kind: &str, version: Version) {
    let manifest_body = serde_json::value::RawValue::from_string("{}".to_owned()).unwrap();
    let namespace: Namespace = namespace.parse().unwrap();
    let name: Name = name.parse().unwrap();
    // The mock client's `register` takes a static-str kind — pass the right
    // tag here so `edit_existing_*` accepts the kind on its way back out.
    let _hash = match kind {
        "evm_rpc" => {
            client
                .register(namespace.clone(), name.clone(), manifest_body, "evm_rpc")
                .await
        }
        "derived" => {
            client
                .register(namespace.clone(), name.clone(), manifest_body, "derived")
                .await
        }
        _ => unreachable!(),
    };
    client.tag(namespace, name, version).await;
}

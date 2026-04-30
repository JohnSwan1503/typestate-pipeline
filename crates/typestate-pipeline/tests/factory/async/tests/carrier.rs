use typestate_pipeline::{Pipeline, Resolved, TypestateFactory, pipelined, transitions};

use super::error::BadInput;

#[derive(Debug, Default)]
pub struct Hub;

pipelined!(pub Author, ctx = Hub, error = BadInput);

impl<'a, S: 'a> Author<'a, S, Resolved> {
    pub fn state(&self) -> &S {
        self.0.state()
    }

    pub fn into_state(self) -> S {
        self.0.into_state()
    }
}

// ---------------------------------------------------------------------------
// Order — Pipeline-integrated bag. Lives alongside `Author` so the
// derive's `pipeline(carrier = Author)` arm has direct access to the
// carrier's internals.
// ---------------------------------------------------------------------------

#[derive(TypestateFactory)]
#[factory(error = BadInput, pipeline(carrier = Author))]
pub struct Order {
    #[field(required, setter = trim_sku_async, async_fn)]
    pub sku: String,
    #[field(required, setter = parse_quantity_async, async_fn, fallible)]
    pub quantity: u32,
}

pub async fn trim_sku_async(value: String) -> String {
    tokio::task::yield_now().await;
    value.trim().to_owned()
}

pub async fn parse_quantity_async(value: u32) -> Result<u32, BadInput> {
    tokio::task::yield_now().await;
    if value == 0 {
        Err(BadInput::Empty)
    } else {
        Ok(value)
    }
}

// ---------------------------------------------------------------------------
// `Booked` phase + transition body that calls `finalize()` mid-chain.
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct Booked {
    pub sku: String,
    pub quantity: u32,
    pub receipt_id: u64,
}

#[transitions(error = BadInput)]
impl<'a> Author<'a, OrderFactory<typestate_pipeline::Yes, typestate_pipeline::Yes>> {
    #[transition(into = Booked)]
    pub async fn book(
        state: OrderFactory<typestate_pipeline::Yes, typestate_pipeline::Yes>,
        ctx: &Hub,
    ) -> Result<Booked, BadInput> {
        let order = state.finalize();
        let _ = ctx;
        Ok(Booked {
            sku: order.sku,
            quantity: order.quantity,
            receipt_id: 1234,
        })
    }
}

// ---------------------------------------------------------------------------
// Helper — wrap the (private-elsewhere) Author construction.
// ---------------------------------------------------------------------------

pub fn empty_order<'a>(hub: &'a Hub) -> Author<'a, OrderFactory, Resolved> {
    Author(Pipeline::resolved(hub, OrderFactory::new()))
}

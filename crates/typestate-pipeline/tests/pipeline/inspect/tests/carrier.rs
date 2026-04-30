use std::sync::atomic::{AtomicU64, Ordering};

use typestate_pipeline::{Pipeline, Resolved, pipelined, transitions};

use super::error::AppError;
use super::phases::{Deployed, Drafted, Tagged};

#[derive(Debug, Default)]
pub struct Hub {
    pub next_id: AtomicU64,
}

impl Hub {
    pub fn allocate(&self) -> u64 {
        self.next_id.fetch_add(1, Ordering::SeqCst) + 1
    }
}

pipelined!(pub Author, ctx = Hub, error = AppError);

impl<'a, S: 'a> Author<'a, S, Resolved> {
    pub fn state(&self) -> &S {
        self.0.state()
    }

    pub fn into_state(self) -> S {
        self.0.into_state()
    }
}

#[transitions]
impl<'a> Author<'a, Drafted> {
    /// Async deferred — produces an InFlight chain.
    #[transition(into = Tagged)]
    pub async fn tag(state: Drafted, ctx: &Hub) -> Result<Tagged, AppError> {
        Ok(Tagged {
            name: state.name,
            tag: ctx.allocate(),
        })
    }
}

#[transitions]
impl<'a> Author<'a, Tagged> {
    #[transition(into = Deployed)]
    pub async fn deploy(state: Tagged, ctx: &Hub) -> Result<Deployed, AppError> {
        Ok(Deployed {
            name: state.name,
            tag: state.tag,
            job_id: ctx.allocate(),
        })
    }
}

// ---------------------------------------------------------------------------
// Helper — wrap the (private-elsewhere) Author construction.
// ---------------------------------------------------------------------------

pub fn drafted<'a>(hub: &'a Hub, name: &str) -> Author<'a, Drafted, Resolved> {
    Author(Pipeline::resolved(
        hub,
        Drafted {
            name: name.to_owned(),
        },
    ))
}

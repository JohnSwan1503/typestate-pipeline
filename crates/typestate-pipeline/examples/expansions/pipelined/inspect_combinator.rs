use core::fmt;

use typestate_pipeline::{Pipeline, Resolved, pipelined};

#[derive(Debug, Default)]
struct Hub;

#[derive(Debug)]
struct AppError;
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("err")
    }
}
impl std::error::Error for AppError {}

pipelined!(Author, ctx = Hub, error = AppError);

#[derive(Debug)]
struct Phase1 {
    label: String,
}

fn main() {
    let hub = Hub;
    let carrier: Author<Phase1, Resolved> = Author(Pipeline::resolved(
        &hub,
        Phase1 {
            label: "first".to_owned(),
        },
    ));

    // Resolved-side inspect: closure observes the carrier; carrier is
    // returned unchanged so the chain continues.
    let carrier = carrier.inspect(|c| {
        println!("inspecting {}", c.0.state().label);
    });

    let _ = carrier.0.into_state();
}

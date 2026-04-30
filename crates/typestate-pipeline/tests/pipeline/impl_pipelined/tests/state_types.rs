// Phase-state markers + the carrier-tag type. None carry data —
// they only exist as type parameters.

#[derive(Debug, Clone)]
pub struct Started;

#[derive(Debug, Clone)]
pub struct Finished;

pub struct MyTag;

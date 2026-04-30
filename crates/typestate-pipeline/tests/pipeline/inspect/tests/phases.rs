// Phase-state types the carrier walks through.

#[derive(Debug, Clone)]
pub struct Drafted {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Tagged {
    pub name: String,
    pub tag: u64,
}

#[derive(Debug, Clone)]
pub struct Deployed {
    pub name: String,
    pub tag: u64,
    pub job_id: u64,
}

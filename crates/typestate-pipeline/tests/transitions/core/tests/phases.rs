// Phase-state types the chain walks through.

#[derive(Debug, Clone)]
pub struct Registered {
    pub name: String,
    pub manifest_hash: u64,
}

#[derive(Debug, Clone)]
pub struct Versioned {
    pub name: String,
    pub manifest_hash: u64,
    pub version: u32,
}

#[derive(Debug, Clone)]
pub struct JobConfigured {
    pub name: String,
    pub version: u32,
    pub parallelism: u16,
    pub verified: bool,
}

#[derive(Debug, Clone)]
pub struct Deployed {
    pub name: String,
    pub version: u32,
    pub job_id: u64,
}

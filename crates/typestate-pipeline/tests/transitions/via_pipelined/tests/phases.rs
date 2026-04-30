// Phase-state types for the carrier's chain.

#[derive(Debug, Clone)]
pub struct Drafted {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Versioned {
    pub name: String,
    pub version: u32,
}

#[derive(Debug, Clone)]
pub struct Published {
    pub name: String,
    pub version: u32,
}

// Node config placeholder: where the mesh + chain settings live.
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub name: String,
    pub cluster: String, // "devnet" for now
}

pub fn load() -> Config {
    Config { name: "node".into(), cluster: "devnet".into() }
}

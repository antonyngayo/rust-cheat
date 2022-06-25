use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub config_path: String,
    pub editor_path: Option<PathBuf>,
}

impl Config {
    pub fn new() -> Self{
        Self { config_path: Default::default(), editor_path: None }
    }
}
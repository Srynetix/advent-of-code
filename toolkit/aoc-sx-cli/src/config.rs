//! Config.

use color_eyre::Result;
use figment::{providers::Env, Figment};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub session_token: Option<String>,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Figment::new()
            .merge(Env::prefixed("AOC_"))
            .extract()
            .map_err(Into::into)
    }
}

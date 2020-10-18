use serde::Deserialize;
use anyhow::{Result, Error};
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
pub struct Baidu {
    pub addr: String,
    pub app_id: String,
    pub key: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub baidu: Option<Baidu>,
}

impl Config {
    pub fn from_file(f: impl AsRef<Path>) -> Result<Self> {
        let f = f.as_ref();
        let toml_str = std::fs::read_to_string(f)?;
        toml::from_str(toml_str.as_str()).map_err(Error::from)
    }
}

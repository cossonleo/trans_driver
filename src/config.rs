use anyhow::{Error, Result};
use serde::Deserialize;
use std::ffi::{OsStr, OsString};
use std::path::{Component, Path, PathBuf};

//#[serde(rename_all = "snake_case", tag = "type")]
//#[derive(Debug, Deserialize, Clone)]
//pub enum Url {
//    Http{ addr: String },
//    Https{ addr: String },
//}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Url {
    Http(String),
    Https(String),
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Baidu {
    pub url: Url,
    pub app_id: String,
    pub key: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    pub baidu: Option<Baidu>,
}

impl Config {
    pub fn from_file(f: impl AsRef<Path>) -> Result<Self> {
        let f = resolv_path(f);
        let toml_str = std::fs::read_to_string(f)?;
        toml::from_str(toml_str.as_str()).map_err(Error::from)
    }
}

fn resolv_env_var(v: impl AsRef<OsStr>) -> OsString {
    let v = v.as_ref();
    std::env::var_os(v).unwrap_or_else(|| {
        let mut e = OsString::from("$");
        e.push(v);
        e
    })
}

fn resolv_path(p: impl AsRef<Path>) -> PathBuf {
    let p = p.as_ref();
    let cs: Vec<String> = p
        .components()
        .map(|c| match c {
            Component::Normal(v) => v
                .to_str()
                .map(|vv| {
                    let mut vv = vv;
                    if vv == "~" {
                        vv = "$HOME"
                    }
                    if !vv.starts_with("$") {
                        return vv.into();
                    }

                    vv.get(1..vv.len())
                        .map(|e| resolv_env_var(e))
                        .unwrap_or_else(|| OsString::from("$"))
                })
                .unwrap_or(v.to_os_string())
                .to_string_lossy()
                .to_string(),
            Component::RootDir => "".to_string(),
            _ => c.as_os_str().to_string_lossy().to_string(),
        })
        .collect();
    cs.join("/").into()
}

#[test]
fn test_resolv_path() {
    println!("{}", resolv_path("/home/ahah").display());
    println!("{}", resolv_path("$HOME/ahah").display());
    println!("{}", resolv_path("~/ahah").display());
    println!("{}", resolv_path("///ahah/$afasdf/adfad").display());
}

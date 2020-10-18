use crate::api::*;
use crate::config;
use anyhow::{ensure, format_err, Result};
use async_trait::async_trait;
use md5::{Digest, Md5};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct TransResult {
    src: String,
    dst: String,
}

#[derive(Deserialize, Debug)]
struct Response {
    error_code: Option<u64>,
    from: String,
    to: String,
    trans_result: Vec<TransResult>,
}

pub struct Translator {
    app_id: String,
    key: String,
    addr: String,
}

impl Translator {
    pub fn new(conf: config::Baidu) -> Self {
        Self {
            app_id: conf.app_id,
            key: conf.key,
            addr: conf.addr,
        }
    }

    fn gen_salt(&self) -> String {
        chrono::Utc::now().timestamp_nanos().to_string()
    }

    fn get_md5(&self, salt: &str, text: &str) -> String {
        let sign = format!("{}{}{}{}", self.app_id, text, salt, self.key);
        let mut m5 = Md5::new();
        m5.update(sign);
        hex::encode(m5.finalize())
    }
}

#[async_trait]
impl Api for Translator {
    async fn translate(&self, from: &str, to: &str, text: &str) -> Result<String> {
        let salt = self.gen_salt();
        let sign = self.get_md5(salt.as_str(), text);
        let url = format!(
            "{}?q={}&from={}&to={}&appid={}&salt={}&sign={}",
            &self.addr, text, from, to, &self.app_id, salt, sign
        );

        let res: Response = surf::get(url)
            .recv_json()
            .await
            .map_err(|err| format_err!("request baidu: {}", err))?;
        ensure!(
            res.error_code.is_none(),
            format_err!("{}", res.error_code.unwrap())
        );
        Ok(res
            .trans_result
            .iter()
            .map(|v| format!("{}\n{}", v.src, v.dst))
            .collect::<Vec<String>>()
            .join("\n"))
    }
}

#[test]
fn test_baidu() {
    println!("{}", text);
}

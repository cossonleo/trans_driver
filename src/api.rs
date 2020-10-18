use async_trait::async_trait;

#[async_trait]
pub trait Api {
    async fn translate(&self, from: &str, to: &str, text: &str) -> anyhow::Result<String>;
}

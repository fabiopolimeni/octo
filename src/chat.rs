use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Chat {
    async fn prompt(&mut self, message: &str) -> Result<String>;
}

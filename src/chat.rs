use anyhow::Result;
use async_trait::async_trait;
use strum::Display;

#[derive(Display)]
pub enum Role {
    System,
    Assistant,
    User,
    Tool,
}

#[async_trait]
pub trait Chat {
    async fn message(&mut self, role: Role, message: &str) -> Result<String>;
}

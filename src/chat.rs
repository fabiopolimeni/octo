use anyhow::Result;
use async_trait::async_trait;
use std::fmt;

pub enum Role {
    System,
    Assistant,
    User,
    Tool,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Role::System => write!(f, "system"),
            Role::Assistant => write!(f, "assistant"),
            Role::User => write!(f, "user"),
            Role::Tool => write!(f, "tool"),
        }
    }
}

pub enum What {
    Start,
    Stop,
    Chunk,
    OutOfCharacters,
    ContentFilter,
    ToolCalls,
    Done,
}

impl fmt::Display for What {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            What::Start => write!(f, "start"),
            What::Stop => write!(f, "stopped"),
            What::Chunk => write!(f, "chunk"),
            What::OutOfCharacters => write!(f, "length"),
            What::ContentFilter => write!(f, "content_filter"),
            What::ToolCalls => write!(f, "tool_calls"),
            What::Done => write!(f, "done"),
        }
    }
}

#[async_trait]
pub trait Chat {
    async fn message(&mut self, role: Role, message: &str) -> Result<String>;
    async fn stream<F>(&mut self, role: Role, message: &str, f: F) -> Result<()>
    where
        F: Fn(&str, What) + Send;
}

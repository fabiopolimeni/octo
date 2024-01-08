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

pub enum StreamState {
    Start,
    Stop,
    Chunk,
    OutOfCharacters,
    ContentFilter,
    ToolCalls,
    Done,
}

impl fmt::Display for StreamState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            StreamState::Start => write!(f, "start"),
            StreamState::Stop => write!(f, "stop"),
            StreamState::Chunk => write!(f, "chunk"),
            StreamState::OutOfCharacters => write!(f, "length"),
            StreamState::ContentFilter => write!(f, "content_filter"),
            StreamState::ToolCalls => write!(f, "tool_calls"),
            StreamState::Done => write!(f, "done"),
        }
    }
}

#[async_trait]
pub trait Chat {
    async fn message(&mut self, role: Role, message: &str) -> Result<String>;
    async fn stream<F>(&mut self, role: Role, message: &str, f: F) -> Result<()>
    where
        F: Fn(&str, StreamState) + Send;
}

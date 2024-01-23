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
        match self {
            Role::System => write!(f, "system"),
            Role::Assistant => write!(f, "assistant"),
            Role::User => write!(f, "user"),
            Role::Tool => write!(f, "tool"),
        }
    }
}

pub enum State<'a> {
    Start,
    Stop,
    Message(&'a String),
    OutOfCharacters,
    ContentFilter,
    ToolCalls,
    Done,
}

impl<'a> fmt::Display for State<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            State::Start => write!(f, "start"),
            State::Stop => write!(f, "stop"),
            State::Message(msg) => write!(f, "message: {}", msg),
            State::OutOfCharacters => write!(f, "length"),
            State::ContentFilter => write!(f, "content_filter"),
            State::ToolCalls => write!(f, "tool_calls"),
            State::Done => write!(f, "done"),
        }
    }
}

#[async_trait]
pub trait Conversation {
    fn build(&mut self, role: Role, message: &str) -> &mut Self;
    async fn send<F>(&mut self, f: F) -> Result<()>
    where
        F: Fn(State) + Send;
}

#![allow(dead_code)]

use crate::chat::Chat;
use std::collections::HashMap;

use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;

use async_trait::async_trait;

pub struct OpenAI {
    client: Client,
    url: String,
    model: String,
}

impl OpenAI {
    pub fn new(url: &str, model: &str) -> Self {
        let client = Client::new();
        OpenAI {
            client,
            url: url.to_string(),
            model: model.to_string(),
        }
    }
}

#[derive(Deserialize, Debug)]
struct Logprob {
    token: String,
    logprob: f64,
    bytes: Vec<i64>,
}

#[derive(Deserialize, Debug)]
struct Content {
    token: String,
    logprob: f64,
    bytes: Vec<i64>,
    top_logprobs: Vec<Logprob>,
}

#[derive(Deserialize, Debug)]
struct Logprobs {
    content: Vec<Content>,
}

#[derive(Deserialize, Debug)]
struct Function {
    name: String,
    arguments: String,
}

#[derive(Deserialize, Debug)]
struct ToolCall {
    id: String,
    #[serde(rename = "type")]
    type_: String,
    function: Function,
}

#[derive(Deserialize, Debug)]
struct Choice {
    index: i64,
    message: Message,
    logprobs: Logprobs,
    finish_reason: String,
}

#[derive(Deserialize, Debug)]
struct Message {
    role: String,
    content: String,
    tool_calls: Vec<ToolCall>,
}

#[derive(Deserialize, Debug)]
struct Usage {
    prompt_tokens: i64,
    completion_tokens: i64,
    total_tokens: i64,
}

#[derive(Deserialize, Debug)]
struct ErrorObject {
    message: String,
    #[serde(rename = "type")]
    type_: String,
    param: Option<String>,
    code: String,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum Response {
    Error {
        error: ErrorObject,
    },
    Completion {
        id: String,
        object: String,
        created: i64,
        model: String,
        system_fingerprint: String,
        choices: Vec<Choice>,
        usage: Usage,
    },
}

#[async_trait]
impl Chat for OpenAI {
    async fn prompt(&mut self, message: &str) -> Result<String> {
        // This will POST a body of `{"message":"message","model":"model"}`
        let mut map = HashMap::new();
        map.insert("message", message);
        map.insert("model", &self.model);

        // Make POST request
        let response = self
            .client
            .post(self.url.clone())
            .header("Content-Type", "application/json")
            .header("Authorization", "Bearer $OPENAI_API_KEY")
            .json(&map)
            .send()
            .await?;

        // Return response body
        match response.json::<Response>().await? {
            Response::Error { error } => Err(anyhow::anyhow!(error.message)),
            Response::Completion { choices, .. } => Ok(choices[0].message.content.clone()),
        }
    }
}

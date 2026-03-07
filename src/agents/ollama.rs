use std::error::Error;

use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::{Agent, Message, Role};

#[derive(Serialize, Deserialize, Debug)]
struct OllamaRequest {
    model: String,
    messages: Vec<OllamaMessage>,
    stream: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct OllamaResponse {
    message: OllamaMessage,
}

#[derive(Serialize, Deserialize, Debug)]
struct OllamaMessage {
    role: OllamaRole,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
enum OllamaRole {
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "tool")]
    Tool,
}

pub struct OllamaAgent {
    client: Client,
    model: String,
}

impl OllamaAgent {
    pub fn new(model: String) -> Self {
        Self {
            client: Client::new(),
            model,
        }
    }
}

impl Agent for OllamaAgent {
    async fn generate(&self, messages: &[Message]) -> Result<Message, Box<dyn Error>> {
        let ollama_messages: Vec<OllamaMessage> =
            messages.iter().map(OllamaMessage::from).collect();

        let request_body = OllamaRequest {
            model: self.model.clone(),
            messages: ollama_messages,
            stream: false,
        };

        let response_text = self
            .client
            .post("http://localhost:11434/api/chat")
            .json(&request_body)
            .send()
            .await?;

        let response: OllamaResponse = response_text.json().await?;

        Ok(response.message.into())
    }
}

impl From<OllamaMessage> for Message {
    fn from(value: OllamaMessage) -> Self {
        Message {
            role: value.role.into(),
            text: value.content,
        }
    }
}

impl From<&Message> for OllamaMessage {
    fn from(value: &Message) -> Self {
        OllamaMessage {
            role: value.role.clone().into(),
            content: value.text.clone(),
        }
    }
}

impl From<OllamaRole> for Role {
    fn from(value: OllamaRole) -> Self {
        match value {
            OllamaRole::Assistant => Role::Model,
            OllamaRole::System => Role::System,
            OllamaRole::User => Role::User,
            _ => panic!("Right now we don't support tool role for Ollama"),
        }
    }
}

impl From<Role> for OllamaRole {
    fn from(value: Role) -> Self {
        match value {
            Role::System => OllamaRole::System,
            Role::User => OllamaRole::User,
            Role::Model => OllamaRole::Assistant,
        }
    }
}

use anyhow::{Context, Result, anyhow};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::agents::Llm;

use super::{Message, Role};

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

pub struct OllamaLlm {
    host: String,
    client: Client,
    model: String,
}

impl OllamaLlm {
    pub fn new(host: String, model: String) -> Self {
        Self {
            host,
            client: Client::new(),
            model,
        }
    }
}

impl Llm for OllamaLlm {
    async fn generate(&self, messages: &[Message]) -> Result<Message> {
        let ollama_messages: Vec<OllamaMessage> =
            messages.iter().map(OllamaMessage::from).collect();

        let request_body = OllamaRequest {
            model: self.model.clone(),
            messages: ollama_messages,
            stream: false,
        };

        let response_text = self
            .client
            .post(format!("http://{}/api/chat", self.host))
            .json(&request_body)
            .send()
            .await?;

        let response: OllamaResponse = response_text
            .json()
            .await
            .context("failed to parse Ollama response")?;

        response.message.try_into()
    }
}

impl TryFrom<OllamaMessage> for Message {
    type Error = anyhow::Error;

    fn try_from(value: OllamaMessage) -> Result<Self> {
        Ok(Message {
            role: value.role.try_into()?,
            text: value.content,
        })
    }
}

impl From<&Message> for OllamaMessage {
    fn from(value: &Message) -> Self {
        OllamaMessage {
            role: (&value.role).into(),
            content: value.text.clone(),
        }
    }
}

impl TryFrom<OllamaRole> for Role {
    type Error = anyhow::Error;

    fn try_from(value: OllamaRole) -> Result<Self> {
        match value {
            OllamaRole::Assistant => Ok(Role::Assistant),
            OllamaRole::System => Ok(Role::System),
            OllamaRole::User => Ok(Role::User),
            OllamaRole::Tool => Err(anyhow!("unsupported Ollama role: {:?}", value)),
        }
    }
}

impl From<&Role> for OllamaRole {
    fn from(value: &Role) -> Self {
        match value {
            Role::System => OllamaRole::System,
            Role::User => OllamaRole::User,
            Role::Assistant => OllamaRole::Assistant,
        }
    }
}

use anyhow::{Result, anyhow};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::agents::Llm;

use super::{Message, Role};

#[derive(Serialize, Deserialize, Debug, Clone)]
enum GeminiRole {
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "model")]
    Model,
}

impl From<&Role> for GeminiRole {
    fn from(role: &Role) -> Self {
        match role {
            Role::System => GeminiRole::System,
            Role::User => GeminiRole::User,
            Role::Assistant => GeminiRole::Model,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct GeminiRequest {
    contents: Vec<Content>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Candidate {
    content: Content,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Part {
    text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Content {
    role: GeminiRole,
    parts: Vec<Part>,
}

pub struct GeminiLlm {
    client: Client,
    api_key: String,
    model: String,
}

impl GeminiLlm {
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            model,
        }
    }
}

impl Llm for GeminiLlm {
    async fn generate(&self, messages: &[Message]) -> Result<Message> {
        let contents: Vec<Content> = messages.iter().map(Content::from).collect();

        let request_body = GeminiRequest { contents };

        let response_text = self
            .client
            .post(format!(
                "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent",
                &self.model
            ))
            .header("x-goog-api-key", &self.api_key)
            .json(&request_body)
            .send()
            .await?;

        let response: GeminiResponse = response_text.json().await?;

        let content = response
            .candidates
            .into_iter()
            .next()
            .ok_or_else(|| anyhow!("Gemini returned no candidates"))?
            .content;

        let text = content
            .parts
            .into_iter()
            .next()
            .ok_or_else(|| anyhow!("Gemini returned empty content"))?
            .text;

        Ok(Message {
            role: Role::Assistant,
            text,
        })
    }
}

impl From<&Message> for Content {
    fn from(value: &Message) -> Self {
        Content {
            role: GeminiRole::from(&value.role),
            parts: vec![Part {
                text: value.text.clone(),
            }],
        }
    }
}

use std::error::Error;

use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::{Agent, Message, Role};

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
    role: Role,
    parts: Vec<Part>,
}

pub struct GeminiAgent {
    client: Client,
    api_key: String,
    model: String,
}

impl GeminiAgent {
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            model,
        }
    }
}

impl Agent for GeminiAgent {
    async fn generate(&self, messages: &[Message]) -> Result<Message, Box<dyn Error>> {
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

        Ok(response.candidates.into_iter().next().unwrap().content.into())
    }
}

impl From<Content> for Message {
    fn from(value: Content) -> Self {
        Message {
            role: value.role,
            text: value.parts.into_iter().next().unwrap().text,
        }
    }
}

impl From<&Message> for Content {
    fn from(value: &Message) -> Self {
        Content {
            role: value.role.clone(),
            parts: vec![Part {
                text: value.text.clone(),
            }],
        }
    }
}

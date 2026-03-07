use std::error::Error;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub role: Role,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Role {
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "model")]
    Model,
}

pub enum Provider {
    Gemini(GeminiAgent),
    Ollama(OllamaAgent),
}

impl Agent for Provider {
    async fn generate(&self, messages: &[Message]) -> Result<Message, Box<dyn Error>> {
        match self {
            Provider::Gemini(a) => a.generate(messages).await,
            Provider::Ollama(a) => a.generate(messages).await,
        }
    }
}

pub struct Chat {
    history: Vec<Message>,
    agent: Provider,
}

impl Chat {
    pub fn new(agent: Provider) -> Self {
        Self {
            history: vec![],
            agent,
        }
    }

    pub async fn send(&mut self, text: &str) -> Result<String, Box<dyn Error>> {
        self.history.push(Message {
            role: Role::User,
            text: text.to_string(),
        });

        let response_message = self.agent.generate(&self.history).await?;
        let response_text = response_message.text.clone();

        self.history.push(response_message);

        Ok(response_text)
    }
}

pub trait Agent {
    async fn generate(&self, messages: &[Message]) -> Result<Message, Box<dyn Error>>;
}

mod gemini;
mod ollama;
pub use gemini::GeminiAgent;
pub use ollama::OllamaAgent;

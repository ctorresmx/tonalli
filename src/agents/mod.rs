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
    #[serde(rename = "assistant")]
    Assistant,
}

pub trait Llm {
    async fn generate(&self, messages: &[Message]) -> Result<Message, Box<dyn Error>>;
}

pub enum Agent {
    Gemini(GeminiLlm),
    Ollama(OllamaLlm),
}

impl Llm for Agent {
    async fn generate(&self, messages: &[Message]) -> Result<Message, Box<dyn Error>> {
        match self {
            Agent::Gemini(a) => a.generate(messages).await,
            Agent::Ollama(a) => a.generate(messages).await,
        }
    }
}

pub struct Chat {
    history: Vec<Message>,
    agent: Agent,
}

impl Chat {
    pub fn new(agent: Agent) -> Self {
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

mod gemini;
mod ollama;
pub use gemini::GeminiLlm;
pub use ollama::OllamaLlm;

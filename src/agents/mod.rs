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

pub struct Chat<A: Agent> {
    history: Vec<Message>,
    agent: A,
}

impl<A: Agent> Chat<A> {
    pub fn new(agent: A) -> Self {
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
pub use gemini::GeminiAgent;

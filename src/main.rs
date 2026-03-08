mod agents;
mod tui;

use std::error::Error;

use crate::agents::{Agent, Chat, GeminiLlm, OllamaLlm};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let api_key = std::env::var("AGENT_API_KEY").unwrap_or_default();
    let model = std::env::var("AGENT_MODEL").expect("AGENT_MODEL not set");
    let provider = std::env::var("AGENT_PROVIDER").expect("AGENT_PROVIDER not set");

    let agent = match provider.as_str() {
        "ollama" => Agent::Ollama(OllamaLlm::new(model)),
        "gemini" => Agent::Gemini(GeminiLlm::new(api_key, model)),
        _ => return Err(format!("Unknown provider: {}", provider).into()),
    };
    let chat = Chat::new(agent);
    tui::run(chat).await?;

    Ok(())
}

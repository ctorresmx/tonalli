mod agents;
mod tui;

use anyhow::{Context, Result, bail};

use crate::agents::{Agent, Chat, GeminiLlm, OllamaLlm};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let model = std::env::var("AGENT_MODEL").context("AGENT_MODEL not set")?;
    let provider = std::env::var("AGENT_PROVIDER").context("AGENT_PROVIDER not set")?;
    let ollama_host =
        std::env::var("AGENT_OLLAMA_HOST").unwrap_or_else(|_| "localhost:11434".to_string());

    let agent = match provider.as_str() {
        "ollama" => Agent::Ollama(OllamaLlm::new(ollama_host, model)),
        "gemini" => Agent::Gemini(GeminiLlm::new(
            std::env::var("AGENT_API_KEY").context("AGENT_API_KEY not set")?,
            model,
        )),
        _ => bail!("Unknown provider: {}", provider),
    };
    let chat = Chat::new(agent);
    tui::run(chat).await?;

    Ok(())
}

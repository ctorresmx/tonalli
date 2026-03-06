mod agents;
mod tui;

use std::error::Error;

use crate::agents::{Chat, GeminiAgent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let api_key = std::env::var("AGENT_API_KEY").expect("AGENT_API_KEY not set");
    let model = std::env::var("AGENT_MODEL").expect("AGENT_MODEL not set");

    let agent = GeminiAgent::new(api_key, model);
    let chat = Chat::new(agent);

    tui::run(chat).await?;

    Ok(())
}

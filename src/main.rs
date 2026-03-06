mod agents;

use std::error::Error;

use crate::agents::{Chat, GeminiAgent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let api_key = std::env::var("AGENT_API_KEY");
    let model = std::env::var("AGENT_MODEL");

    let agent = GeminiAgent::new(api_key.unwrap(), model.unwrap());

    let mut chat = Chat::new(agent);

    let response = chat.send("Hi there!").await?;

    println!("history: {:?}", response);

    let response2 = chat.send("Why is the sky blue? Be concise").await?;

    println!("history: {:?}", response2);

    Ok(())
}

mod error;
mod gemini;

use error::Result;
use gemini::GeminiClient;

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = GeminiClient::new()?;
    client.request().await;

    Ok(())
}

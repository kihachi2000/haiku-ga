mod error;

use dotenv::dotenv;
use std::env;

use error::{Error, Result};

fn main() -> Result<()> {
    let gemini_api_key = get_api_key("GEMINI_API_KEY")?;
    println!("{}", gemini_api_key);

    Ok(())
}

fn get_api_key(key: &str) -> Result<String> {
    match dotenv() {
        Ok(_) => env::var(key).map_err(|_| Error::VariableNotFound(key.to_string())),
        _ => Err(Error::DotEnvNotFound),
    }
}

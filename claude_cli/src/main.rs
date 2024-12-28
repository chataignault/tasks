use clap::Parser;
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use dirs;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::Deserialize;
use std::error::Error;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    message: String,
}

#[derive(Debug, Deserialize)]
struct Credentials {
    api_key: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // Load credentials from YAML file
    let credentials_folder = dirs::home_dir()
        .expect("Could not find home directory")
        .join(".credentials");
    let credentials_content = fs::read_to_string(credentials_folder.join("claude.yaml"))?;
    let credentials: Credentials = serde_yaml::from_str(&credentials_content)?;

    // Set up headers
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert("x-api-key", HeaderValue::from_str(&credentials.api_key)?);
    headers.insert(
        "anthropic-version",
        HeaderValue::from_str(&"2023-06-01".to_string())?,
    );

    // Prepare the request body
    let body = serde_json::json!({
        "model": "claude-3-5-sonnet-20240620",
        "max_tokens": 1000,
        "messages": [
            {
                "role": "user",
                "content": args.message
            }
        ]
    });

    // Send the request
    let client = reqwest::Client::new();
    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .headers(headers)
        .json(&body)
        .send()
        .await?;

    // Check if the request was successful
    if response.status().is_success() {
        let response_body: serde_json::Value = response.json().await?;
        if let Some(content) = response_body["content"][0]["text"].as_str() {
            let mut ctx = ClipboardContext::new().unwrap();
            ctx.set_contents(content.to_owned()).unwrap();
            println!("Claude's response:\n{}", content);
        } else {
            println!("Unexpected response format");
        }
    } else {
        println!("Error: {}", response.status());
        println!("Response body: {}", response.text().await?);
    }

    Ok(())
}

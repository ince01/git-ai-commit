use reqwest::Client;
use serde_json::json;
use std::env;

use crate::ai_prompt::get_ai_commit_message_prompt;

pub async fn generate_commit_message_with_gemini(
    diff: &str,
    debug: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    let api_key = env::var("GEMINI_API_KEY").map_err(
        |_| "🚫 GEMINI_API_KEY environment variable not set. Please provide an API key.",
    )?;

    let api_url =
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent";

    let prompt = get_ai_commit_message_prompt(diff);

    if debug {
        println!("🛠️ Debug: Commit Message Prompt 📜\n{}", prompt);
    }

    let client = Client::new();
    let payload = json!({
        "contents": [{
            "parts": [{"text": prompt}]
        }]
    });

    let response = client
        .post(format!("{}?key={}", api_url, api_key))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await?;
        return Err(format!(
            "❌ API request failed. Status: {}, Error: {}",
            status, error_text
        )
        .into());
    }

    let response_text = response.text().await?;

    if debug {
        println!("🛠️ Debug: Raw API Response 📡\n{}", response_text);
    }

    let response_json: serde_json::Value = serde_json::from_str(&response_text)?;

    let commit_msg = response_json
        .get("candidates")
        .and_then(|candidates| candidates.get(0))
        .and_then(|candidate| candidate.get("content"))
        .and_then(|content| content.get("parts"))
        .and_then(|parts| parts.get(0))
        .and_then(|part| part.get("text"))
        .and_then(|text| text.as_str())
        .ok_or("Failed to extract generated text from API response")?;

    let cleaned_msg = commit_msg
        .lines()
        .next()
        .unwrap_or(commit_msg)
        .trim()
        .trim_matches('"');

    Ok(cleaned_msg.to_string())
}

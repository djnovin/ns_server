use actix_web::Error;
use awc::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Deserialize)]
struct Choice {
    text: String,
}

#[derive(Serialize)]
struct GPT3Request {
    prompt: String,
    max_tokens: u32,
    temperature: f64,
    top_p: f64,
    frequency_penalty: f64,
    presence_penalty: f64,
    stop: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct GPT3Response {
    choices: Vec<Choice>,
}

pub async fn generate_message(prompt: String) -> Result<String, Error> {
    let openai_endpoint: String =
        env::var("OPEN_AI_ENDPOINT").expect("OPEN_AI_ENDPOINT must be set");
    let openai_token: String = env::var("OPEN_AI_TOKEN").expect("OPEN_AI_TOKEN must be set");

    let client = Client::default();

    let options = GPT3Request {
        prompt,
        max_tokens: 64,
        temperature: 0.7,
        top_p: 1.0,
        frequency_penalty: 0.0,
        presence_penalty: 0.0,
        stop: vec!["\n".to_string()],
    };

    let response: GPT3Response = client
        .post(openai_endpoint)
        .insert_header(("Authorization", format!("Bearer {}", openai_token)))
        .send_json(&options)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?
        .json()
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let message = response.choices[0].text.clone();

    Ok(message)
}

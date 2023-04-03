// main.rs

use anyhow::{anyhow, Result};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use shuttle_secrets::SecretStore;
use std::env;
use std::time::Duration;
use tokio::time::timeout;

mod extractor;
mod fetcher;
mod gpt_client;
mod simularity;

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    name: String,
    price: String,
    description: String,
    image_url: String,
}

#[derive(Deserialize)]
pub struct QueryParameters {
    url: String,
}

#[shuttle_runtime::main]
async fn axum(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> shuttle_axum::ShuttleAxum {
    let open_api_key = if let Some(secret) = secret_store.get("OPENAI_API_KEY") {
        secret
    } else {
        return Err(anyhow!("secret was not found").into());
    };

    let router = Router::new()
        .route("/", get(get_product_info))
        .with_state(open_api_key);

    Ok(router.into())
}

pub async fn get_product_info(
    Query(query_params): Query<QueryParameters>,
    State(open_api_key): State<String>,
) -> Result<Json<Product>, ChatGptError> {
    // Visit website and get their HTML body

    let url = query_params.url;

    println!("url: {url:#?}");
    println!("open_api_key: {open_api_key}");

    let body = timeout(Duration::from_secs(2), fetcher::fetch_html(&url))
        .await
        .map_err(|_| {
            ChatGptError::InvalidData("Fetching HTML took longer than 2 seconds".to_string())
        })?
        .map_err(ChatGptError::Reqwest)?;

    // Condense text and images from a website
    let (texts, images) = extractor::extract_text_and_images(&body, &url);

    // Formulate prompt from the text and images
    let prompt = gpt_client::create_chatgpt_prompt(
        &url,
        &texts.iter().map(|t| t).collect::<Vec<_>>(),
        &images,
        600,
        10,
    );

    println!("{prompt}");

    // Call chatGPT with the prompt
    let product = gpt_client::call_chat_gpt(&prompt, &open_api_key).await?;

    // Return product
    Ok(Json(product))
}

#[derive(Debug)]
pub enum ChatGptError {
    Env(env::VarError),
    Reqwest(reqwest::Error),
    InvalidData(String),
    SerdeJson(serde_json::error::Error),
}

impl From<env::VarError> for ChatGptError {
    fn from(err: env::VarError) -> ChatGptError {
        ChatGptError::Env(err)
    }
}

impl From<reqwest::Error> for ChatGptError {
    fn from(err: reqwest::Error) -> ChatGptError {
        ChatGptError::Reqwest(err)
    }
}

impl From<serde_json::error::Error> for ChatGptError {
    fn from(err: serde_json::error::Error) -> ChatGptError {
        ChatGptError::SerdeJson(err)
    }
}

impl IntoResponse for ChatGptError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            ChatGptError::Env(env::VarError::NotPresent) => (
                StatusCode::NOT_FOUND,
                "Environment variable not found".to_string(),
            ),
            ChatGptError::Env(env::VarError::NotUnicode(_)) => (
                StatusCode::BAD_REQUEST,
                "Environment variable contains invalid Unicode".to_string(),
            ),
            ChatGptError::Reqwest(reqwest_error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Reqwest error: {}", reqwest_error),
            ),
            ChatGptError::SerdeJson(serde_json_error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Serde JSON error: {}", serde_json_error),
            ),
            ChatGptError::InvalidData(invalid_data_error) => (
                StatusCode::BAD_REQUEST,
                format!("Invalid data error: {}", invalid_data_error),
            ),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

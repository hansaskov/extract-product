// gpt_client.rs

use reqwest::Client;

use crate::{ChatGptError, Product};

pub fn create_chatgpt_prompt(
    url: &str,
    texts: &[&String],
    images: &[String],
    max_words: usize,
    max_images: usize,
) -> String {
    let text_sample = texts
        .iter()
        .flat_map(|text| text.split_whitespace())
        .take(max_words)
        .collect::<Vec<&str>>()
        .join(" ");

    let image_sample = images
        .iter()
        .take(max_images)
        .cloned()
        .collect::<Vec<String>>()
        .join("\n");

    format!(
        r#"
You will help me find product information from this product website. From the provided data and website url extract the relevant values into the JSON schema. 
- Return one product
- Return the most likely product
- YOU ARE ONLY ALLOWED TO RESPOND USING JSON
- Return a short and precise name
- Give a description of the product using fewer than 10 words
- Return Unknown if there is not sufficient information
- Use an end token after writing JSON

- Schema 
    {{
        "name": "productName",
        "price": "productPrice",
        "description": "productDescription"
        "image_url": "productImage_url"
    }}

- Website
    {url}

- Data
    {text_sample} 

    {image_sample}
"#
    )
}

pub async fn call_chat_gpt(prompt: &str, api_key: &str) -> Result<Product, ChatGptError> {
   
    let data = create_request_data(prompt);
    let response = send_request(&api_key, &data).await?;
    parse_response(response).await
}

pub fn create_request_data(prompt: &str) -> serde_json::Value {
    serde_json::json!({
        "model": "gpt-3.5-turbo",
        "messages": [
            {
                "role": "user",
                "content": prompt,
            }
        ],
        "max_tokens": 150
    })
}

async fn send_request(
    api_key: &str,
    data: &serde_json::Value,
) -> Result<reqwest::Response, reqwest::Error> {
    let client = Client::new();
    let api_url = "https://api.openai.com/v1/chat/completions";
    client
        .post(api_url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(data)
        .send()
        .await
}

async fn parse_response(response: reqwest::Response) -> Result<Product, ChatGptError> {
    let json: serde_json::Value = response.json().await.map_err(ChatGptError::Reqwest)?;
    if let Some(completion_str) = json
        .get("choices")
        .and_then(|choices| choices.get(0))
        .and_then(|choice| choice.get("message"))
        .and_then(|message| message.get("content"))
        .and_then(|content| content.as_str())
    {
        println!("{completion_str}");
        return serde_json::from_str::<Product>(completion_str).map_err(ChatGptError::SerdeJson);
    }

    println!("{json:#?}");

    Err(ChatGptError::InvalidData(
        "Failed to extract product information".to_string(),
    ))
}

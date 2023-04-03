// fetcher.rs
use reqwest::Client;

// Takes an url as input and returns it as scraper HTML
pub async fn fetch_html(url: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let body = client.get(url).send().await?.text().await?;
    Ok(body)
}

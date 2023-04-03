// extractor.rs
use crate::simularity;
use reqwest::Url;
use scraper::{ElementRef, Html, Selector};

pub fn extract_text_and_images(resp: &String, url: &str) -> (Vec<String>, Vec<String>) {
    let html = Html::parse_document(&resp);
    let keywords = vec!["price", "product", "item", "name"];

    // 3. Use the ? operator to simplify error handling
    let texts = extract_texts(&html, &keywords);
    let images = extract_images(&html, url);

    (texts, images)
}

fn extract_texts(parsed_html: &Html, keywords: &[&str]) -> Vec<String> {
    let mut texts = Vec::new();

    // Extract text content from the elements
    let text_selectors = vec![
        Selector::parse("h1").unwrap(),
        Selector::parse("h2").unwrap(),
        Selector::parse("h3").unwrap(),
        Selector::parse("h4").unwrap(),
        Selector::parse("h5").unwrap(),
        Selector::parse("h6").unwrap(),
        Selector::parse("span").unwrap(),
        Selector::parse("p").unwrap(),
        // Selector::parse("div").unwrap(),
    ];

    for selector in text_selectors {
        for element in parsed_html.select(&selector) {
            let text = element.text().collect::<String>().trim().to_string();
            if !text.is_empty() {
                texts.push(text);
            }
        }
    }

    // Start the traversal from the root element of the parsed HTML document.
    for node in parsed_html.root_element().children() {
        if let Some(node_el) = ElementRef::wrap(node) {
            traverse_nodes(node_el, keywords, &mut texts);
        }
    }
    texts
}

fn extract_images(parsed_html: &Html, base_url: &str) -> Vec<String> {
    let mut images = Vec::new();
    let image_selector = Selector::parse("img").expect("Failed to create img selector");

    let base_url = Url::parse(base_url).expect("Failed to parse base URL");

    for element in parsed_html.select(&image_selector) {
        if let Some(src) = element.value().attr("src") {
            let mut full_url = src.to_string();

            if !src.starts_with("http://") && !src.starts_with("https://") {
                if let Ok(absolute_url) = base_url.join(src) {
                    full_url = absolute_url.to_string();
                }
            }

            images.push(full_url);
        }
    }

    images
}

// The `traverse_nodes` function is a helper function that performs a recursive traversal of the HTML elements.
fn traverse_nodes(node: ElementRef, keywords: &[&str], texts: &mut Vec<String>) {
    // Get the class attribute of the current element, or use an empty string if it's not present.
    let class_value = node.value().attr("class").unwrap_or("");

    // Check if the class is related to any of the provided keywords.
    if simularity::is_related(class_value, &keywords.to_vec()) {
        // Collect the text content of the element and trim it.
        let text = node.text().collect::<String>().trim().to_string();

        // Add the text to the output if it's not empty.
        if !text.is_empty() && text.len() < 75 {
            texts.push(text);
        }
    }

    // Recursively traverse the child elements of the current element.
    for child in node.children() {
        if let Some(child_el) = ElementRef::wrap(child) {
            traverse_nodes(child_el, keywords, texts);
        }
    }
}

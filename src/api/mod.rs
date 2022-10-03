use std::collections::HashMap;
use reqwest;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::{Deserialize};

#[derive(Deserialize)]
struct Memo {
    message: String,
}

pub fn send(token: &str, memo: &str) {
    let url = format!("https://flomoapp.com/iwh/NTk5OQ/{}/", token);

    let mut headers = HeaderMap::new();
    headers.append(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let mut body = HashMap::new();
    body.insert("content", memo);

    let client = reqwest::blocking::Client::new();

    let response = match client.post(url).json(&body).send() {
        Ok(response) => response,
        Err(_) => panic!("Failed to post the memo"),
    };

    let resp: Memo = match response.json() {
        Ok(content) => content,
        Err(_) => panic!("Failed to obtain response"),
    };

    println!("{}", resp.message);
}
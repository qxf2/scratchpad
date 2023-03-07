/*
A stubbed library for http methods
*/

use reqwest::{Client, Response};

pub async fn http_get(base_url: &str, endpoint: &str) -> Result<Response, reqwest::Error> {
    let client = Client::new();
    let response = client.get(format!("{}{}", base_url, endpoint)).send().await;
    return response;
}

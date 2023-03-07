/*
A stubbed library for http methods
*/

use reqwest::{Client, Response};

pub async fn http_get(endpoint: &str) -> Result<Response, reqwest::Error> {
    let base_url = "https://not-there.qxf2.com";
    let client = Client::new();
    let response = client.get(format!("{}{}", base_url, endpoint)).send().await;
    return response;
}

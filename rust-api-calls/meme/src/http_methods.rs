/*
A module for making HTTP methods using reqwest.
*/
use reqwest::{Client, Response};

pub async fn http_get(base_url: &str, endpoint: &str) -> Result<Response, reqwest::Error> {
    let client = Client::new();
    let response = client.get(format!("{}{}", base_url, endpoint)).send().await;
    return response;
}

#[cfg(test)]
mod tests{
    use super::*;

    #[tokio::test]
    async fn test_valid_http_get(){
        let base_url = "https://qxf2.com";
        let endpoint = "/qxf2-name";
        let response = match http_get(base_url, endpoint).await{
            Ok(_val) => true,
            Err(_err) => false
        };
        assert!(response)
    }

    #[tokio::test]
    async fn test_invalid_http_get(){
        let base_url = "https://blah.qxf2.com";
        let endpoint = "/this-does-not-exist";
        let response = match http_get(base_url, endpoint).await{
            Ok(_val) => true,
            Err(_err) => false
        };
        assert!(!response)        
    }


}
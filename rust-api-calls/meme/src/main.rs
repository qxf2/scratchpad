/*
Fetch the top 100 memes from https://imgflip.com/
Print out the id and the name of the meme
URL: https://api.imgflip.com/get_memes
*/
use std::error::Error;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let base_url = "https://api.imgflip.com";
    let endpoint = "/get_memes";
    let client = Client::new();
    let response = client
                    .get(format!("{}{}",base_url, endpoint))
                    .send()
                    .await?
                    .text()
                    .await;
    println!("{:?}", response);
    Ok(())
}

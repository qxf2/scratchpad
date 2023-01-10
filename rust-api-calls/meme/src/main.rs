/*
Fetch the top 100 memes from https://imgflip.com/
Print out the id and the name of the meme
URL: https://api.imgflip.com/get_memes
*/
use std::error::Error;
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Meme{
    id: String,
    name: String,
    url: String,
    width: i32,
    height: i32,
    captions: u8
}

#[derive(Deserialize, Debug)]
struct Memes{
    memes: Vec<Meme>
}

#[derive(Deserialize, Debug)]
struct GetMemes{
    success: bool,
    data: Memes
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let base_url = "https://api.imgflip.com";
    let endpoint = "/get_memes";
    let client = Client::new();
    let response = client
                    .get(format!("{}{}",base_url, endpoint))
                    .send()
                    .await?
                    .json::<GetMemes>()
                    .await;
    let all_memes = match response{
        Ok(val) => val.data,
        _ => panic!("Error! I did not get a JSON value from /get_memes")
    };
    for meme in all_memes.memes{
        println!("{:?}:{:?}",meme.name,meme.url);
    }
    Ok(())
}

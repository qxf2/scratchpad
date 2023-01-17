/*
Fetch the top 100 memes from https://imgflip.com/
Print out the id and the name of the meme
URL: https://api.imgflip.com/get_memes
*/
use reqwest::{Client, Response};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Meme {
    id: String,
    name: String,
    url: String,
}

#[derive(Deserialize, Debug)]
struct Memes {
    memes: Vec<Meme>,
}

#[derive(Deserialize, Debug)]
struct GetMemes {
    data: Memes,
}

async fn http_get(base_url: &str, endpoint: &str) -> Result<Response, reqwest::Error> {
    let client = Client::new();
    let response = client.get(format!("{}{}", base_url, endpoint)).send().await;
    return response;
}

fn get_empty_memes() -> Memes{
    Memes{memes: vec![]}
}

async fn parse_memes_json(value: Response) -> Memes{
    let get_memes = value.json::<GetMemes>().await;
    let memes = match get_memes{
        Ok(val) => val.data,
        Err(err) => {println!("Rust says {:?}", err);
                            get_empty_memes()}
    };

    return memes
}

async fn get_meme_data() -> Memes {
    let base_url = "https://api.imgflip.com";
    let endpoint = "/get_memes";
    let memes = match http_get(base_url, endpoint).await{
        Ok(val) => parse_memes_json(val).await,
        Err(err) => { println!("Rust says {:?}", err);
                            get_empty_memes()}
    };

    return memes;
}

#[tokio::main]
async fn main() {
    let all_memes = get_meme_data().await;
    if all_memes.memes.len() > 0{
        for meme in all_memes.memes {
            println!("{},{},{}", meme.name, meme.id, meme.url);
        }
    } else {
        println!("Error when fetching memes. Got zero memes!")
    }
}

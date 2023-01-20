/*
A collection of imgflip endpoints
*/
use crate::http_methods;
use crate::meme_type::{Memes, parse_memes_json, get_empty_memes};

pub async fn get_meme_data() -> Memes {
    let base_url = "https://api.imgflip.com";
    let endpoint = "/get_memes";
    let memes = match http_methods::http_get(base_url, endpoint).await{
        Ok(val) => parse_memes_json(val).await,
        Err(err) => { println!("Rust says {:?}", err);
                            get_empty_memes()}
    };

    return memes;
}

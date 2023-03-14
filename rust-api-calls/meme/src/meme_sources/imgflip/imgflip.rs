/*
A collection of imgflip endpoints
*/
#[cfg(not(test))]
use crate::http_methods;
#[cfg(test)]
use crate::stubbed_http_methods as http_methods;
use crate::meme_sources::imgflip::meme_type::{Memes, parse_memes_json, get_empty_memes};

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

#[cfg(test)]
mod tests{
    use super::*;

    #[tokio::test]
    async fn test_valid_get_memes(){
        let outcome = get_meme_data().await;
        assert_eq!(outcome.length(),100)
    }

    #[tokio::test]
    #[ignore]
    async fn test_invalid_get_memes(){
        let outcome = get_meme_data().await;
        assert_eq!(outcome.length(),0)
    }

}
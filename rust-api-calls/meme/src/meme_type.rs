/*
The meme struct and associated structs and methods
*/
use serde::Deserialize;
use reqwest::{Response};


#[derive(Deserialize, Debug)]
pub struct Meme {
    pub id: String,
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct Memes {
    pub memes: Vec<Meme>,
}

#[derive(Deserialize, Debug)]
pub struct GetMemes {
    pub data: Memes,
}

pub fn get_empty_memes() -> Memes{
    Memes{memes: vec![]}
}

pub async fn parse_memes_json(value: Response) -> Memes{
    let get_memes = value.json::<GetMemes>().await;
    let memes = match get_memes{
        Ok(val) => val.data,
        Err(err) => {println!("Rust says {:?}", err);
                            get_empty_memes()}
    };

    return memes
}

/*
The meme struct and associated structs and methods
*/
use serde::Deserialize;
use reqwest::{Response};


#[derive(Deserialize, Debug)]
pub struct Meme {
    id: String,
    name: String,
    url: String,
}

#[derive(Deserialize, Debug)]
pub struct Memes {
    memes: Vec<Meme>,
}

#[derive(Deserialize, Debug)]
pub struct GetMemes {
    pub data: Memes,
}

impl Meme{
    pub fn print(&self){
        println!("{},{},{}", self.name, self.id, self.url);
    }
}

impl Memes {
    pub fn length(&self) -> usize{
        self.memes.len()
    }

    pub fn all_memes(&self) -> &Vec<Meme>{
        &self.memes
    }

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

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

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_get_empty_memes(){
        let my_memes = get_empty_memes();
        assert_eq!(my_memes.length(), 0, "Expected empty Memes, but got {}", my_memes.length())
    }
    
    #[test]
    fn test_memes_length_one(){
        let my_memes = Memes{
            memes: vec![Meme{
                name:String::from("AJ Styles vs Undertaker"),
                id: String::from("4096"),
                url: String::from("https://qxf2.com/wwe")
            }]
        };
        assert_eq!(my_memes.length(), 1, "Expected length to be one but got {}", my_memes.length())
    }

    #[tokio::test]
    async fn test_parse_memes_json_error(){
        use http::response::Builder;
        let my_response = Builder::new()
                                .body("Qxf2")
                                .unwrap();
        let my_response = Response::from(my_response);
        let parsed_json = parse_memes_json(my_response).await;
        
        assert_eq!(0, parsed_json.length())
    }

    #[tokio::test]
    async fn test_parse_memes_json_valid(){
        use http::response::Builder;
        let my_response = Builder::new()
                                .body("{\"data\": {\"memes\":[{\"id\":\"64\", \"name\":\"Surprised Rock\", \"url\":\"https://qxf2.com\"}]}}")
                                .unwrap();
        let my_response = Response::from(my_response);
        let parsed_json = parse_memes_json(my_response).await;

        assert_eq!(1, parsed_json.length())
    }
}

/*
The meme struct and associated structs and methods
*/
use serde::{Deserialize, Serialize};
// from serde import Deserilize, Serialize
use reqwest::{Response};

trait Content<T> {
    fn new(&self) -> T {
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Meme {
    id: String,
    name: String,
    url: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Memes {
    memes: Vec<Meme>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetMemes {
    pub data: Memes,
}

impl GetMemes{
    fn new
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
    use serde_json;

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
        let meme = Meme{
            id: String::from("64"),
            name: String::from("Surprised Rock"),
            url: String::from("https://qxf2.com")
        }; 
        let memes: Memes = Memes{
            memes: vec!(meme)
        };
        let json_data: GetMemes = GetMemes{
            data: memes
         };
 
        
        // Serialize the GetMemes struct
        let meme_data_json = serde_json::to_string(&json_data).unwrap();
        println!("The Meme Data JSON is {:?}", meme_data_json);
    
        let my_response = Builder::new()
                                .body(meme_data_json)
                                .unwrap();
        let my_response = Response::from(my_response);
        let parsed_json = parse_memes_json(my_response).await;

        assert_eq!(1, parsed_json.length())
    }
    }

// Add a trait for the structs defined based on the following example
/*
pub trait Content {
    fn get_test_data() -> Self;
}

#[derive(Debug)]
pub struct Meme {
    pub id: String,
    pub name: String,
    pub url: String,
}

#[derive(Debug)]
pub struct Memes {
    memes: Vec<Meme>,
}


impl Content for Meme{
    fn get_test_data() -> Meme{
        return Meme{id: String::from("1"),
                    name: String::from("Qxf2"),
                    url: String::from("https://qxf2.com")}
    }
}

impl Content for Memes{
    fn get_test_data() -> Memes{
        return Memes{
            memes: vec!(Meme::get_test_data())
        }
    }
}

fn main(){
    let test_meme_data = Meme::get_test_data();
    println!("The Meme is {:?}", test_meme_data);
    let memes = Memes::get_test_data();
    println!("The Meme is {:?}", memes);
}
 */
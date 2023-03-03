/*
The meme struct and associated structs and methods
*/
use serde::{Deserialize, Serialize};
use reqwest::{Response};


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

    trait TestData<T> {
        fn single() -> T;
        fn multiple(num: u32) -> Vec<T>;
    }

    impl TestData<Meme> for Meme{
        fn single() -> Meme{
            return Meme{id: String::from("1"),
                    name: String::from("Qxf2"),
                    url: String::from("https://qxf2.com")}
        }
        fn multiple(num: u32) -> Vec<Meme>{
            let mut meme_array = vec!();
            for i in 0..num{
                meme_array.push(
                    Meme{id: String::from(i.to_string()),
                        name: format!("{} {}",String::from("Qxf2"),i.to_string()),
                        url: format!("{}/{}",String::from("https://qxf2.com"),i.to_string())
                    })
                }
            return meme_array
            }
    }

    impl TestData<Memes> for Memes{
        fn single() -> Memes{
            return Memes{
                    memes: vec!(Meme::single())
                }
        }

        fn multiple(num: u32) -> Vec<Memes>{
            let memes: Vec<Memes> = vec!(Memes{
                memes: Meme::multiple(num)
            });
            
            return memes
        }
    }

    impl TestData<GetMemes> for GetMemes{
        fn single() -> GetMemes{
            return GetMemes{
                data: Memes::single()
             }
        }

        fn multiple(num: u32) -> Vec<GetMemes>{
            return vec!(GetMemes::single())
        }
    }



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

    #[test]
    fn test_memes_length_five(){
        let num = 5;
        let my_memes = Memes::multiple(num);
        let memes = my_memes.into_iter().nth(0);
        match memes{
            Some(val) => assert_eq!(val.length(), num as usize, "Expected length to be {} but got {}", num, val.length()),
            None => assert_eq!(0, num, "Could not get a Memes struct")
        }
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
        let data = GetMemes::single();
        
        // Serialize the GetMemes struct
        let meme_data_json = serde_json::to_string(&data).unwrap();
        println!("The Meme Data JSON is {:?}", meme_data_json);
    
        let my_response = Builder::new()
                                .body(meme_data_json)
                                .unwrap();
        let my_response = Response::from(my_response);
        let parsed_json = parse_memes_json(my_response).await;

        assert_eq!(1, parsed_json.length())
    }
}

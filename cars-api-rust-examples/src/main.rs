/*
Example of making different REST calls to cars app using Rust
*/
use std::error::Error;
use reqwest;
use serde::Deserialize;
mod getenv;

#[derive(Deserialize, Debug)]
struct CarsList{
    cars_list: Vec<Car>
}

impl CarsList {
    fn count(&self){
        println!("There are {} cars in our inventory.", self.cars_list.len());
    }
}

#[derive(Deserialize, Debug)]
struct Car{
    brand: String,
    car_type: String,
    name: String,
    price_range: String
}

impl Car{
    fn print(&self){
        println!("\tCar brand: {:?}", self.brand);
        println!("\tCar type: {:?}", self.car_type);
        println!("\tCar name: {:?}", self.name);
        println!("\tCar price range: {:?}", self.price_range);
    }
    fn is_type(&self, car_type: &str) -> bool{
        if self.car_type == car_type{
            true
        } else {
            false
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let username = getenv::get_os_env_var("CARS_APP_USER");
    let password = getenv::get_os_env_var("CARS_APP_PASSWORD");
    let base_url = "https://cars-app.qxf2.com";
    let endpoint = "/cars";
    let client = reqwest::Client::new();
    let response = client.get(format!("{}{}", base_url, endpoint))
                        .basic_auth(username, Some(password))
                        .send()
                        .await?
                        .json::<CarsList>()
                        .await?;
    response.count();
    for car in response.cars_list.iter(){
        if car.is_type("hatchback"){
            println!("=========");
            car.print();
            println!("=========");
        }
    }
    Ok(())
}

/*
Fetch the top 100 memes from https://imgflip.com/
Print out the id and the name of the meme
URL: https://api.imgflip.com/get_memes
*/
use meme::imgflip;

#[tokio::main]
async fn main() {
    let all_memes = imgflip::get_meme_data().await;
    if all_memes.length() > 0{
        for meme in all_memes.all_memes() {
            meme.print();
        }
    } else {
        println!("Error when fetching memes. Got zero memes!")
    }
}

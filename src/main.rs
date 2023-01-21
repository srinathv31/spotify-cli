use reqwest::{self, header::{AUTHORIZATION, CONTENT_TYPE, ACCEPT}};
use std::fs::File;
use std::io::prelude::*;
use std::io;
mod spotify;

// tokio let's us use "async" on our main function
#[tokio::main]
async fn main() {

    let url = format!(
        "https://api.spotify.com/v1/search?q={query}&type=track,artist",
        // go check out her latest album. It's 🔥
        query = "Winona"
    );

    // chaining .await will yield our query result
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(AUTHORIZATION, "Bearer someToken")
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        // confirm the request using send()
        .send()
        .await
        // the rest is the same!
        .unwrap()
        .json::<spotify::APIResponse>()
        .await;
        // .text()
        // .await;
    let data = response.unwrap();
    print_tracks(data.tracks.items.iter().collect());
    
    println!("Enter any key to exit ...");
    let mut user_input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut user_input)
        .expect("Woah");
    println!("{user_input}")

}

fn print_tracks(tracks: Vec<&spotify::Track>) {
    for track in tracks {
        println!("🔥 {}", track.name);
        println!("💿 {}", track.album.name);
        println!(
            "🕺 {}",
            track
                .album
                .artists
                .iter()
                .map(|artist| artist.name.to_string())
                .collect::<String>()
        );
        println!("🌎 {}", track.external_urls.spotify);
        println!("---------")
    }
}

fn write_res(res: String) -> std::io::Result<()> {
    let mut file = File::create("./foo.txt")?;
    file.write_all(res.as_bytes())
        .expect("Write error");
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    assert_eq!(contents, res);
    Ok(())
}

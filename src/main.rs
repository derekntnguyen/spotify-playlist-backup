use reqwest;
use reqwest::header::ACCEPT;
use reqwest::header::AUTHORIZATION;
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

#[tokio::main]
async fn main() {
    let url = format!(
        "https://api.spotify.com/v1/recommendations?seed_artists={query}",
        // Creating an IU playlist
        query = "3HqSLMAZ3g3d5poNaI7GOU"
    );
    let client = reqwest::Client::new();
    let token = env::var("TOKEN").expect("$TOKEN is not set");
    let bearer = format!("Bearer {}", token);
    let response = client
        .get(url)
        .header(AUTHORIZATION, bearer)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap();
    match response.status() {
        reqwest::StatusCode::OK => {
            // on success, parse our JSON to an APIResponse
            match response.json::<APIResponse>().await {
                Ok(parsed) => print_tracks(parsed.tracks.items.iter().collect()),
                Err(_) => println!("Unexpected json structure"),
            };
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Token Expired");
        }
        other => {
            panic!("Unknown Error: {:?}", other);
        }
    };
}

fn write_tracks(tracks: Vec<&Track>) {
    let path = "playlist.txt";
    let f = File::create(path).expect("Error opening file.");
    let mut f = BufWriter::new(f);
    for track in tracks {
        write!(f, "{}", format!("Track: {}", track.name)).expect("Error writing to file.");
        write!(f, "{}", format!("Album: {}", track.album.name)).expect("Error writing to file.");
        write!(
            f,
            "{}",
            format!(
                "Artist(s): {}",
                track
                    .album
                    .artists
                    .iter()
                    .map(|artist| artist.name.to_string())
                    .collect::<String>()
            )
        )
        .expect("Error writing to file.");
        write!(f, "{}", format!("URL: {}", track.external_urls.spotify))
            .expect("Error writing to file.");
        write!(f, "{}", "---------").expect("Error writing to file.");
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {
    tracks: Items<Track>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Items<T> {
    items: Vec<T>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Track {
    album: Album,
    href: String,
    name: String,
    external_urls: ExternalUrls,
}
#[derive(Serialize, Deserialize, Debug)]
struct Album {
    artists: Vec<Artist>,
    external_urls: ExternalUrls,
    name: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Artist {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ExternalUrls {
    spotify: String,
}

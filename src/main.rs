use reqwest;
use reqwest::header::ACCEPT;
use reqwest::header::AUTHORIZATION;
use reqwest::header::CONTENT_TYPE;
use serde::Deserialize;
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
                Ok(parsed) => write_tracks(parsed.tracks.iter().collect()),
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
        writeln!(f, "{}", format!("Track: {}", track.name)).expect("Error writing to file.");
        writeln!(f, "{}", format!("   Album: {}", track.album.name))
            .expect("Error writing to file.");
        writeln!(
            f,
            "{}",
            format!(
                "   Artist(s): {}",
                track
                    .album
                    .artists
                    .iter()
                    .map(|artist| artist.name.to_string())
                    .collect::<String>()
            )
        )
        .expect("Error writing to file.");
        writeln!(f, "{}", format!("   URL: {}", track.external_urls.spotify))
            .expect("Error writing to file.");
        writeln!(f, "{}", "\n").expect("Error writing to file.");
    }
}

#[derive(Deserialize, Debug)]
struct APIResponse {
    tracks: Vec<Track>,
}

#[derive(Deserialize, Debug)]
struct Track {
    album: Album,
    external_urls: ExternalUrls,
    name: String,
}

#[derive(Deserialize, Debug)]
struct Album {
    artists: Vec<Artist>,
    // external_urls: ExternalUrls,
    name: String,
}

#[derive(Deserialize, Debug)]
struct Artist {
    name: String,
}

#[derive(Deserialize, Debug)]
struct ExternalUrls {
    spotify: String,
}

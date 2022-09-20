use reqwest::Client;
use serde::Deserialize;
use std::error::Error;

pub mod command;
mod format;
mod glyphs;
mod helper;
pub mod log;
pub mod queue;

// todo: autodetect the port (%appdata%/MusicBee/WWWServerconfig.xml)
static MUSICBEE_REST_URL: &str = "http://localhost:8080";

#[allow(dead_code)]
#[derive(Deserialize)]
struct NowPlaying {
    #[serde(rename = "Album")]
    album: String,
    #[serde(rename = "Artist")]
    artist: String,
    #[serde(rename = "Title")]
    title: String,
    position: u32,
    duration: u32,
    file: String,
    playing: String,
    queued: bool,
    repeat: String,
    scrobbling: bool,
    shuffle: bool,
    volume: f32,
}

impl NowPlaying {
    async fn new() -> Result<NowPlaying, Box<dyn Error>> {
        let body = Client::new()
            .get(format::url("NP"))
            .send()
            .await?
            .text()
            .await?;
        let np: NowPlaying = serde_json::from_str(&body)?;

        Ok(np)
    }
}

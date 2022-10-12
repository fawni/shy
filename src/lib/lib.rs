use reqwest::Client;
use serde::Deserialize;
use std::error::Error;

pub mod command;
mod format;
mod glyphs;
mod helper;
pub mod log;
pub mod playback;

// todo: autodetect the port (%appdata%/MusicBee/WWWServerconfig.xml)
static MUSICBEE_REST_URL: &str = "http://localhost:8080";

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
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
    playing: Option<PlayingStatus>,
    queued: bool,
    repeat: Option<String>,
    scrobbling: bool,
    shuffle: bool,
    volume: f32,
}

impl NowPlaying {
    async fn new() -> Result<NowPlaying, Box<dyn Error>> {
        let body = reqwest::get(format::url("NP")).await?.text().await?;
        let np: NowPlaying = serde_json::from_str(&body)?;

        Ok(np)
    }

    async fn with(c: &Client) -> Result<NowPlaying, Box<dyn Error>> {
        let body = c.get(format::url("NP")).send().await?.text().await?;
        let np: NowPlaying = serde_json::from_str(&body)?;

        Ok(np)
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum PlayingStatus {
    #[serde(rename = "loading")]
    Loading,
    #[serde(rename = "playing")]
    Playing,
    #[serde(rename = "paused")]
    Paused,
    #[serde(rename = "stopped")]
    Stopped,
    #[serde(other)]
    Unkown,
}

#[derive(Debug)]
pub enum ShuffleStatus {
    On,
    Off,
    Toggle,
}

impl From<bool> for ShuffleStatus {
    fn from(b: bool) -> Self {
        match b {
            true => Self::On,
            false => Self::Off,
        }
    }
}

impl From<&String> for ShuffleStatus {
    fn from(s: &String) -> Self {
        match s.as_str() {
            "on" => Self::On,
            "off" => Self::Off,
            _ => Self::Toggle,
        }
    }
}

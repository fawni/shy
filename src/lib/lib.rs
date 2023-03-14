use async_once::AsyncOnce;
use lazy_static::lazy_static;
use reqwest::Client;
use roxmltree::Document;
use serde::Deserialize;
use tokio::fs;

pub mod player;

mod fmt;
mod glyphs;
mod helper;
mod log;

lazy_static! {
    static ref API_BASE: AsyncOnce<String> = AsyncOnce::new(async {
        let port = get_port().await.unwrap();

        format!("http://localhost:{port}")
    });
}

static VALID_FORMATS: [&str; 29] = [
    "mp3", "m4a", "mp4", "3gp", "m4b", "m4p", "m4r", "m4v", "aac", "mpc", "mp+", "mpp", "ogg",
    "ogv", "oga", "ogx", "ogm", "spx", "opus", "flac", "caf", "ape", "wv", "wma", "wav", "wave",
    "mid", "mod", "xm",
];

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct NowPlaying {
    #[serde(rename = "Album")]
    album: String,
    #[serde(rename = "Artist")]
    artist: String,
    #[serde(rename = "Title")]
    title: String,
    position: i32,
    duration: i32,
    file: String,
    playing: Option<PlayingStatus>,
    queued: bool,
    repeat: Option<String>,
    scrobbling: bool,
    shuffle: bool,
    volume: f32,
}

impl NowPlaying {
    async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let body = reqwest::get(url!("NP")).await?.text().await?;
        let np: Self = serde_json::from_str(&body)?;

        Ok(np)
    }

    async fn with(client: &Client) -> Result<Self, Box<dyn std::error::Error>> {
        let body = client.get(url!("NP")).send().await?.text().await?;
        let np: Self = serde_json::from_str(&body)?;

        Ok(np)
    }
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Clone, Copy)]
pub enum ShuffleStatus {
    Off,
    On,
}

impl ShuffleStatus {
    fn toggle(&self) -> Self {
        match self {
            Self::Off => Self::On,
            Self::On => Self::Off,
        }
    }

    fn text(&self) -> &'static str {
        match self {
            Self::Off => "OFF",
            Self::On => "ON",
        }
    }
}

impl From<bool> for ShuffleStatus {
    fn from(b: bool) -> Self {
        if b {
            Self::On
        } else {
            Self::Off
        }
    }
}

impl TryFrom<String> for ShuffleStatus {
    type Error = &'static str;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "on" => Ok(Self::On),
            "off" => Ok(Self::Off),
            _ => Err("Invalid shuffle status"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RepeatStatus {
    Off,
    On,
    One,
}

impl RepeatStatus {
    fn toggle(&self) -> Self {
        match self {
            Self::Off => Self::On,
            Self::On => Self::One,
            Self::One => Self::Off,
        }
    }

    fn text(&self) -> &'static str {
        match self {
            Self::Off => "OFF",
            Self::On => "ON",
            Self::One => "ONE",
        }
    }
}

impl TryFrom<String> for RepeatStatus {
    type Error = &'static str;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "none" | "off" => Ok(Self::Off),
            "all" | "queue" | "on" => Ok(Self::On),
            "single" | "one" | "track" => Ok(Self::One),
            _ => Err("Invalid repeat status"),
        }
    }
}

async fn get_port() -> Result<String, Box<dyn std::error::Error>> {
    let dir = dirs::config_dir().unwrap();
    let file = fs::read_to_string(format!(
        "{}\\MusicBee\\WWWServerconfig.xml",
        dir.to_string_lossy()
    ))
    .await?;
    let doc = Document::parse(&file)?;
    let port = Document::descendants(&doc)
        .find(|n| n.has_tag_name("port"))
        .and_then(|n| n.text())
        .map(|n| n.to_string())
        .unwrap();

    Ok(port)
}

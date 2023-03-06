use async_once::AsyncOnce;
use lazy_static::lazy_static;
use reqwest::Client;
use roxmltree::Document;
use serde::Deserialize;

pub mod player;

mod fmt;
mod glyphs;
mod helper;
mod macros;

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
    async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let body = reqwest::get(fmt::url("NP").await).await?.text().await?;
        let np: Self = serde_json::from_str(&body)?;

        Ok(np)
    }

    async fn with(c: &Client) -> Result<Self, Box<dyn std::error::Error>> {
        let body = c.get(fmt::url("NP").await).send().await?.text().await?;
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

#[derive(Debug)]
pub enum ShuffleStatus {
    On,
    Off,
    Toggle,
}

impl ShuffleStatus {
    fn toggle(&self) -> Self {
        match self {
            Self::On => Self::Off,
            Self::Off => Self::On,
            Self::Toggle => unreachable!(),
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

impl From<&String> for ShuffleStatus {
    fn from(s: &String) -> Self {
        match s.as_str() {
            "on" => Self::On,
            "off" => Self::Off,
            _ => Self::Toggle,
        }
    }
}

#[derive(Debug)]
pub enum RepeatStatus {
    None,
    All,
    Single,
    Toggle,
}

impl RepeatStatus {
    fn toggle(&self) -> Self {
        match self {
            Self::None => Self::All,
            Self::All => Self::Single,
            Self::Single => Self::None,
            Self::Toggle => unreachable!(),
        }
    }
}

impl From<&String> for RepeatStatus {
    fn from(s: &String) -> Self {
        match s.as_str() {
            "none" | "off" => Self::None,
            "all" | "queue" | "on" => Self::All,
            "single" | "one" | "track" => Self::Single,
            _ => Self::Toggle,
        }
    }
}

async fn get_port() -> Result<String, Box<dyn std::error::Error>> {
    let dir = dirs::config_dir().unwrap();
    let file = tokio::fs::read_to_string(format!(
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

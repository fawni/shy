use async_once::AsyncOnce;
use clap::ValueEnum;
use lazy_static::lazy_static;
use miette::{Context, IntoDiagnostic};
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
        let port = get_port().await.unwrap_or(8080.to_string());

        format!("http://localhost:{port}")
    });
}

static VALID_FORMATS: [&str; 29] = [
    "mp3", "m4a", "mp4", "3gp", "m4b", "m4p", "m4r", "m4v", "aac", "mpc", "mp+", "mpp", "ogg",
    "ogv", "oga", "ogx", "ogm", "spx", "opus", "flac", "caf", "ape", "wv", "wma", "wav", "wave",
    "mid", "mod", "xm",
];

#[derive(Deserialize, Debug)]
pub struct NowPlaying {
    pub album: String,
    pub artist: String,
    pub title: String,
    pub position: i32,
    pub duration: i32,
    pub file: String,
    pub playing: Option<PlayingStatus>,
    pub queued: bool,
    pub repeat: Option<String>,
    pub scrobbling: bool,
    pub shuffle: bool,
    pub volume: f32,
}

impl NowPlaying {
    async fn new() -> miette::Result<Self> {
        let np = reqwest::get(url!("NP"))
            .await
            .into_diagnostic()?
            .json::<Self>()
            .await
            .into_diagnostic()?;

        Ok(np)
    }

    async fn with(client: &Client) -> miette::Result<Self> {
        let np = client
            .get(url!("NP"))
            .send()
            .await
            .into_diagnostic()?
            .json::<Self>()
            .await
            .into_diagnostic()?;

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

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ShuffleMode {
    Off,
    On,
}

impl From<bool> for ShuffleMode {
    fn from(b: bool) -> Self {
        if b {
            Self::On
        } else {
            Self::Off
        }
    }
}

impl ShuffleMode {
    const fn toggle(&self) -> Self {
        match self {
            Self::Off => Self::On,
            Self::On => Self::Off,
        }
    }

    fn text(&self) -> String {
        match self {
            Self::Off => String::from("OFF"),
            Self::On => String::from("ON"),
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum RepeatMode {
    /// Repeat off
    Off,

    /// Repeat queue
    On,

    /// Repeat track
    One,
}

impl RepeatMode {
    const fn toggle(&self) -> Self {
        match self {
            Self::Off => Self::On,
            Self::On => Self::One,
            Self::One => Self::Off,
        }
    }

    fn text(&self) -> String {
        match self {
            Self::Off => String::from("OFF"),
            Self::On => String::from("ON"),
            Self::One => String::from("ONE"),
        }
    }
}

impl From<String> for RepeatMode {
    fn from(s: String) -> Self {
        match &*s {
            "none" | "off" => Self::Off,
            "all" | "queue" | "on" => Self::On,
            "single" | "track" | "one" => Self::One,
            _ => unreachable!(),
        }
    }
}

async fn get_port() -> miette::Result<String> {
    let dir = dirs::config_dir().unwrap();
    let file = fs::read_to_string(format!(
        "{}\\MusicBee\\WWWServerconfig.xml",
        dir.to_string_lossy()
    ))
    .await
    .into_diagnostic()
    .wrap_err("mb_WWWServer::Config")?;
    let doc = Document::parse(&file).into_diagnostic()?;
    let port = Document::descendants(&doc)
        .find(|n| n.has_tag_name("port"))
        .and_then(|n| n.text())
        .map(ToString::to_string)
        .unwrap();

    Ok(port)
}

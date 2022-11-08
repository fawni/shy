use lazy_static::lazy_static;
use reqwest::Client;
use roxmltree::Document;
use serde::Deserialize;
use std::{error::Error, fs};

pub mod command;
mod format;
mod glyphs;
mod helper;
pub mod log;
pub mod playback;

lazy_static! {
    static ref API_BASE: String = format!("http://localhost:{}", get_port().unwrap());
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

fn get_port() -> Result<String, Box<dyn Error>> {
    let config_dir = dirs::config_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();
    let file = fs::read_to_string(format!("{config_dir}\\MusicBee\\WWWServerconfig.xml"))?;
    let doc = Document::parse(&file)?;
    let port = doc
        .descendants()
        .find(|n| n.has_tag_name("port"))
        .unwrap()
        .text()
        .unwrap()
        .to_string();

    Ok(port)
}

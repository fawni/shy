use clap::ValueEnum;
use miette::IntoDiagnostic;
use once_cell::sync::OnceCell;
use reqwest::Client;
use serde::Deserialize;

pub mod player;

mod fmt;
mod glyphs;
mod helper;
mod log;

pub fn api_base() -> &'static String {
    static API_BASE: OnceCell<String> = OnceCell::new();
    API_BASE.get_or_init(|| {
        let port = get_port().unwrap_or(8080.to_string());
        format!("http://localhost:{port}")
    })
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
#[serde(rename_all = "lowercase")]
pub enum PlayingStatus {
    Loading,
    Playing,
    Paused,
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

#[derive(Deserialize)]
struct PluginConfig {
    port: String,
}

fn get_port() -> Result<String, Box<dyn std::error::Error>> {
    let config_file = fs_err::read_to_string(format!(
        "{}\\MusicBee\\WWWServerconfig.xml",
        dirs::config_dir().unwrap().to_string_lossy()
    ))?;
    let port = serde_xml_rs::from_str::<PluginConfig>(&config_file)?.port;

    Ok(port)
}

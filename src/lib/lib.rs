use std::error::Error;

use helper::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};

mod helper;

pub mod control;
pub mod queue;

static MUSICBEE_REST_URL: &str = "http://localhost:5454";

#[derive(Serialize, Deserialize)]
pub(crate) struct NowPlaying {
    #[serde(rename = "Album")]
    pub(crate) album: String,
    #[serde(rename = "Artist")]
    pub(crate) artist: String,
    #[serde(rename = "Title")]
    pub(crate) title: String,
    pub(crate) position: u32,
    pub(crate) duration: u32,
    pub(crate) file: String,
    pub(crate) playing: String,
    pub(crate) queued: bool,
    pub(crate) repeat: String,
    pub(crate) scrobbling: bool,
    pub(crate) shuffle: bool,
    pub(crate) volume: f32,
}

pub(crate) async fn np() -> Result<NowPlaying, Box<dyn Error>> {
    let body = Client::new()
        .get(format_url("NP"))
        .send()
        .await?
        .text()
        .await?;

    let np: NowPlaying = serde_json::from_str(&body)?;
    Ok(np)
}

use owo_colors::OwoColorize;
use reqwest::Client;
use std::error::Error;

use crate::{helper::*, np};

pub async fn play() -> Result<String, Box<dyn Error>> {
    Client::new()
        .get(format_url("C_PP"))
        .send()
        .await?
        .text()
        .await?;
    let np = np().await?;
    let res = format!(
        "{} | {} by {}",
        np.playing.to_uppercase(),
        np.title.bold(),
        np.artist,
    );

    Ok(res)
}

pub async fn stop() -> Result<String, Box<dyn Error>> {
    Client::new()
        .get(format_url("C_STOP"))
        .send()
        .await?
        .text()
        .await?;
    let np = np().await?;
    let res = format!("STOPPED | {} by {}", np.title.bold(), np.artist,);

    Ok(res)
}

pub async fn next() -> Result<String, Box<dyn Error>> {
    Ok(Client::new()
        .get(format_url("C_NEXT"))
        .send()
        .await?
        .text()
        .await?)
}

pub async fn previous() -> Result<String, Box<dyn Error>> {
    Ok(Client::new()
        .get(format_url("C_PREV"))
        .send()
        .await?
        .text()
        .await?)
}

pub async fn volume(amount: &str) -> Result<String, Box<dyn Error>> {
    Ok(Client::new()
        .get(format_url_path("C_VOL", calculate_volume(amount).await))
        .send()
        .await?
        .text()
        .await?)
}

async fn calculate_volume(_amount: &str) -> &str {
    // amount could be: +10, -200, 85
    "40"
}

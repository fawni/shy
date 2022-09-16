use crate::{helper::*, NowPlaying};
use owo_colors::OwoColorize;
use reqwest::Client;
use std::error::Error;

pub async fn play() -> Result<String, Box<dyn Error>> {
    Client::new()
        .get(format_url("C_PP"))
        .send()
        .await?
        .text()
        .await?;
    let np = NowPlaying::new().await?;
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
    let np = NowPlaying::new().await?;
    let res = format!("STOPPED | {} by {}", np.title.bold(), np.artist,);

    Ok(res)
}

pub async fn next() -> Result<String, Box<dyn Error>> {
    let old = NowPlaying::new().await?;
    Client::new()
        .get(format_url("C_NEXT"))
        .send()
        .await?
        .text()
        .await?;
    let np = NowPlaying::new().await?;
    let res = format!(
        "SKIPPED | {} by {}\nPLAYING | {} by {}",
        old.title.bold(),
        old.artist,
        np.title.bold(),
        np.artist
    );

    Ok(res)
}

pub async fn previous() -> Result<String, Box<dyn Error>> {
    let old = NowPlaying::new().await?;
    Client::new()
        .get(format_url("C_PREV"))
        .send()
        .await?
        .text()
        .await?;
    let np = NowPlaying::new().await?;
    let res = format!(
        "SKIPPED | {} by {}\nPLAYING | {} by {}",
        old.title.bold(),
        old.artist,
        np.title.bold(),
        np.artist
    );

    Ok(res)
}

pub async fn volume(amount: impl ToString) -> Result<String, Box<dyn Error>> {
    Client::new()
        .get(format_url_path(
            "C_VOL",
            parse_volume(amount.to_string()).await?,
        ))
        .send()
        .await?
        .text()
        .await?;
    let res = format!("Changed volume to {}", NowPlaying::new().await?.volume);

    Ok(res)
}

async fn parse_volume(amount: impl ToString) -> Result<impl ToString, Box<dyn Error>> {
    let amount = amount.to_string();
    if amount.starts_with('+') {
        let current = (NowPlaying::new().await?.volume * 100.0) as u32;
        let res = current + amount.trim_start_matches('+').parse::<u32>()?;
        Ok(res.to_string())
    // this gets parsed as an arg which can be bypassed by preceeding it with "--". i'm not sure if there is a nicer way to do this or not.
    } else if amount.starts_with('-') {
        let current = (NowPlaying::new().await?.volume * 100.0) as u32;
        let res = current - amount.trim_start_matches('-').parse::<u32>()?;
        Ok(res.to_string())
    } else {
        Ok(amount.to_string())
    }
}

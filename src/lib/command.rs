use crate::{helper::*, NowPlaying};
use owo_colors::OwoColorize;
use reqwest::Client;
use std::error::Error;

pub async fn play() -> Result<String, Box<dyn Error>> {
    Client::new().get(format_url("C_PP")).send().await?;
    let np = NowPlaying::new().await?;
    let res = format!(
        "{} | {} by {}",
        np.playing.to_uppercase(),
        np.title,
        np.artist,
    );

    Ok(res)
}

pub async fn stop() -> Result<String, Box<dyn Error>> {
    Client::new().get(format_url("C_STOP")).send().await?;
    let np = NowPlaying::new().await?;
    let res = format!("STOPPED | {} by {}", np.title.bold(), np.artist,);

    Ok(res)
}

pub async fn next() -> Result<String, Box<dyn Error>> {
    let old = NowPlaying::new().await?;
    Client::new().get(format_url("C_NEXT")).send().await?;
    let np = NowPlaying::new().await?;
    let res = format!(
        "SKIPPED | {} by {}\nPLAYING | {} by {}",
        old.title, old.artist, np.title, np.artist
    );

    Ok(res)
}

pub async fn previous() -> Result<String, Box<dyn Error>> {
    let old = NowPlaying::new().await?;
    Client::new().get(format_url("C_PREV")).send().await?;
    let np = NowPlaying::new().await?;
    let res = format!(
        "SKIPPED | {} by {}\nPLAYING | {} by {}",
        old.title, old.artist, np.title, np.artist
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
        .await?;
    let res = format!("Changed volume to {}", NowPlaying::new().await?.volume);

    Ok(res)
}

pub async fn seek(amount: impl ToString) -> Result<String, Box<dyn Error>> {
    Client::new()
        .get(format_url_path(
            "C_SEEK",
            parse_position(amount.to_string()).await?,
        ))
        .send()
        .await?;
    let res = format!("Changed volume to {}", NowPlaying::new().await?.volume);

    Ok(res)
}

async fn parse_volume(input: impl ToString) -> Result<impl ToString, Box<dyn Error>> {
    let amount = input.to_string();
    let volume = NowPlaying::new().await?.volume;
    if amount.starts_with('+') {
        let current = (volume * 100.0) as u32;
        let res = current + amount.trim_start_matches('+').parse::<u32>()?;
        Ok(res.to_string())
    } else if amount.starts_with('-') {
        let current = (volume * 100.0) as u32;
        let res = current - amount.trim_start_matches('-').parse::<u32>()?;
        Ok(res.to_string())
    } else {
        Ok(amount)
    }
}

async fn parse_position(input: impl ToString) -> Result<impl ToString, Box<dyn Error>> {
    let amount = input.to_string();
    let np = NowPlaying::new().await?;
    let (pos, total) = (np.position, np.duration);
    if amount.ends_with('%') {
        let percentage = amount.trim_end_matches('%');
        // +5%
        if percentage.starts_with('+') {
            let amount = percentage.trim_start_matches('+').parse::<u32>()?;
            let current = (pos / total) * 100;
            let desired = current + amount;
            let res = (desired * total) / 100;
            Ok(res.to_string())
        // -5%
        } else if percentage.starts_with('-') {
            let amount = percentage.trim_start_matches('-').parse::<u32>()?;
            let current = (pos / total) * 100;
            let desired = current - amount;
            let res = (desired * total) / 100;
            Ok(res.to_string())
        // 5%
        } else {
            let amount = percentage.parse::<u32>()?;
            let res = (total * amount) / 100;
            Ok(res.to_string())
        }
    } else {
        // +5 (seconds)
        if amount.starts_with('+') {
            let amount = amount.trim_start_matches('+').parse::<u32>()? * 1000;
            let res = pos + amount;
            Ok(res.to_string())
        // -5 (seconds)
        } else if amount.starts_with('-') {
            let amount = amount.trim_start_matches('-').parse::<u32>()? * 1000;
            let res = pos - amount;
            Ok(res.to_string())
        // 5 (will treat as 5% anyway)
        } else {
            let res = (total * amount.parse::<u32>()?) / 100;
            Ok(res.to_string())
        }
    }
}

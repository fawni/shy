use std::path::Path;

use owo_colors::OwoColorize;
use reqwest::Client;
use tokio::fs;

use crate::{
    glyphs, helper, info, url, NowPlaying, PlayingStatus, RepeatStatus, ShuffleStatus,
    VALID_FORMATS,
};

pub async fn add(path: &str, next: bool) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    if fs::metadata(path).await?.is_dir() {
        add_directory(&client, path, next).await?;
    } else {
        add_file(&client, path, next).await?;
    }

    Ok(())
}

pub async fn clear() -> Result<String, Box<dyn std::error::Error>> {
    reqwest::get(url!("CLEAR")).await?;
    let res = format!("{} Cleared queue", glyphs::CLEAR.red());
    Ok(res)
}

pub async fn play() -> Result<String, Box<dyn std::error::Error>> {
    reqwest::get(url!("C_PP")).await?;
    let np = NowPlaying::new().await?;
    match np.playing {
        Some(PlayingStatus::Playing) => {
            let res = format!(
                "{} {} by {}",
                glyphs::PLAY.green(),
                np.title.bold(),
                np.artist
            );
            Ok(res)
        }
        Some(PlayingStatus::Paused) => {
            let res = format!(
                "{} {} by {}",
                glyphs::PAUSE.red(),
                np.title.bold(),
                np.artist
            );
            Ok(res)
        }
        _ => Err("Failed to fetch NP".into()),
    }
}

pub async fn stop() -> Result<String, Box<dyn std::error::Error>> {
    reqwest::get(url!("C_STOP")).await?;
    let np = NowPlaying::new().await?;
    let res = format!(
        "{} {} by {}",
        glyphs::STOP.red(),
        np.title.bold(),
        np.artist,
    );

    Ok(res)
}

pub async fn next() -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let old = NowPlaying::with(&client).await?;
    client.get(url!("C_NEXT")).send().await?;
    let np = NowPlaying::with(&client).await?;
    let res = format!(
        "{} {} by {}\n{} {} by {}",
        glyphs::NEXT.red(),
        old.title.bold(),
        old.artist,
        glyphs::PLAY.green(),
        np.title.bold(),
        np.artist
    );

    Ok(res)
}

pub async fn previous() -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let old = NowPlaying::with(&client).await?;
    client.get(url!("C_PREV")).send().await?;
    let np = NowPlaying::with(&client).await?;

    let res = format!(
        "{} {} by {}\n{} {} by {}",
        glyphs::PREV.red(),
        old.title.bold(),
        old.artist,
        glyphs::PLAY.green(),
        np.title.bold(),
        np.artist
    );

    Ok(res)
}

pub async fn volume(amount: Option<String>) -> Result<String, Box<dyn std::error::Error>> {
    let res = if let Some(amount) = amount {
        let client = Client::new();
        let volume = helper::parse_volume(amount).await?;
        client.get(url!("C_VOL", &volume)).send().await?;
        let volume = NowPlaying::with(&client).await?.volume * 100.0;

        volume.to_string()
    } else {
        let np = NowPlaying::new().await?;
        let volume = (np.volume * 100.0) as u8;

        volume.to_string()
    };

    Ok(res)
}

pub async fn seek(amount: String) -> Result<String, Box<dyn std::error::Error>> {
    let pos = helper::parse_position(amount.clone()).await?.to_string();
    reqwest::get(url!("C_SEEK", &pos)).await?;
    let res = if amount.ends_with('%') {
        format!("Set position to {}", amount.bold())
    } else {
        format!("Seeked {} seconds", amount.bold())
    };

    Ok(res)
}

pub async fn shuffle(
    status: Option<Result<ShuffleStatus, &str>>,
) -> Result<&str, Box<dyn std::error::Error>> {
    let client = Client::new();
    let status = match status {
        Some(status) => status?,
        None => {
            let current_status = ShuffleStatus::from(NowPlaying::with(&client).await?.shuffle);
            ShuffleStatus::toggle(&current_status)
        }
    };

    let path = (status as u8).to_string();
    client.get(url!("C_SHUF", &path)).send().await?;

    Ok(status.text())
}

pub async fn repeat(
    status: Option<Result<RepeatStatus, &str>>,
) -> Result<&str, Box<dyn std::error::Error>> {
    let client = Client::new();
    let status = match status {
        Some(status) => status?,
        None => {
            let current_status = RepeatStatus::try_from(
                NowPlaying::with(&client).await?.repeat.unwrap_or_default(),
            )?;
            RepeatStatus::toggle(&current_status)
        }
    };

    let path = (status as u8).to_string();
    client.get(url!("C_REP", &path)).send().await?;

    Ok(status.text())
}

async fn add_file(
    client: &Client,
    path: &str,
    next: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let absolute_path = fs::canonicalize(path).await?.to_string_lossy().into_owned();
    let encoded = urlencoding::encode(&absolute_path);
    let endpoint = if next { "ADDNEXT" } else { "ADDITEM" };

    client.get(url!(endpoint, &encoded)).send().await?;
    let name = Path::file_name(Path::new(&path)).unwrap().to_string_lossy();

    Ok(info!("Added \"{name}\""))
}

async fn add_directory(
    client: &Client,
    path: &str,
    next: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    for file in std::fs::read_dir(path)? {
        let path = file?.path();
        let ext = match &path.extension() {
            Some(ext) => ext.to_str().unwrap(),
            None => continue,
        };

        if VALID_FORMATS.contains(&ext) {
            add_file(client, path.to_str().unwrap(), next).await?;
        }
    }

    Ok(())
}

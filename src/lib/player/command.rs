use std::path::Path;

use miette::{miette, IntoDiagnostic};
use owo_colors::OwoColorize;
use reqwest::Client;

use crate::{
    glyphs, helper, info, url, NowPlaying, PlayingStatus, RepeatMode, ShuffleMode, VALID_FORMATS,
};

pub async fn start() -> miette::Result<String> {
    tokio::task::spawn_blocking(|| -> miette::Result<()> {
        // TODO: Don't hardcode path somehow. this probably won't work on 32bit systems or msstore version
        tokio::process::Command::new(r"C:\Program Files (x86)\MusicBee\MusicBee.exe")
            .spawn()
            .into_diagnostic()?;
        Ok(())
    });

    Ok("Started MusicBee".to_owned())
}

pub async fn add(path: &str, next: bool) -> miette::Result<()> {
    let client = Client::new();
    if Path::new(path).is_dir() {
        add_directory(&client, path, next).await?;
    } else {
        add_file(&client, path, next).await?;
    }

    Ok(())
}

pub async fn clear() -> miette::Result<String> {
    reqwest::get(url!("CLEAR")).await.into_diagnostic()?;
    let res = format!("{} Cleared queue", glyphs::CLEAR.red());

    Ok(res)
}

pub async fn play() -> miette::Result<String> {
    reqwest::get(url!("C_PP")).await.into_diagnostic()?;
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
        _ => Err(miette!("Could not determine playing status")),
    }
}

pub async fn stop() -> miette::Result<String> {
    reqwest::get(url!("C_STOP")).await.into_diagnostic()?;
    let np = NowPlaying::new().await?;
    let res = format!(
        "{} {} by {}",
        glyphs::STOP.red(),
        np.title.bold(),
        np.artist,
    );

    Ok(res)
}

pub async fn next() -> miette::Result<String> {
    let client = Client::new();
    let old = NowPlaying::with(&client).await?;
    client.get(url!("C_NEXT")).send().await.into_diagnostic()?;
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

pub async fn previous() -> miette::Result<String> {
    let client = Client::new();
    let old = NowPlaying::with(&client).await?;
    client.get(url!("C_PREV")).send().await.into_diagnostic()?;
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

pub async fn volume(amount: Option<String>) -> miette::Result<String> {
    let res = if let Some(amount) = amount {
        let client = Client::new();
        let volume = helper::parse_volume(&amount).await?;
        client
            .get(url!("C_VOL", &volume))
            .send()
            .await
            .into_diagnostic()?;
        let volume = NowPlaying::with(&client).await?.volume * 100.0;

        format!("{:.0}", volume)
    } else {
        let np = NowPlaying::new().await?;
        let volume = (np.volume * 100.0) as u8;

        volume.to_string()
    };

    Ok(res)
}

pub async fn seek(amount: String) -> miette::Result<String> {
    let pos = helper::parse_position(&amount).await?.to_string();
    reqwest::get(url!("C_SEEK", &pos)).await.into_diagnostic()?;
    let res = if amount.ends_with('%') {
        format!("Set position to {}", amount.bold())
    } else {
        format!("Seeked {} seconds", amount.bold())
    };

    Ok(res)
}

pub async fn shuffle(mode: Option<ShuffleMode>) -> miette::Result<String> {
    let client = Client::new();
    let mode = if let Some(m) = mode {
        m
    } else {
        let current_mode = NowPlaying::with(&client).await?.shuffle.into();
        ShuffleMode::toggle(current_mode)
    };

    let path = (mode as u8).to_string();
    client
        .get(url!("C_SHUF", &path))
        .send()
        .await
        .into_diagnostic()?;

    Ok(mode.text())
}

pub async fn repeat(mode: Option<RepeatMode>) -> miette::Result<String> {
    let client = Client::new();
    let mode = if let Some(m) = mode {
        m
    } else {
        let current_mode = NowPlaying::with(&client)
            .await?
            .repeat
            .ok_or_else(|| miette!("Could not determine repeat mode"))?;
        RepeatMode::toggle(current_mode)
    };

    let path = (mode as u8).to_string();
    client
        .get(url!("C_REP", &path))
        .send()
        .await
        .into_diagnostic()?;

    Ok(mode.text())
}

async fn add_file(client: &Client, path: &str, next: bool) -> miette::Result<()> {
    let absolute_path = fs_err::tokio::canonicalize(path)
        .await
        .into_diagnostic()?
        .to_string_lossy()
        .into_owned();
    let encoded = urlencoding::encode(&absolute_path);
    let endpoint = if next { "ADDNEXT" } else { "ADDITEM" };

    client
        .get(url!(endpoint, &encoded))
        .send()
        .await
        .into_diagnostic()?;
    let name = Path::file_name(Path::new(path))
        .ok_or_else(|| miette!("Could not get file name"))?
        .to_string_lossy();

    Ok(info!("Added \"{name}\""))
}

async fn add_directory(client: &Client, path: &str, next: bool) -> miette::Result<()> {
    for file in fs_err::read_dir(path).into_diagnostic()? {
        let path = file.into_diagnostic()?.path();
        let ext = match &path.extension() {
            Some(ext) => ext
                .to_str()
                .ok_or_else(|| miette!("Could not convert extension to str"))?,
            None => continue,
        };

        if VALID_FORMATS.contains(&ext) {
            add_file(client, &path.to_string_lossy(), next).await?;
        }
    }

    Ok(())
}

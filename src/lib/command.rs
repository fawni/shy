use crate::{format, glyphs::*, helper::*, log, NowPlaying};
use owo_colors::OwoColorize;
use reqwest::Client;
use std::{error::Error, fs, path::Path, vec};

pub async fn add(path: impl ToString) -> Result<(), Box<dyn Error>> {
    if fs::metadata(path.to_string())?.is_dir() {
        add_directory(path).await?;
    } else {
        add_file(path).await?;
    }

    Ok(())
}

pub async fn play() -> Result<String, Box<dyn Error>> {
    Client::new().get(format::url("C_PP")).send().await?;
    let np = NowPlaying::new().await?;
    let res = if np.playing == "playing" {
        format!("{} {} by {}", PLAY.green(), np.title.bold(), np.artist)
    } else {
        format!("{} {} by {}", PAUSE.red(), np.title.bold(), np.artist)
    };

    Ok(res)
}

pub async fn stop() -> Result<String, Box<dyn Error>> {
    Client::new().get(format::url("C_STOP")).send().await?;
    let np = NowPlaying::new().await?;
    let res = format!("{} {} by {}", STOP.red(), np.title.bold(), np.artist,);

    Ok(res)
}

pub async fn next() -> Result<String, Box<dyn Error>> {
    let old = NowPlaying::new().await?;
    Client::new().get(format::url("C_NEXT")).send().await?;
    let np = NowPlaying::new().await?;
    let res = format!(
        "{} {} by {}\n{} {} by {}",
        NEXT.red(),
        old.title.bold(),
        old.artist,
        PLAY.green(),
        np.title.bold(),
        np.artist
    );

    Ok(res)
}

pub async fn previous() -> Result<String, Box<dyn Error>> {
    let old = NowPlaying::new().await?;
    Client::new().get(format::url("C_PREV")).send().await?;
    let np = NowPlaying::new().await?;
    let res = format!(
        "{} {} by {}\n{} {} by {}",
        PREV.red(),
        old.title.bold(),
        old.artist,
        PLAY.green(),
        np.title.bold(),
        np.artist
    );

    Ok(res)
}

pub async fn volume(amount: impl ToString) -> Result<String, Box<dyn Error>> {
    Client::new()
        .get(format::url_path(
            "C_VOL",
            parse_volume(amount.to_string()).await?,
        ))
        .send()
        .await?;
    let res = format!(
        "Volume set to {}%",
        (NowPlaying::new().await?.volume * 100.0).bold()
    );

    Ok(res)
}

pub async fn seek(amount: impl ToString) -> Result<String, Box<dyn Error>> {
    Client::new()
        .get(format::url_path(
            "C_SEEK",
            parse_position(amount.to_string()).await?,
        ))
        .send()
        .await?;
    let res = if amount.to_string().ends_with('%') {
        format!("Set position to {}", amount.to_string().bold())
    } else {
        format!("Seeked {} seconds", amount.to_string().bold())
    };

    Ok(res)
}

async fn add_file(path: impl ToString) -> Result<(), Box<dyn Error>> {
    let absolute_path = fs::canonicalize(path.to_string())?
        .to_string_lossy()
        .to_string();
    let encoded = urlencoding::encode(absolute_path.trim_end_matches(r"\\?\"));

    log::info(format!(
        "Adding {:?}",
        Path::file_name(Path::new(&path.to_string())).unwrap()
    ));
    // returns an error when it shouldnt so just ignore error lole, https://github.com/hyperium/hyper/issues/2136
    _ = Client::new()
        .get(format::url_path("ADDITEM", &encoded))
        .send()
        .await;

    Ok(())
}

async fn add_directory(path: impl ToString) -> Result<(), Box<dyn Error>> {
    let valid = vec![
        "mp3", "m4a", "mp4", "3gp", "m4b", "m4p", "m4r", "m4v", "aac", "mpc", "mp+", "mpp", "ogg",
        "ogv", "oga", "ogx", "ogm", "spx", "opus", "flac", "caf", "ape", "wv", "wma", "wav",
        "wave", "mid", "mod", "xm",
    ];

    for file in fs::read_dir(path.to_string())? {
        let path = file?.path();
        let ext = match &path.extension() {
            Some(ext) => ext.to_str().unwrap(),
            None => continue,
        };
        if valid.contains(&ext) {
            add_file(path.display()).await?;
        }
    }

    Ok(())
}

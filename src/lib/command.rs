use crate::{format, helper::*, log, NowPlaying};
use reqwest::Client;
use std::{error::Error, fs, path::Path};

pub async fn add(path: impl ToString) -> Result<(), Box<dyn Error>> {
    if fs::metadata(path.to_string())?.is_dir() {
        let files = fs::read_dir(path.to_string())?;
        for file in files {
            let path = file?.path();
            add_file(path.display()).await?;
        }
    } else {
        add_file(path).await?;
    }

    Ok(())
}

pub async fn play() -> Result<String, Box<dyn Error>> {
    Client::new().get(format::url("C_PP")).send().await?;
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
    Client::new().get(format::url("C_STOP")).send().await?;
    let np = NowPlaying::new().await?;
    let res = format!("STOPPED | {} by {}", np.title, np.artist,);

    Ok(res)
}

pub async fn next() -> Result<String, Box<dyn Error>> {
    let old = NowPlaying::new().await?;
    Client::new().get(format::url("C_NEXT")).send().await?;
    let np = NowPlaying::new().await?;
    let res = format!(
        "SKIPPED | {} by {}\nPLAYING | {} by {}",
        old.title, old.artist, np.title, np.artist
    );

    Ok(res)
}

pub async fn previous() -> Result<String, Box<dyn Error>> {
    let old = NowPlaying::new().await?;
    Client::new().get(format::url("C_PREV")).send().await?;
    let np = NowPlaying::new().await?;
    let res = format!(
        "SKIPPED | {} by {}\nPLAYING | {} by {}",
        old.title, old.artist, np.title, np.artist
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
        "Changed volume to {}%",
        NowPlaying::new().await?.volume * 100.0
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

    Ok("Seeked!".to_string())
}

async fn add_file(path: impl ToString) -> Result<(), Box<dyn Error>> {
    let absolute_path = fs::canonicalize(path.to_string())?;
    let absolute = absolute_path.to_str().unwrap().trim_start_matches(r"\\?\");
    let encoded = urlencoding::encode(absolute);

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

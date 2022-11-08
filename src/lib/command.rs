use crate::{
    format, glyphs::*, helper::*, log, NowPlaying, PlayingStatus, ShuffleStatus, VALID_FORMATS,
};
use owo_colors::OwoColorize;
use reqwest::Client;
use std::{error::Error, fs, path::Path};

pub async fn add(path: impl ToString) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    if fs::metadata(path.to_string())?.is_dir() {
        add_directory(&client, path).await?;
    } else {
        add_file(&client, path).await?;
    }

    Ok(())
}

pub async fn clear() -> Result<String, Box<dyn Error>> {
    reqwest::get(format::url("CLEAR")).await?;
    let res = format!("{} Cleared queue", CLEAR.red());
    Ok(res)
}

pub async fn play() -> Result<String, Box<dyn Error>> {
    reqwest::get(format::url("C_PP")).await?;
    let np = NowPlaying::new().await?;
    let res = if np.playing == Some(PlayingStatus::Playing) {
        format!("{} {} by {}", PLAY.green(), np.title.bold(), np.artist)
    } else {
        format!("{} {} by {}", PAUSE.red(), np.title.bold(), np.artist)
    };

    Ok(res)
}

pub async fn stop() -> Result<String, Box<dyn Error>> {
    reqwest::get(format::url("C_STOP")).await?;
    let np = NowPlaying::new().await?;
    let res = format!("{} {} by {}", STOP.red(), np.title.bold(), np.artist,);

    Ok(res)
}

pub async fn next() -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let old = NowPlaying::with(&client).await?;
    client.get(format::url("C_NEXT")).send().await?;
    let np = NowPlaying::with(&client).await?;
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
    let client = Client::new();
    let old = NowPlaying::with(&client).await?;
    client.get(format::url("C_PREV")).send().await?;
    let np = NowPlaying::with(&client).await?;

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

pub async fn volume(amount: Option<impl ToString>) -> Result<String, Box<dyn Error>> {
    let res = match amount {
        Some(amount) => {
            let client = Client::new();
            client
                .get(format::url_path(
                    "C_VOL",
                    parse_volume(amount.to_string()).await?,
                ))
                .send()
                .await?;
            format!(
                "Volume set to {}%",
                (NowPlaying::with(&client).await?.volume * 100.0).bold()
            )
        }
        None => {
            let np = NowPlaying::new().await?;
            let volume = ((np.volume * 100.0) as u8).to_string();

            return Ok(volume);
        }
    };

    Ok(res)
}

pub async fn seek(amount: impl ToString) -> Result<String, Box<dyn Error>> {
    reqwest::get(format::url_path(
        "C_SEEK",
        parse_position(amount.to_string()).await?,
    ))
    .await?;
    let res = if amount.to_string().ends_with('%') {
        format!("Set position to {}", amount.to_string().bold())
    } else {
        format!("Seeked {} seconds", amount.to_string().bold())
    };

    Ok(res)
}

pub async fn shuffle(mut status: Option<ShuffleStatus>) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    if let None | Some(ShuffleStatus::Toggle) = status {
        let current_status = NowPlaying::with(&client).await?.shuffle;
        status = Some(ShuffleStatus::from(!current_status));
    };

    let res = match status {
        Some(ShuffleStatus::On) => {
            client.get(format::url_path("C_SHUF", 1)).send().await?;
            "Turned shuffle ON".to_string()
        }
        Some(ShuffleStatus::Off) => {
            client.get(format::url_path("C_SHUF", 0)).send().await?;
            "Turned shuffle OFF".to_string()
        }
        _ => String::from("???"),
    };

    Ok(res)
}

async fn add_file(c: &Client, path: impl ToString) -> Result<(), Box<dyn Error>> {
    let absolute_path = fs::canonicalize(path.to_string())?
        .to_string_lossy()
        .to_string();
    let encoded = urlencoding::encode(absolute_path.trim_end_matches(r"\\?\"));

    log::info(format!(
        "Adding {:?}",
        Path::file_name(Path::new(&path.to_string())).unwrap()
    ));
    // returns an error when it shouldnt so just ignore error lole, https://github.com/hyperium/hyper/issues/2136
    _ = c.get(format::url_path("ADDITEM", &encoded)).send().await;

    Ok(())
}

async fn add_directory(c: &Client, path: impl ToString) -> Result<(), Box<dyn Error>> {
    for file in fs::read_dir(path.to_string())? {
        let path = file?.path();
        let ext = match &path.extension() {
            Some(ext) => ext.to_str().unwrap(),
            None => continue,
        };
        if VALID_FORMATS.contains(&ext) {
            add_file(c, path.display()).await?;
        }
    }

    Ok(())
}

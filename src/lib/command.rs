use std::path::Path;

use owo_colors::OwoColorize;
use reqwest::Client;

use crate::*;

pub async fn add(path: impl ToString) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    if std::fs::metadata(path.to_string())?.is_dir() {
        add_directory(&client, path).await?;
    } else {
        add_file(&client, path).await?;
    }

    Ok(())
}

pub async fn clear() -> Result<String, Box<dyn std::error::Error>> {
    reqwest::get(fmt::url("CLEAR")).await?;
    let res = format!("{} Cleared queue", glyphs::CLEAR.red());
    Ok(res)
}

pub async fn play() -> Result<String, Box<dyn std::error::Error>> {
    reqwest::get(fmt::url("C_PP")).await?;
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
    reqwest::get(fmt::url("C_STOP")).await?;
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
    client.get(fmt::url("C_NEXT")).send().await?;
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
    client.get(fmt::url("C_PREV")).send().await?;
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

pub async fn volume(amount: Option<impl ToString>) -> Result<String, Box<dyn std::error::Error>> {
    let res = match amount {
        Some(amount) => {
            let client = Client::new();
            client
                .get(fmt::url_path(
                    "C_VOL",
                    helper::parse_volume(amount.to_string()).await?,
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
            let volume = format!("{}%", (np.volume * 100.0) as u8);

            return Ok(volume);
        }
    };

    Ok(res)
}

pub async fn seek(amount: impl ToString) -> Result<String, Box<dyn std::error::Error>> {
    reqwest::get(fmt::url_path(
        "C_SEEK",
        helper::parse_position(amount.to_string()).await?,
    ))
    .await?;
    let res = if amount.to_string().ends_with('%') {
        format!("Set position to {}", amount.to_string().bold())
    } else {
        format!("Seeked {} seconds", amount.to_string().bold())
    };

    Ok(res)
}

pub async fn shuffle(
    mut status: Option<ShuffleStatus>,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    if let None | Some(ShuffleStatus::Toggle) = status {
        let current_status = ShuffleStatus::from(NowPlaying::with(&client).await?.shuffle);
        status = Some(ShuffleStatus::toggle(&current_status));
    };

    let res = match status {
        Some(ShuffleStatus::Off) => {
            client.get(fmt::url_path("C_SHUF", 0)).send().await?;
            "Turned shuffle OFF".to_owned()
        }
        Some(ShuffleStatus::On) => {
            client.get(fmt::url_path("C_SHUF", 1)).send().await?;
            "Turned shuffle ON".to_owned()
        }
        _ => unreachable!(),
    };

    Ok(res)
}

pub async fn repeat(
    mut status: Option<RepeatStatus>,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    if let None | Some(RepeatStatus::Toggle) = status {
        let current_status = RepeatStatus::from(&NowPlaying::with(&client).await?.repeat.unwrap());
        status = Some(RepeatStatus::toggle(&current_status));
    };

    let res = match status {
        Some(RepeatStatus::None) => {
            client.get(fmt::url_path("C_REP", 0)).send().await?;
            "Changed loop to OFF".to_string()
        }
        Some(RepeatStatus::All) => {
            client.get(fmt::url_path("C_REP", 1)).send().await?;
            "Changed loop to ALL".to_string()
        }
        Some(RepeatStatus::Single) => {
            client.get(fmt::url_path("C_REP", 2)).send().await?;
            "Changed loop to TRACK".to_string()
        }
        _ => unreachable!(),
    };

    Ok(res)
}

async fn add_file(c: &Client, path: impl ToString) -> Result<(), Box<dyn std::error::Error>> {
    let absolute_path = std::fs::canonicalize(path.to_string())?
        .to_string_lossy()
        .to_string();
    let encoded = urlencoding::encode(absolute_path.trim_end_matches(r"\\?\"));

    log::info(format!(
        "Adding \"{}\"",
        Path::file_name(Path::new(&path.to_string()))
            .unwrap()
            .to_string_lossy()
    ));
    // returns an error when it shouldnt so just ignore error lole, https://github.com/hyperium/hyper/issues/2136
    _ = c.get(fmt::url_path("ADDITEM", &encoded)).send().await;

    Ok(())
}

async fn add_directory(c: &Client, path: impl ToString) -> Result<(), Box<dyn std::error::Error>> {
    for file in std::fs::read_dir(path.to_string())? {
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

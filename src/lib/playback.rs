use crate::{format, glyphs, helper::*, NowPlaying};
use owo_colors::OwoColorize;
use std::{cmp::max, error::Error};

pub async fn nowplaying() -> Result<String, Box<dyn Error>> {
    let np = NowPlaying::new().await?;
    let (pos, total) = (np.position, np.duration);
    let res = format!(
        "{} {} {}\n{}",
        parse_duration(pos),
        pb(pos, total),
        parse_duration(total),
        format::info(np),
    );

    Ok(res)
}

pub async fn queue() -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let body = client.get(format::url("PL")).send().await?.text().await?;
    let queue: Vec<NowPlaying> = serde_json::from_str(&body)?;
    let np = NowPlaying::with(&client).await?;
    let mut res = String::new();
    for (i, track) in queue.iter().enumerate() {
        let prefix = if np.file == track.file {
            format!("{} ", glyphs::PLAY.green())
        } else {
            format!("{}.", i + 1)
        };

        res += &format!("{} {} by {}\n", prefix, track.title.bold(), track.artist);
    }

    Ok(res.trim_end().to_string())
}

fn pb(pos: u32, total: u32) -> String {
    const BAR_LENGTH: usize = 25;
    let (c, t) = (pos as usize, total as usize);
    let p = if c == 0 {
        c
    } else {
        max((c * BAR_LENGTH) / t, 1)
    };
    let l = BAR_LENGTH - p;

    format!("{}{}", "━".repeat(p).red(), "━".repeat(l).bright_black())
}

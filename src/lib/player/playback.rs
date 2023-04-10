use miette::IntoDiagnostic;
use owo_colors::OwoColorize;

use crate::{fmt, glyphs, helper::parse_duration, url, NowPlaying, Playlist};

pub async fn nowplaying() -> miette::Result<String> {
    let np = NowPlaying::new().await?;
    let (pos, total) = (np.position, np.duration);
    let res = format!(
        "{} {} {}\n{}",
        parse_duration(pos),
        pb(pos, total),
        parse_duration(total),
        fmt::info(&np),
    );

    Ok(res)
}

pub async fn queue() -> miette::Result<String> {
    let client = reqwest::Client::new();
    let queue = client
        .get(url!("PL"))
        .send()
        .await
        .into_diagnostic()?
        .json::<Playlist>()
        .await
        .into_diagnostic()?;
    let np = NowPlaying::with_client(&client).await?;
    let res = queue
        .iter()
        .enumerate()
        .map(|(i, track)| {
            let prefix = if np.file == track.file {
                format!("{} ", glyphs::PLAY.green())
            } else {
                format!("{i}.")
            };

            format!("{} {} by {}\n", prefix, track.title.bold(), track.artist)
        })
        .collect::<String>();

    Ok(res.trim_end().to_owned())
}

fn pb(pos: i32, total: i32) -> String {
    const BAR_LENGTH: usize = 25;
    let (c, t) = (pos as usize, total as usize);
    let p = if c == 0 {
        c
    } else {
        std::cmp::max((c * BAR_LENGTH) / t, 1)
    };
    let l = BAR_LENGTH - p;

    format!("{}{}", "━".repeat(p).red(), "━".repeat(l).bright_black())
}

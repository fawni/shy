use lofty::{Accessor, AudioFile, TaggedFileExt};
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
    let np = NowPlaying::with(&client).await?;
    let res = queue
        .iter()
        .enumerate()
        .map(|(i, track)| {
            let prefix = if np.file == track.file {
                format!("{} ", glyphs::PLAY.green())
            } else {
                format!("{i:02}.")
            };

            if i == 0 {
                let (pos, total) = (np.position, np.duration);
                format!(
                    "{} {} :: {} | {} {}%\n",
                    prefix,
                    track.title.bold().cyan(),
                    track.artist.purple(),
                    pb(pos, total),
                    (pos as f32 / total as f32 * 100.0) as u32
                )
            } else {
                format!(
                    "{} {} :: {}\n",
                    prefix,
                    track.title.bold().cyan(),
                    track.artist.purple()
                )
            }
        })
        .collect::<String>();

    Ok(res.trim_end().to_owned())
}

pub async fn info(track: Option<String>) -> miette::Result<String> {
    let track = if let Some(track) = track {
        track
    } else {
        NowPlaying::new().await?.file
    };

    let tagged_file = lofty::read_from_path(track).into_diagnostic()?;
    let tag = tagged_file.primary_tag().unwrap();
    let properties = tagged_file.properties();

    let duration = properties.duration().as_secs();
    let h = (duration / 60) / 60;
    let m = (duration / 60) % 60;
    let s = duration % 60;
    let time = if h == 0 {
        format!("{m:02}:{s:02}")
    } else {
        format!("{h:02}:{m:02}:{s:02}")
    };

    Ok(format!(
        "{} ({} of {})\n{}\n{} ({})\n{:?} {} kHz, {}k, {}",
        tag.title().unwrap_or_else(|| "-".into()).bold(),
        tag.track().unwrap_or(0),
        tag.track_total().unwrap_or(0),
        tag.artist().unwrap_or_else(|| "-".into()),
        tag.album().unwrap_or_else(|| "-".into()),
        tag.year().unwrap_or(0),
        tagged_file.file_type(),
        properties.sample_rate().unwrap() as f32 / 1000.0,
        properties.audio_bitrate().unwrap(),
        time
    ))
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

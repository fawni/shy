use crate::{format, helper::*, NowPlaying};
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

pub async fn volume() -> Result<String, Box<dyn Error>> {
    let np = NowPlaying::new().await?;
    let volume = ((np.volume * 100.0) as u32).to_string();

    Ok(volume)
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

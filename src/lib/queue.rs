use crate::{helper::parse_duration, NowPlaying};
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
        format_info(np),
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
    let p = max((c * BAR_LENGTH) / t, 1);
    let l = BAR_LENGTH - p;

    format!("{}{}", "━".repeat(p).red(), "━".repeat(l).bright_black())
}

fn format_info(np: NowPlaying) -> String {
    let t_h = np.duration / 1000 / 60 / 60;
    let p_h = np.position / 1000 / 60 / 60;

    // guhhhhhhhhhhhhhhh
    if t_h > 0 {
        if p_h > 0 {
            format!(
                "{:^43.43}\n{:^43.43}",
                np.title.bold(),
                np.artist.bright_black()
            )
        } else {
            format!(
                "{:^40.40}\n{:^40.40}",
                np.title.bold(),
                np.artist.bright_black()
            )
        }
    } else {
        format!(
            "{:^37.37}\n{:^37.37}",
            np.title.bold(),
            np.artist.bright_black()
        )
    }
}

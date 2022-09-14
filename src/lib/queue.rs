use crate::NowPlaying;
use owo_colors::OwoColorize;
use std::error::Error;

pub async fn nowplaying() -> Result<String, Box<dyn Error>> {
    let np = NowPlaying::get().await?;
    let current = np.position;
    let total = np.duration;

    let res = format!(
        "{}\n{}\n{} / {}",
        np.title.bold(),
        np.artist.bright_black(),
        parse_duration(current),
        parse_duration(total),
    );

    Ok(res)
}

pub async fn volume() -> Result<String, Box<dyn Error>> {
    let np = NowPlaying::get().await?;
    let volume = ((np.volume * 100.0) as u32).to_string();

    Ok(volume)
}

fn parse_duration(mut d: u32) -> String {
    d /= 1000;
    match (d / 60) / 60 {
        0 => format!("{:02}:{:02}", ((d / 60) % 60), (d % 60)),
        _ => format!(
            "{:02}:{:02}:{:02}",
            ((d / 60) / 60),
            ((d / 60) % 60),
            (d % 60)
        ),
    }
}

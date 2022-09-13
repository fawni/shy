use crate::np;
use chrono::Duration;
use owo_colors::OwoColorize;
use std::error::Error;

pub async fn nowplaying() -> Result<String, Box<dyn Error>> {
    let np = np().await?;
    let current = chrono::Duration::milliseconds(np.position as i64);
    let total = chrono::Duration::milliseconds(np.duration as i64);

    let res = format!(
        "{}\n{}\n{} / {}",
        np.title.bold(),
        np.artist.bright_black(),
        parse_duration(current),
        parse_duration(total),
    );

    Ok(res)
}

fn parse_duration(d: Duration) -> String {
    match d.num_hours() {
        0 => format!("{:02}:{:02}", d.num_minutes() % 60, d.num_seconds() % 60),
        _ => format!(
            "{:02}:{:02}:{:02}",
            d.num_hours(),
            d.num_minutes() % 60,
            d.num_seconds() % 60
        ),
    }
}

use owo_colors::OwoColorize;

use crate::{NowPlaying, API_BASE};

pub fn url(endpoint: impl ToString) -> String {
    format!("{}/{}", *API_BASE, endpoint.to_string())
}

pub fn url_path(endpoint: impl ToString, path: &impl ToString) -> String {
    format!(
        "{}/{}?{}",
        *API_BASE,
        endpoint.to_string(),
        path.to_string()
    )
}

pub(crate) fn info(np: &NowPlaying) -> String {
    let total_hours = np.duration / 1000 / 60 / 60;
    let played_hours = np.position / 1000 / 60 / 60;

    if total_hours > 0 {
        if played_hours > 0 {
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

use crate::{NowPlaying, MUSICBEE_REST_URL};
use owo_colors::OwoColorize;

pub(crate) fn info(np: NowPlaying) -> String {
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

pub(crate) fn url(endpoint: impl ToString) -> String {
    format!("{}/{}", MUSICBEE_REST_URL, endpoint.to_string())
}

pub(crate) fn url_path(endpoint: impl ToString, path: impl ToString) -> String {
    format!(
        "{}/{}?{}",
        MUSICBEE_REST_URL,
        endpoint.to_string(),
        path.to_string()
    )
}

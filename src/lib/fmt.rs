use owo_colors::OwoColorize;

use crate::NowPlaying;

#[macro_export]
macro_rules! url {
    ($e:literal) => {
        format!("{}/{}", $crate::API_BASE.get().await, $e)
    };
    ($e:expr, $p:expr) => {
        format!("{}/{}?{}", $crate::API_BASE.get().await, $e, $p)
    };
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

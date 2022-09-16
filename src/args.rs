use clap::{Arg, Command};

pub fn get_app() -> Command<'static> {
    Command::new("shy")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(
            Command::new("add")
                .about("Add a track to queue")
                .visible_alias("queue")
                .visible_alias("a")
                .arg(Arg::new("track").takes_value(true).multiple_values(true)),
        )
        .subcommand(
            Command::new("play")
                .about("Play/pause the current track")
                .visible_alias("pause")
                .visible_alias("p"),
        )
        .subcommand(
            Command::new("stop")
                .about("Stop playback")
                .visible_alias("s"),
        )
        .subcommand(
            Command::new("next")
                .about("Play the next track in the queue")
                .visible_alias("n"),
        )
        .subcommand(
            Command::new("previous")
                .about("Play the previous track in the queue")
                .visible_alias("b"),
        )
        .subcommand(
            Command::new("nowplaying")
                .about("Print information about the current track")
                .visible_alias("np"),
        )
        .subcommand(
            Command::new("volume")
                .about("Modify player volume")
                .visible_alias("vol")
                .visible_alias("v")
                .arg(
                    Arg::new("amount")
                        .takes_value(true)
                        .allow_hyphen_values(true),
                ),
        )
        .subcommand(
            Command::new("seek").about("Seek playback").arg(
                Arg::new("amount")
                    .required(true)
                    .takes_value(true)
                    .allow_hyphen_values(true),
            ),
        )
}

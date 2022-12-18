use clap::{builder::PossibleValuesParser, Arg, Command};

pub fn get_app() -> Command {
    Command::new("shy")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(
            Command::new("add")
                .about("Add a track to queue")
                .visible_alias("a")
                .arg(Arg::new("track").value_delimiter(' ')),
        )
        .subcommand(
            Command::new("clear")
                .about("Clear current queue")
                .visible_alias("c"),
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
                .visible_alias("prev")
                .visible_alias("b"),
        )
        .subcommand(
            Command::new("nowplaying")
                .about("Print information about the current track")
                .visible_alias("np"),
        )
        .subcommand(
            Command::new("queue")
                .about("List queued tracks")
                .visible_alias("q"),
        )
        .subcommand(
            Command::new("volume")
                .about("Modify player volume")
                .visible_alias("vol")
                .visible_alias("v")
                .arg(Arg::new("amount").allow_hyphen_values(true)),
        )
        .subcommand(
            Command::new("seek")
                .about("Seek playback")
                .arg(Arg::new("amount").required(true).allow_hyphen_values(true)),
        )
        .subcommand(
            Command::new("shuffle")
                .about("Change shuffle status")
                .arg(Arg::new("status").value_parser(PossibleValuesParser::new(["on", "off"]))),
        )
}

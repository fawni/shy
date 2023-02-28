use clap::{builder::PossibleValuesParser, command, Arg, Command};

pub fn get_app() -> Command {
    command!()
        .subcommand(
            Command::new("add")
                .about("Add a track to queue")
                .visible_alias("a")
                .arg(Arg::new("track").num_args(1..)),
        )
        .subcommand(
            Command::new("clear")
                .about("Clear current queue")
                .visible_alias("c"),
        )
        .subcommand(
            Command::new("play")
                .about("Play/pause the current track")
                .visible_aliases(["pause", "p"]),
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
                .visible_aliases(["prev", "b"]),
        )
        .subcommand(
            Command::new("nowplaying")
                .about("Print information about the current track")
                .visible_alias("np"),
        )
        .subcommand(
            Command::new("queue")
                .about("List queued tracks")
                .visible_aliases(["q", "list", "ls", "l"]),
        )
        .subcommand(
            Command::new("volume")
                .about("Modify player volume")
                .visible_aliases(["vol", "v"])
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
        .subcommand(
            Command::new("repeat")
                .about("Change repeat status")
                .visible_aliases(["loop", "r"])
                .arg(Arg::new("status").value_parser(PossibleValuesParser::new([
                    "none", "off", "single", "track", "one", "all", "queue", "on",
                ]))),
        )
}

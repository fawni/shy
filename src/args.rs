use clap::{builder::PossibleValuesParser, Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, propagate_version = true, arg_required_else_help(true))]
pub struct ShyArgs {
    #[command(subcommand)]
    pub command: ShyCommand,
}

#[derive(Subcommand)]
pub enum ShyCommand {
    Play(Play),
    Stop(Stop),
    Next(Next),
    Previous(Previous),
    Add(Add),
    Clear(Clear),
    NowPlaying(NowPlaying),
    Queue(Queue),
    Volume(Volume),
    Seek(Seek),
    Shuffle(Shuffle),
    Repeat(Repeat),
}

#[derive(Args)]
#[command(about = "Play/Pause the current track", visible_aliases = ["pause", "p"])]
pub struct Play {}

#[derive(Args)]
#[command(about = "Stop playback", visible_aliases = ["s"])]
pub struct Stop {}

#[derive(Args)]
#[command(about = "Play the next track in the queue", visible_aliases = ["n"])]
pub struct Next {}

#[derive(Args)]
#[command(about = "Play the previous track in the queue", visible_aliases = ["prev", "b"])]
pub struct Previous {}

#[derive(Args)]
#[command(about = "Add track(s) to queue", visible_aliases = ["a"])]
pub struct Add {
    #[arg(long, short, num_args = 0)]
    pub next: bool,
    #[arg(required = true, num_args = 1..)]
    pub tracks: Vec<String>,
}

#[derive(Args)]
#[command(about = "Clear current queue", visible_aliases = ["c"])]
pub struct Clear {}

#[derive(Args)]
#[command(about = "Print information about the current track", visible_aliases = ["np"])]
pub struct NowPlaying {}

#[derive(Args)]
#[command(about = "List queued tracks", visible_aliases = ["q", "list", "ls", "l"])]
pub struct Queue {}

#[derive(Args)]
#[command(about = "Modify player volume", visible_aliases = ["vol", "v"])]
pub struct Volume {
    #[arg(allow_hyphen_values = true)]
    pub amount: Option<String>,
}

#[derive(Args)]
#[command(about = "Seek playback")]
pub struct Seek {
    #[arg(required = true, allow_hyphen_values = true)]
    pub amount: String,
}

#[derive(Args)]
#[command(about = "Change shuffle status")]
pub struct Shuffle {
    #[arg(value_parser = PossibleValuesParser::new(["on", "off"]))]
    pub status: Option<String>,
}

#[derive(Args)]
#[command(about = "Change repeat status", visible_aliases = ["loop", "r"])]
pub struct Repeat {
    #[arg(value_parser = PossibleValuesParser::new(["none", "off", "single", "track", "one", "all", "queue", "on",]))]
    pub status: Option<String>,
}

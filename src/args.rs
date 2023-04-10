use clap::{Args, Parser, Subcommand};
use shy::{RepeatMode, ShuffleMode};

/// A command line remote controller for MusicBee
#[derive(Parser)]
#[command(arg_required_else_help = true)]
pub struct ShyArgs {
    #[command(subcommand)]
    pub command: ShyCommand,
}

#[derive(Subcommand)]
pub enum ShyCommand {
    Start(Start),
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

/// Start the MusicBee player
#[derive(Args)]
pub struct Start {}

/// Play or pause the current track
#[derive(Args)]
#[command(visible_aliases = ["pause", "p"])]
pub struct Play {}

/// Stop the current playing track
#[derive(Args)]
#[command(visible_aliases = ["s"])]
pub struct Stop {}

/// Skip to the next track in queue
#[derive(Args)]
#[command(visible_aliases = ["skip", "n"])]
pub struct Next {}

/// Skip to the previous track in queue
#[derive(Args)]
#[command(visible_aliases = ["prev", "b"])]
pub struct Previous {}

/// Add track(s) to queue
#[derive(Args)]
#[command(visible_aliases = ["a"])]
pub struct Add {
    /// Add track(s) to the front of the queue
    #[arg(long, short, num_args = 0)]
    pub next: bool,

    /// Track(s) to add to the queue
    #[arg(required = true, num_args = 1..)]
    pub tracks: Vec<String>,
}

/// Clear the current queue
#[derive(Args)]
#[command(visible_aliases = ["c"])]
pub struct Clear {}

/// Display the current playing track
#[derive(Args)]
#[command(visible_aliases = ["np"])]
pub struct NowPlaying {}

/// Display the current queue
#[derive(Args)]
#[command(visible_aliases = ["q", "list", "ls", "l"])]
pub struct Queue {}

/// Display or modify the player's volume
#[derive(Args)]
#[command(visible_aliases = ["vol", "v"])]
pub struct Volume {
    /// Volume amount to set. Can be negative. If not provided, the current volume will be displayed
    #[arg(allow_hyphen_values = true)]
    pub amount: Option<String>,
}

/// Set the position of the track
#[derive(Args)]
pub struct Seek {
    /// Position to seek to. Can be in seconds or a percentage. Negative values seek backwards
    #[arg(required = true, allow_hyphen_values = true)]
    pub position: String,
}

/// Set shuffle mode
#[derive(Args)]
pub struct Shuffle {
    /// Shuffle mode to set. If not provided, the current shuffle mode will toggle between on and off
    #[arg(value_enum)]
    pub mode: Option<ShuffleMode>,
}

/// Set repeat mode
#[derive(Args)]
#[command(visible_aliases = ["loop", "r"])]
pub struct Repeat {
    /// Repeat mode to set. If not provided, the current repeat mode will toggle to the next mode
    #[arg(value_enum)]
    pub mode: Option<RepeatMode>,
}

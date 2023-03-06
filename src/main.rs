use clap::ArgMatches;
use owo_colors::OwoColorize;
use shy::{info, player::command, player::playback, RepeatStatus, ShuffleStatus};

mod args;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if cfg!(windows) {
        enable_windows_virtual_terminal();
    }

    match args::get_app().get_matches().subcommand() {
        Some(("play", _)) => play().await,
        Some(("stop", _)) => stop().await,
        Some(("next", _)) => next().await,
        Some(("previous", _)) => previous().await,
        Some(("nowplaying", _)) => now_playing().await,
        Some(("queue", _)) => queue().await,
        Some(("clear", _)) => clear().await,
        Some(("add", cmd)) => add(cmd).await,
        Some(("seek", cmd)) => seek(cmd).await,
        Some(("volume", cmd)) => volume(cmd).await,
        Some(("shuffle", cmd)) => shuffle(cmd).await,
        Some(("repeat", cmd)) => repeat(cmd).await,
        None => Ok(args::get_app().print_help()?),
        _ => unreachable!(),
    }
}

async fn add(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let paths = matches.get_many::<String>("track(s)").unwrap();
    let next = matches.get_one::<bool>("next").unwrap_or(&false);
    for path in paths {
        command::add(path, *next).await?;
    }
    Ok(())
}

async fn clear() -> Result<(), Box<dyn std::error::Error>> {
    let res = command::clear().await?;
    Ok(println!("{res}"))
}

async fn now_playing() -> Result<(), Box<dyn std::error::Error>> {
    let res = playback::nowplaying().await?;
    Ok(println!("{res}"))
}

async fn queue() -> Result<(), Box<dyn std::error::Error>> {
    let res = playback::queue().await?;
    Ok(println!("{res}"))
}

async fn play() -> Result<(), Box<dyn std::error::Error>> {
    let res = command::play().await?;
    Ok(println!("{res}"))
}

async fn stop() -> Result<(), Box<dyn std::error::Error>> {
    let res = command::stop().await?;
    Ok(println!("{res}"))
}

async fn next() -> Result<(), Box<dyn std::error::Error>> {
    let res = command::next().await?;
    Ok(println!("{res}"))
}

async fn previous() -> Result<(), Box<dyn std::error::Error>> {
    let res = command::previous().await?;
    Ok(println!("{res}"))
}

async fn volume(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let amount = matches.get_one::<String>("amount");
    let res = command::volume(amount).await?;
    Ok(info!("{res}"))
}

async fn seek(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let amount = matches.get_one::<String>("amount").unwrap();
    let res = command::seek(amount).await?;
    Ok(info!("{res}"))
}

async fn shuffle(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let status = matches.get_one::<String>("status").map(ShuffleStatus::from);
    let res = command::shuffle(status).await?;
    Ok(info!("{res}"))
}

async fn repeat(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let status = matches.get_one::<String>("status").map(RepeatStatus::from);
    let res = command::repeat(status).await?;
    Ok(info!("{res}"))
}

#[cfg(target_os = "windows")]
fn enable_windows_virtual_terminal() {
    use winapi::{
        shared::minwindef::DWORD,
        um::{
            consoleapi::{GetConsoleMode, SetConsoleMode},
            processenv::GetStdHandle,
            winbase::STD_OUTPUT_HANDLE,
            wincon::ENABLE_VIRTUAL_TERMINAL_PROCESSING,
        },
    };

    unsafe {
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);
        let mut original_mode: DWORD = 0;

        GetConsoleMode(handle, &mut original_mode);
        SetConsoleMode(handle, ENABLE_VIRTUAL_TERMINAL_PROCESSING | original_mode)
    };
}

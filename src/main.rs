use clap::ArgMatches;
use shy::{command, log, playback, RepeatStatus, ShuffleStatus};

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
        None => {
            args::get_app().print_help()?;
            Ok(())
        }
        _ => unreachable!(),
    }
}

async fn add(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let paths = matches.get_many::<String>("track").unwrap();
    for path in paths {
        command::add(path).await?;
    }
    Ok(())
}

async fn clear() -> Result<(), Box<dyn std::error::Error>> {
    let res = command::clear().await?;
    println!("{res}");
    Ok(())
}

async fn now_playing() -> Result<(), Box<dyn std::error::Error>> {
    let res = playback::nowplaying().await?;
    println!("{res}");
    Ok(())
}

async fn queue() -> Result<(), Box<dyn std::error::Error>> {
    let res = playback::queue().await?;
    println!("{res}");
    Ok(())
}

async fn play() -> Result<(), Box<dyn std::error::Error>> {
    let res = command::play().await?;
    println!("{res}");
    Ok(())
}

async fn stop() -> Result<(), Box<dyn std::error::Error>> {
    let res = command::stop().await?;
    println!("{res}");
    Ok(())
}

async fn next() -> Result<(), Box<dyn std::error::Error>> {
    let res = command::next().await?;
    println!("{res}");
    Ok(())
}

async fn previous() -> Result<(), Box<dyn std::error::Error>> {
    let res = command::previous().await?;
    println!("{res}");
    Ok(())
}

async fn volume(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let amount = matches.get_one::<String>("amount");
    let res = command::volume(amount).await?;
    log::info(res);
    Ok(())
}

async fn seek(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let amount = matches.get_one::<String>("amount").unwrap();
    let res = command::seek(amount).await?;
    log::info(res);
    Ok(())
}

async fn shuffle(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let status = matches.get_one::<String>("status").map(ShuffleStatus::from);
    let res = command::shuffle(status).await?;
    log::info(res);
    Ok(())
}

async fn repeat(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let status = matches.get_one::<String>("status").map(RepeatStatus::from);
    let res = command::repeat(status).await?;
    log::info(res);
    Ok(())
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

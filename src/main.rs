use args::{Add, Repeat, Seek, Shuffle, ShyCommand, Volume};
use clap::Parser;
use owo_colors::OwoColorize;

use shy::{
    info,
    player::{command, playback},
};

mod args;

#[tokio::main]
async fn main() -> miette::Result<()> {
    #[cfg(windows)]
    enable_windows_virtual_terminal();

    match args::ShyArgs::parse().command {
        ShyCommand::Play(_) => play().await,
        ShyCommand::Stop(_) => stop().await,
        ShyCommand::Next(_) => next().await,
        ShyCommand::Previous(_) => previous().await,
        ShyCommand::NowPlaying(_) => now_playing().await,
        ShyCommand::Queue(_) => queue().await,
        ShyCommand::Clear(_) => clear().await,
        ShyCommand::Add(args) => add(args).await,
        ShyCommand::Volume(args) => volume(args).await,
        ShyCommand::Seek(args) => seek(args).await,
        ShyCommand::Shuffle(args) => shuffle(args).await,
        ShyCommand::Repeat(args) => repeat(args).await,
    }
}

async fn add(add_args: Add) -> miette::Result<()> {
    let paths = add_args.tracks;
    let next = add_args.next;
    for path in paths {
        command::add(&path, next).await?;
    }

    Ok(())
}

async fn clear() -> miette::Result<()> {
    let res = command::clear().await?;

    Ok(println!("{res}"))
}

async fn now_playing() -> miette::Result<()> {
    let res = playback::nowplaying().await?;

    Ok(println!("{res}"))
}

async fn queue() -> miette::Result<()> {
    let res = playback::queue().await?;

    Ok(println!("{res}"))
}

async fn play() -> miette::Result<()> {
    let res = command::play().await?;

    Ok(println!("{res}"))
}

async fn stop() -> miette::Result<()> {
    let res = command::stop().await?;

    Ok(println!("{res}"))
}

async fn next() -> miette::Result<()> {
    let res = command::next().await?;

    Ok(println!("{res}"))
}

async fn previous() -> miette::Result<()> {
    let res = command::previous().await?;

    Ok(println!("{res}"))
}

async fn volume(volume_args: Volume) -> miette::Result<()> {
    let amount = volume_args.amount;
    let res = command::volume(amount).await?;

    Ok(info!("Volume: {}%", res.bold()))
}

async fn seek(seek_args: Seek) -> miette::Result<()> {
    let amount = seek_args.position;
    let res = command::seek(amount).await?;

    Ok(info!("{res}"))
}

async fn shuffle(shuffle_args: Shuffle) -> miette::Result<()> {
    let mode = shuffle_args.mode;
    let res = command::shuffle(mode).await?;

    Ok(info!("Shuffle: {}", res.bold()))
}

async fn repeat(repeat_args: Repeat) -> miette::Result<()> {
    let mode = repeat_args.mode;
    let res = command::repeat(mode).await?;

    Ok(info!("Repeat: {}", res.bold()))
}

#[cfg(windows)]
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

use clap::ArgMatches;
use shy::{control, queue};
use std::error::Error;

mod args;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    match args::get_app().get_matches().subcommand() {
        Some(("play", _)) => play().await,
        Some(("stop", _)) => stop().await,
        Some(("next", _)) => next().await,
        Some(("previous", _)) => previous().await,
        Some(("nowplaying", _)) => now_playing().await,
        Some(("volume", cmd)) => parse_volume(cmd).await,
        _ => {
            args::get_app().print_help()?;
            println!();
            Err("Invalid command".into())
        }
    }
}

async fn play() -> Result<(), Box<dyn Error>> {
    let res = control::play().await?;
    println!("{}", res);
    Ok(())
}

async fn stop() -> Result<(), Box<dyn Error>> {
    control::stop().await?;
    Ok(())
}

async fn next() -> Result<(), Box<dyn Error>> {
    control::next().await?;
    Ok(())
}

async fn previous() -> Result<(), Box<dyn Error>> {
    control::previous().await?;
    Ok(())
}

async fn parse_volume(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    control::volume(matches.value_of("amount").unwrap_or_default()).await?;
    Ok(())
}

async fn now_playing() -> Result<(), Box<dyn Error>> {
    let res = queue::nowplaying().await?;
    println!("{}", res);
    Ok(())
}

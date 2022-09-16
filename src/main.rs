use clap::ArgMatches;
use shy::{command, queue};
use std::error::Error;

mod args;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    match args::get_app().get_matches().subcommand() {
        Some(("nowplaying", _)) => now_playing().await,
        Some(("play", _)) => play().await,
        Some(("stop", _)) => stop().await,
        Some(("next", _)) => next().await,
        Some(("previous", _)) => previous().await,
        Some(("volume", cmd)) => volume(cmd).await,
        Some(("seek", cmd)) => seek(cmd).await,
        None => {
            args::get_app().print_help()?;
            Ok(())
        }
        _ => Err("invalid command.".into()),
    }
}

async fn now_playing() -> Result<(), Box<dyn Error>> {
    let res = queue::nowplaying().await?;
    println!("{}", res);
    Ok(())
}

async fn play() -> Result<(), Box<dyn Error>> {
    let res = command::play().await?;
    println!("{}", res);
    Ok(())
}

async fn stop() -> Result<(), Box<dyn Error>> {
    command::stop().await?;
    Ok(())
}

async fn next() -> Result<(), Box<dyn Error>> {
    let res = command::next().await?;
    println!("{}", res);
    Ok(())
}

async fn previous() -> Result<(), Box<dyn Error>> {
    let res = command::previous().await?;
    println!("{}", res);
    Ok(())
}

async fn volume(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let amount = match matches.value_of("amount") {
        Some(amount) => amount,
        None => {
            println!("Current volume: {}", queue::volume().await?);
            return Ok(());
        }
    };
    command::volume(amount).await?;
    Ok(())
}

async fn seek(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let amount = matches.value_of("amount").unwrap();
    command::seek(amount).await?;
    Ok(())
}

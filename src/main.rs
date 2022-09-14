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
        Some(("volume", cmd)) => volume(cmd).await,
        None => {
            args::get_app().print_help()?;
            Ok(())
        }
        _ => Err("invalid command.".into()),
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
    let res = control::next().await?;
    println!("{}", res);
    Ok(())
}

async fn previous() -> Result<(), Box<dyn Error>> {
    let res = control::previous().await?;
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

    control::volume(amount).await?;
    Ok(())
}

async fn now_playing() -> Result<(), Box<dyn Error>> {
    let res = queue::nowplaying().await?;
    println!("{}", res);
    Ok(())
}

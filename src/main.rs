use clap::{Parser, Subcommand};
use std::error::Error;

mod days;
mod utils;

#[derive(Parser)]
#[command(subcommand_required = true)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds favorite command
    Run {
        #[arg(short, long)]
        challenge_id: String,
    },
}

// TODO -> should this not return a std::io::Error?
fn load_challenge(challenge_id: String) -> Result<(), Box<dyn Error>> {
    match challenge_id.as_str() {
        // day 1
        "1a" => Ok(days::day1::a::solve()?),
        "1b" => Ok(days::day1::b::solve()?),
        // day 2
        "2a" => Ok(days::day2::a::solve()?),
        "2b" => Ok(days::day2::b::solve()?),
        // day 3
        "3a" => Ok(days::day3::a::solve()?),
        // day 4
        "4a" => Ok(days::day4::a::solve()?),
        "4b" => Ok(days::day4::b::solve()?),
        // day 5
        "5a" => Ok(days::day5::a::solve()?),
        "5b" => Ok(days::day5::b::solve()?),
        // day 6
        "6a" => Ok(days::day6::a::solve()?),
        "6b" => Ok(days::day6::b::solve()?),
        challenge_id_str => {
            Err(format!("Challenge {} invalid or not implemented!", challenge_id_str).into())
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { challenge_id } => load_challenge(challenge_id),
    }
}

use clap::{Parser, Subcommand};
use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Pokemon {
    pub name: String,
    pub height: u32,
    pub weight: u32,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Get a pokemon
    Get { name: String },
}

#[derive(Parser, Debug)]
#[command(version, about = "Fetches Pokémon from PokéAPI.")]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();

    match args.command {
        Command::Get { name } => {
            let url = format!("https://pokeapi.co/api/v2/pokemon/{}", name);
            let response = reqwest::get(&url).await?;

            if response.status().is_success() {
                let pokemon: Pokemon = response.json().await?;
                dbg!(&pokemon);
            } else {
                eprintln!("Error: Could not fetch data for Pokémon '{}'", name);
            }
        }
    };

    Ok(())
}

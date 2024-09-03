use anyhow::Context;
use clap::{Parser, Subcommand};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Pokemon {
    pub name: String,
    pub height: u32,
    pub weight: u32,
}

#[derive(Subcommand)]
pub enum Command {
    /// Get a pokemon
    Get { name: String },
}

#[derive(Parser)]
#[command(version, about = "Fetches Pokémon from PokéAPI.")]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(thiserror::Error, Debug)]
enum GetPokemonError {
    #[error("Pokemon not found")]
    NotFound,
    #[error("Failed to get pokemon: {0}")]
    Other(#[from] anyhow::Error),
}

async fn get_pokemon(name: String) -> Result<Pokemon, GetPokemonError> {
    let url = format!("https://pokeapi.co/api/v2/pokemon/{}", name);
    let response = reqwest::get(&url).await.context("GET request failed")?;

    if response.status().is_success() {
        let pokemon = response
            .json()
            .await
            .context("failed to deserialize pokemon")?;
        Ok(pokemon)
    } else {
        Err(GetPokemonError::NotFound)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Get { name } => {
            let pokemon = get_pokemon(name).await?;
            dbg!(&pokemon);
        }
    };

    Ok(())
}

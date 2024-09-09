use clap::{Parser, Subcommand};
use pokemon_repository::{GetPokemonError, PokemonRepository};

mod pokemon_repository;

#[derive(Subcommand)]
pub enum Command {
    /// Get a pokemon
    Get { name: String },
}

/// Fetches Pokémon from PokéAPI.
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let args = Cli::parse();
    let base_url = "https://pokeapi.co/api/v2/pokemon".to_string();
    let repository = PokemonRepository::new(base_url);

    match args.command {
        Command::Get { name } => {
            let pokemon = repository.get_pokemon(&name).await;
            match pokemon {
                Ok(pkm) => println!("{pkm}"),
                Err(GetPokemonError::NotFound) => println!("pokemon {name} not found"),
                Err(GetPokemonError::Other(err)) => {
                    println!("something went wrong when fetching pokemon {name}: {err}")
                }
            }
        }
    };

    Ok(())
}

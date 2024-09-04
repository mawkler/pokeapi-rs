use clap::{Parser, Subcommand};
use pokemon_repository::PokemonRepository;

mod pokemon_repository;

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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let base_url = "https://pokeapi.co/api/v2/pokemon".to_string();
    let repository = PokemonRepository::new(base_url);

    match args.command {
        Command::Get { name } => {
            let pokemon = repository.get_pokemon(&name).await?;
            println!("{}", pokemon)
        }
    };

    Ok(())
}

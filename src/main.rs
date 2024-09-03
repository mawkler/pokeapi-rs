use clap::{Parser, Subcommand};

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
    let client = reqwest::Client::new();
    let base_url = "https://pokeapi.co/api/v2/pokemon/".to_string();
    let repository = pokemon_repository::PokemonRepository::new(base_url, client);

    match args.command {
        Command::Get { name } => {
            let pokemon = repository.get_pokemon(name).await?;
            dbg!(&pokemon);
        }
    };

    Ok(())
}

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

    match args.command {
        Command::Get { name } => {
            let pokemon = pokemon_repository::get_pokemon(name).await?;
            dbg!(&pokemon);
        }
    };

    Ok(())
}

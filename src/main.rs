use clap::Parser;
use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Pokemon {
    name: String,
    height: u32,
    weight: u32,
}

#[derive(Parser, Debug)]
#[command(version, about = "Fetches Pokémon from PokéAPI.")]
struct Args {
    /// The name of the Pokémon to fetch data for
    #[arg(short, long)]
    pokemon: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();

    let url = format!("https://pokeapi.co/api/v2/pokemon/{}", args.pokemon);
    let response = reqwest::get(&url).await?;

    if response.status().is_success() {
        let pokemon: Pokemon = response.json().await?;
        dbg!(&pokemon);
    } else {
        eprintln!("Error: Could not fetch data for Pokémon '{}'", args.pokemon);
    }

    Ok(())
}

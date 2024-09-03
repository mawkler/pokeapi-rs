use anyhow::Context;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Pokemon {
    pub name: String,
    pub height: u32,
    pub weight: u32,
}

#[derive(thiserror::Error, Debug)]
pub enum GetPokemonError {
    #[error("Pokemon not found")]
    NotFound,
    #[error("Failed to get pokemon: {0}")]
    Other(#[from] anyhow::Error),
}

pub async fn get_pokemon(name: String) -> Result<Pokemon, GetPokemonError> {
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

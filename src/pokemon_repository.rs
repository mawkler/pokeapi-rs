use anyhow::Context;
use serde::Deserialize;

pub struct PokemonRepository {
    base_url: String,
    client: reqwest::Client,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
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

impl PokemonRepository {
    pub fn new(base_url: String, client: reqwest::Client) -> Self {
        Self { base_url, client }
    }

    pub async fn get_pokemon(&self, name: String) -> Result<Pokemon, GetPokemonError> {
        let url = format!("{}/{}", self.base_url, name);
        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("GET request failed")?;

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
}

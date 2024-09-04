use std::fmt::Display;

use anyhow::{anyhow, Context};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Pokemon {
    pub name: String,
    pub height: u32,
    pub weight: u32,
}

impl Display for Pokemon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {} cm, {} hg", self.name, self.height, self.weight)
    }
}

pub struct PokemonRepository {
    pub base_url: String,
}

#[derive(thiserror::Error, Debug)]
pub enum GetPokemonError {
    #[error("Pokemon not found")]
    NotFound,
    #[error("Failed to get pokemon: {0}")]
    Other(#[from] anyhow::Error),
}

impl PokemonRepository {
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }

    pub async fn get_pokemon(&self, name: &str) -> Result<Pokemon, GetPokemonError> {
        let url = format!("{}/{}", self.base_url, name);
        let response = reqwest::get(&url).await.context("GET request failed")?;

        match response.status() {
            reqwest::StatusCode::NOT_FOUND => Err(GetPokemonError::NotFound),
            reqwest::StatusCode::OK => {
                let pokemon = response
                    .json()
                    .await
                    .context("failed to deserialize pokemon")?;
                Ok(pokemon)
            }
            otherwise => Err(anyhow!("failed to GET pokemon: {}", otherwise).into()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::main]
    #[test]
    async fn responds_with_pokemon() {
        // Given

        // Start a local mock HTTP server on a random port
        let mock_server = MockServer::start().await;
        let response = ResponseTemplate::new(200).set_body_json(json!({
            "name": "charmander",
            "height": 6,
            "weight": 85
        }));
        Mock::given(method("GET"))
            .and(path("/charmander"))
            .respond_with(response)
            .mount(&mock_server) // Mount the behaviour on the mock server
            .await;

        let repository = PokemonRepository::new(mock_server.uri());

        // When
        let charmander = repository.get_pokemon("charmander").await.unwrap();

        // Then
        assert_eq!(charmander.name, "charmander");
        assert_eq!(charmander.height, 6);
        assert_eq!(charmander.weight, 85);
    }

    #[tokio::main]
    #[test]
    async fn responds_with_not_found_on_404() {
        // Given
        let mock_server = MockServer::start().await;
        let response = ResponseTemplate::new(404);
        Mock::given(method("GET"))
            .and(path("/charmander"))
            .respond_with(response)
            .mount(&mock_server)
            .await;

        let repository = PokemonRepository::new(mock_server.uri());

        // When
        let response = repository.get_pokemon("charmander").await;

        // Then
        assert!(matches!(response, Err(GetPokemonError::NotFound)));
    }
}

use crate::models::{Animu, AnimuTypes, AnimuQuote, Animal, AnimalEndpoints};
use reqwest::Client;
use serde::de::DeserializeOwned;

#[derive(Clone)]
pub struct SraClient {
    base_url: String,
    client: Client
}

impl SraClient {
    pub fn new() -> Self {
        SraClient { base_url: "https://api.some-random-api.com".to_string(), client: Client::new(), }
    }

    async fn fetch<T>(&self, path: &str) -> reqwest::Result<T>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, path);
        self.client
            .get(&url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
    }

    pub async fn animu_types(&self) -> reqwest::Result<AnimuTypes> {
        self.fetch("/animu").await
    }

    pub async fn animu(&self, kind: &str) -> reqwest::Result<Animu> {
        self.fetch(&format!("/animu/{}", kind)).await
    }

    pub async fn animu_quote(&self) -> reqwest::Result<AnimuQuote> {
        self.fetch(&format!("/animu/quote")).await
    }

    pub async fn animal_endpoints(&self) -> reqwest::Result<AnimalEndpoints> {
        self.fetch("/animal").await
    }

    pub async fn animal(&self, kind: &str) -> reqwest::Result<Animal> {
        self.fetch("/animal/endpoints").await
    }
}

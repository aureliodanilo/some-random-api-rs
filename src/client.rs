use crate::models::{ AnimuTypes };
use reqwest::Client;

#[derive(Clone)]
pub struct SraClient {
    base_url: String,
    client: Client
}

impl SraClient {
    pub fn new() -> Self {
        SraClient { base_url: "https://api.some-random-api.com".to_string(), client: Client::new(), }
    }

    pub async fn animu_types(&self) -> reqwest::Result<AnimuTypes> {
        let url = format!("{}/animu", self.base_url);
        self.client
            .get(url)
            .send()
            .await?
            .json::<AnimuTypes>()
            .await
    }
}
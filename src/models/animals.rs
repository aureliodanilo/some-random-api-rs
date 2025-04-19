use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AnimalEndpoints {
    pub endpoints: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Animal {
    pub image: String,
    pub fact: String,
}

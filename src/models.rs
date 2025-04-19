use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AnimuTypes {
    pub types: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct AnimuMedia {
    pub link: String,
    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Debug, Deserialize)]
pub struct AnimuQuote {
    pub quote: String,
    pub anime: String,
    pub id: u32,
    pub name: String,
}

use some_random_api::client::SraClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client: SraClient = SraClient::new();

    let types = client.animu_types().await?;
    println!("Available Animu types: {:?}", types.types);

    let kiss = client.animu("kiss").await?;
    println!("Animated Kiss {:?}", kiss);

    let quote = client.animu_quote().await?;
    println!("Animated Quote {:?}", quote);

    Ok(())
}

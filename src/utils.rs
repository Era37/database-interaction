use mongodb::{error::Error, options::ClientOptions, Client};

pub async fn mongo_connect() -> Result<Client, Error> {
    let mongo_options = ClientOptions::parse("mongodb://0.0.0.0:27017").await?;
    Client::with_options(mongo_options)
}

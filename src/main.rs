use actix_web::{self, error::HttpError};
use async_std;
use mongodb::{error::Error, options::ClientOptions, Client};
use redis;

struct DbConn {}

async fn db_connect() -> Result<(), Error> {
    let mongo_options = ClientOptions::parse("mongodb://0.0.0.0:27017").await?;
    let client = Client::with_options(mongo_options)?;
    Ok(())
}
fn main() {}

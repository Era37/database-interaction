use async_std::task;
use mongodb::{error::Error, options::ClientOptions, Client};

pub async fn mongo_connect() -> Result<Client, Error> {
    let mongo_options = ClientOptions::parse("mongodb://0.0.0.0:27017").await?;
    Client::with_options(mongo_options)
}

pub struct DbConn {
    pub mongo_conn: Client,
}

impl DbConn {
    pub fn new() -> Result<Self, Error> {
        let client = task::block_on(mongo_connect())?;
        Ok(Self { mongo_conn: client })
    }
}

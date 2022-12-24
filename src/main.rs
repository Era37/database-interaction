use async_std::task;
use mongodb::Client;
use tokio;
use utils::mongo_connect;
mod utils;

struct DbConn {
    mongo_conn: Client,
}

impl DbConn {
    fn new() -> Result<DbConn, Box<dyn std::error::Error>> {
        let client = task::block_on(mongo_connect())?;
        Ok(DbConn { mongo_conn: client })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn: DbConn = DbConn::new()?;
    println!("{:?}", conn.mongo_conn);
    Ok(())
}

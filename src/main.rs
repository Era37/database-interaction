use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use std::io::{Error, ErrorKind::Other, Result};
use utils::DbConn;
mod utils;

#[get("/")]
async fn my_test() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> Result<()> {
    DbConn::new().map_err(|e| Error::new(Other, e))?;
    HttpServer::new(|| App::new().service(my_test))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

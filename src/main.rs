use actix_cors::Cors;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use dotenvy;
use mongodb::{Client, Database};
use std::io::Result;
use utils::ChatBotLog;
mod utils;

#[post("/add")]
async fn database_append(db: web::Data<Database>, data: web::Json<ChatBotLog>) -> impl Responder {
    let collection = db.collection::<ChatBotLog>("messages");
    let result = collection.insert_one(data.into_inner(), None).await;
    match result {
        Ok(_) => HttpResponse::Ok().body("data added"),
        Err(_) => HttpResponse::Ok().body("failed to add data"),
    }
}

#[actix_web::main]
async fn main() -> Result<()> {
    let uri = dotenvy::var("MONGO").unwrap();
    let client = Client::with_uri_str(uri).await.expect("failed to connect");
    let db = client.database("ChatLogs");
    println!("Server Online");
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin();
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(database_append)
            .wrap(cors)
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}

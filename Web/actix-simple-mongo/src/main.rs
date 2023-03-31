use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use mongodb::{
    bson::{doc, Document},
    options::ClientOptions,
    Client,
};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    first_name: String,
    last_name: String,
    username: String,
    email: String,
}

async fn add_user(user: web::Json<User>) -> impl Responder {
    let uri = env::var("MONGO_URI").expect("MONGO_URI is not set.");
    let client_options = ClientOptions::parse(&uri).await.unwrap();
    let client = Client::with_options(client_options).unwrap();

    let db = client.database("mydb");
    let coll = db.collection("users");

    let document = doc! {
        "first_name": &user.first_name,
        "last_name": &user.last_name,
        "username": &user.username,
        "email": &user.email,
    };

    coll.insert_one(document, None).await.unwrap();

    HttpResponse::Ok().body("User added to database")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| App::new().service(web::resource("/users").route(web::post().to(add_user))))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

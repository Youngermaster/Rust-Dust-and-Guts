use axum::{handler::get, Router};
use dotenv::dotenv;
use std::env;
use tokio::sync::Mutex;

mod model;
mod routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok();

    let mongo_uri = env::var("MONGO_URI")?;
    let db_name = env::var("MONGO_DB_NAME")?;

    let client = mongodb::Client::with_uri_str(&mongo_uri).await?;
    let db = client.database(&db_name);

    let users = db.collection("users");
    let users = web::Data::new(users);

    let app = Router::new()
        .route("/", get(|| async { "Hello, Axum!" }))
        .route("/users", get(routes::get_users))
        .route("/users/:id", get(routes::get_user))
        .route("/users", post(routes::create_user))
        .route("/users/:id", put(routes::update_user))
        .route("/users/:id", delete(routes::delete_user))
        .layer(AddExtensionLayer::new(users))
        .layer(AddExtensionLayer::new(Mutex::new(client)));

    let addr = "0.0.0.0:3000";
    println!("Listening on {}", addr);

    Server::bind(&addr.parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

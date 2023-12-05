use actix_web::{App, HttpServer};
use sea_orm::{Database, DatabaseConnection};

mod handlers;
mod models;
mod service;
mod tls;

use handlers::*;
use tls::load_rustls_config;

#[derive(Debug, Clone)]
struct AppState {
    conn: DatabaseConnection,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // load environment variables
    dotenvy::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("'DATABASE_URL' should be set in .env file'");

    // set up database connection
    let conn = Database::connect(&db_url).await.unwrap();

    // create app state
    let state = AppState { conn };

    // set up TLS
    let rustls_config = load_rustls_config();

    // serve backend
    HttpServer::new(move || App::new().service(health_check))
        .bind_rustls_021(("127.0.0.1", 8000), rustls_config)?
        .run()
        .await
}

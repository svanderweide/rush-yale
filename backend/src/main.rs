use actix_identity::IdentityMiddleware;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, web, App, HttpServer};
use sea_orm::{Database, DatabaseConnection};

mod controllers;
mod models;
mod routes;
mod tls;

use routes::*;
use tls::load_rustls_config;

#[derive(Debug, Clone)]
struct AppState {
    conn: DatabaseConnection,
    base_url: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // load environment variables
    dotenvy::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("'DATABASE_URL' should be set in .env file'");
    let host = std::env::var("HOST").expect("'HOST' should be set in .env file");
    let port = std::env::var("PORT").expect("'PORT' should be set in .env file");
    let base_url = format!("{}:{}", host, port);

    // set up database connection
    let conn = Database::connect(&db_url)
        .await
        .expect("unable to connect to database");

    // create app state
    let state = AppState { conn, base_url };

    // set up TLS
    let rustls_config = load_rustls_config();

    // serve backend
    HttpServer::new(move || {
        App::new()
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                Key::generate(),
            ))
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/auth")
                            .app_data(web::Data::new(state.clone()))
                            .configure(auth_config),
                    )
                    .service(
                        web::scope("/events")
                            .app_data(web::Data::new(state.clone()))
                            .configure(event_config),
                    )
                    .service(
                        web::scope("/organizations")
                            .app_data(web::Data::new(state.clone()))
                            .configure(organization_config),
                    )
                    .service(
                        web::scope("/threads")
                            .app_data(web::Data::new(state.clone()))
                            .configure(thread_config),
                    )
                    .service(
                        web::scope("/users")
                            .app_data(web::Data::new(state.clone()))
                            .configure(user_config),
                    ),
            )
    })
    .bind_rustls_021(("127.0.0.1", 8000), rustls_config)?
    .run()
    .await
}

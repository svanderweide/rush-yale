use actix_identity::IdentityMiddleware;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, web, App, HttpServer};
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
    let conn = Database::connect(&db_url).await.unwrap();

    // create app state
    let state = AppState { conn, base_url };

    // set up TLS
    let rustls_config = load_rustls_config();

    // serve backend
    HttpServer::new(move || {
        App::new().service(health_check).service(
            web::scope("/api")
                .app_data(web::Data::new(state.clone()))
                .wrap(IdentityMiddleware::default())
                .wrap(SessionMiddleware::new(
                    CookieSessionStore::default(),
                    Key::generate(),
                ))
                .service(
                    web::scope("/auth")
                        .app_data(web::Data::new(state.clone()))
                        .service(login)
                        .service(logout),
                )
                .service(event_get_all_ids)
                .service(event_create)
                .service(event_get)
                .service(event_update)
                .service(organization_get_all_ids)
                .service(organization_create)
                .service(organization_get)
                .service(organization_update)
                .service(organization_get_all_users)
                .service(organization_get_user_status)
                .service(organization_create_user_status)
                .service(organization_update_user_status)
                .service(thread_get_all_ids)
                .service(thread_create)
                .service(thread_get)
                .service(thread_create_message)
                .service(user_get_all_ids)
                .service(user_create)
                .service(user_get)
                .service(user_update)
                .service(user_get_events)
                .service(user_get_statuses)
                .service(user_get_thread_ids),
        )
    })
    .bind_rustls_021(("127.0.0.1", 8000), rustls_config)?
    .run()
    .await
}

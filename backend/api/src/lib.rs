use actix_web::{App, HttpServer};
use rush_yale_service::sea_orm::{Database, DatabaseConnection};

mod handlers;
use handlers::health_check;

mod tls;
use tls::load_rustls_config;

#[derive(Debug, Clone)]
struct AppState {
    _conn: DatabaseConnection,
}

#[actix_web::main]
async fn serve() -> std::io::Result<()> {
    // load environment variables
    dotenvy::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("'DATABASE_URL' should be set in .env file'");

    // set up database connection
    let _conn = Database::connect(&db_url).await.unwrap();

    // create app state
    let state = AppState { _conn };

    // set up TLS
    let rustls_config = load_rustls_config();

    // serve backend
    HttpServer::new(move || App::new().app_data(state.clone()).service(health_check))
        .bind_rustls_021(("127.0.0.1", 8000), rustls_config)?
        .run()
        .await
}

pub fn main() {
    match serve() {
        Ok(_) => (),
        Err(_) => eprintln!("An error occurred."),
    }
}

use actix_web::{App, HttpServer};

mod handlers;
use handlers::health_check;

mod tls;
use tls::load_rustls_config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // set up TLS
    let rustls_config = load_rustls_config();

    HttpServer::new(|| App::new().service(health_check))
        .bind_rustls_021(("127.0.0.1", 8000), rustls_config)?
        .run()
        .await
}

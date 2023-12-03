use actix_web::{App, HttpServer};

mod handlers;
use handlers::health_check;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(health_check))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}

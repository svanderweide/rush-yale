use actix_web::{get, App, HttpResponse, HttpServer};

#[get("/health-check")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(health_check))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}

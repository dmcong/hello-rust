use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Params {
    num1: u32,
    num2: u32,
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[get("/sum/{num1}/{num2}")]
async fn sum(params: web::Path<Params>) -> impl Responder {
    HttpResponse::Ok().body(format!(
        "{} + {} = {}",
        params.num1,
        params.num2,
        (params.num1 + params.num2)
    ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(health).service(sum))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

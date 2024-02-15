use actix_web::{web, App, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct VeriIstegi {
    deger: String,
}

async fn index() -> impl Responder {
    "Welcome to blockchain app"
}

async fn veri(data: web::Json<VeriIstegi>) -> impl Responder {
    format!("data: {}", data.deger)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/veri", web::post().to(veri))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
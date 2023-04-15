use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io;

// Configure route
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("health", web::get().to(health_check_handler));
}

// Configure handler
pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().json("Actix Web Service is running!")
}

// Impl HTTP server and run
#[actix_rt::main]
async fn main() -> io::Result<()> {
    // generate app, configure route
    let app = move || App::new().configure(general_routes);

    // run HTTP server
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}

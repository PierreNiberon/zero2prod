use actix_web::dev::Server;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[get("/echo/{req}")]
async fn echo(req: web::Path<String>) -> impl Responder {
    format!("{}", req)
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .service(greet)
            .service(echo)
    })
    .listen(listener)?
    .run();

    Ok(server)
}

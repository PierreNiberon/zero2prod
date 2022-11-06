use actix_web::dev::Server;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

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

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(health_check))
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .service(greet)
            .service(echo)
    })
    .bind(("127.0.0.1", 8080))?
    .run();

    Ok(server)
}

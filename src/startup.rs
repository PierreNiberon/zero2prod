use crate::routes::{health_check::health_check, subscriptions::subscribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

// the run function builds the webserver and routes the requests
pub fn run(listener: TcpListener, connection: PgPool) -> Result<Server, std::io::Error> {
    // web::data is the bridge between actix and postgres
    let connection = web::Data::new(connection);
    // first we build the HttpServer that will contain an App inside a closure which handle the routing
    // then the HttpServer connects to the TCP socket
    // the run method makes the workers pop up
    // the app takes .route() with an http path to select where to send the request
    // then we choose the http verb with web::get, web::post, etc...
    // then we chose a handler to process the request
    // the app_data is for persistence in db
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();
    // if the server builds, then we send Ok with the server to main
    Ok(server)
}

use env_logger::Env;
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // `init` does call `set_logger`, so this is all we need to do.
    // We are falling back to printing all logs at info-level or above
    // if the RUST_LOG environment variable has not been set.
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    // Get configuration from the configuration module
    // the get_configuration method is built from the config crate
    // it reads the configuration.yaml file that contains info about our db and app
    let configuration = get_configuration().expect("Failed to read configuration.");
    // because our webserver may have concurential connection
    // we use PgPool instead of PgConnect
    // this way we can have a pool of connections
    // sqlx would not allow possible async concurential approach at compile time with PgConnect since you need the connection
    // to be &mut
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failure to connect to DB");
    // here we build the address that our app will serve
    // We have removed the hard-coded `8000` - it's now coming from our settings!
    let address = format!("127.0.0.1:{}", configuration.application_port);
    // we connect a socket and listen to the port
    let listener = TcpListener::bind(address)?;
    //we run the app from the run function comming from the startup module
    // the REST routing is going on inside the run function
    // it takes a TcpListener to connect to the port and a db connection pool to persist data
    // it is awaited because of th IO which makes main async and which requires tokio or actix runtime
    run(listener, connection_pool)?.await
}

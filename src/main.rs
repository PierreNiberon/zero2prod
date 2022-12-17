use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // init a subscriber for tracing purpose
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Get configuration from the configuration module
    // the get_configuration method is built from the config crate
    // it reads the configuration.yaml file that contains info about our db and app
    let configuration = get_configuration().expect("Failed to read configuration.");
    // because our webserver may have concurential connection
    // we use PgPool instead of PgConnect
    // this way we can have a pool of connections
    // sqlx would not allow possible async concurential approach at compile time with PgConnect since you need the connection
    // to be &mut
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy(&configuration.database.connection_string().expose_secret())
        .expect("Failed to create Postgres connection pool.");

    // here we build the address that our app will serve
    // We have removed the hard-coded `8000` - it's now coming from our settings!
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    // we connect a socket and listen to the port
    let listener = TcpListener::bind(address)?;
    //we run the app from the run function comming from the startup module
    // the REST routing is going on inside the run function
    // it takes a TcpListener to connect to the port and a db connection pool to persist data
    // it is awaited because of th IO which makes main async and which requires tokio or actix runtime
    run(listener, connection_pool)?.await
}

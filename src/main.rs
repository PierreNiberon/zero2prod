use sqlx::PgPool;
use std::net::TcpListener;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Redirect all `log`'s events to our subscriber
    LogTracer::init().expect("Failed to set logger");

    // We removed the `env_logger` line we had before!
    // We are falling back to printing all spans at info-level or above
    // if the RUST_LOG environment variable has not been set.
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new(
        "zero2prod".into(),
        // Output the formatted spans to stdout.
        std::io::stdout,
    );
    // The `with` method is provided by `SubscriberExt`, an extension
    // trait for `Subscriber` exposed by `tracing_subscriber`
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    // `set_global_default` can be used by applications to specify
    // what subscriber should be used to process spans.
    set_global_default(subscriber).expect("Failed to set subscriber");

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

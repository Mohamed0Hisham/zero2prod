use sqlx::PgPool;
use std::net::TcpListener;
use tracing::dispatcher::{self, Dispatch};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{EnvFilter, Registry, layer::SubscriberExt};
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_log::LogTracer::init().expect("Failed to set logger");
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new("zero2prod".into(), std::io::stdout);
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    dispatcher::set_global_default(Dispatch::new(subscriber))
        .expect("Failed to set tracing subscriber");

    let configuration = get_configuration().expect("Failed to read configuration file");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind random port");
    let db_connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to the Database");

    run(listener, db_connection)?.await
}

use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();
    let configuration = get_configuration().expect("Failed to read configuration file");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind random port");

    let configuration = get_configuration().expect("Failed to read configuration file");
    let db_connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to the Database");

    run(listener, db_connection)?.await
}

use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

fn create_listener(host: String, port: u16) -> TcpListener {
    let address = format!("{}:{}", host, port);
    TcpListener::bind(&address).expect("Failed to bind to address")
}

fn create_db_pool(connection_string: &str) -> PgPool {
    PgPool::connect_lazy(connection_string).expect("Failed to connect to the database")
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration file");
    let listener = create_listener(
        configuration.application.host,
        configuration.application.port,
    );
    let db_pool = create_db_pool(&configuration.database.connection_string().expose_secret());

    run(listener, db_pool)?.await
}

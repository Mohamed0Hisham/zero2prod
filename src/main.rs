use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

fn create_listener(port: &u16) -> TcpListener {
    let address = format!("127.0.0.1:{}", port);
    TcpListener::bind(&address).expect("Failed to bind to address")
}

async fn create_db_pool(connection_string: &str) -> PgPool {
    PgPool::connect(connection_string)
        .await
        .expect("Failed to connect to the database")
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration file");
    let listener = create_listener(&configuration.application_port);
    let db_pool = create_db_pool(&configuration.database.connection_string()).await;

    run(listener, db_pool)?.await
}

use env_logger::Env;
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Configure logging
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Read app configuration file
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");
    let address_str = format!("127.0.0.1:{}", configuration.application_port);
    let address = TcpListener::bind(address_str)?;
    run(address, connection_pool)?.await
}

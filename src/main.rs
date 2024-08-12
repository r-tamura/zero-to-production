use std::net::TcpListener;

use env_logger::Env;
use sqlx::PgPool;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let configuration = get_configuration().expect("Faile to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgress");
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind to localhost");
    run(listener, connection_pool)?.await
}

use std::net::TcpListener;

use secrecy::ExposeSecret;
use sqlx::PgPool;
use zero2prod::configuration::{get_configuration, get_subscriber, init_subscriber};
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    let configuration = get_configuration().expect("Faile to read configuration.");
    let connection_pool =
        PgPool::connect_lazy(&configuration.database.connection_string().expose_secret())
            .expect("Failed to connect to Postgress");
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address).expect("Failed to bind to localhost");
    run(listener, connection_pool)?.await
}

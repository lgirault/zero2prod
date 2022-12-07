use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::email_client::EmailClient;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

//cargo +nightly expand
#[tokio::main]
async fn main() -> Result<(), AppError> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");

    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );

    let sender_email = configuration
        .email_client
        .sender()
        .expect("Invalid sender email address.");
    let timeout = configuration.email_client.timeout();
    let email_client = EmailClient::new(
        configuration.email_client.base_url,
        sender_email,
        configuration.email_client.authorization_token,
        timeout,
    )?;
    let listener = TcpListener::bind(address)?;
    let _ = run(listener, connection_pool, email_client)?.await;
    Ok(())
}

#[derive(Debug)]
pub enum AppError{
    EmailClientError(zero2prod::email_client::EmailClientError),
    IOError(std::io::Error)
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::IOError(e)
    }
}

impl From<zero2prod::email_client::EmailClientError> for AppError {
    fn from(e: zero2prod::email_client::EmailClientError) -> Self {
        AppError::EmailClientError(e)
    }
}

use std::net::TcpListener;
use zero2prod::run;

//cargo +nightly expand
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000")?;
    run(listener)?.await
}

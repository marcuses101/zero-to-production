use std::net::TcpListener;
use zero_to_production::startup::run;

const PORT: &str = "8080";

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener =
        TcpListener::bind(format!("127.0.0.1:{}", PORT)).expect("Failed to bind to random port");
    run(listener)?.await
}

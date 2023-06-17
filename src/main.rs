use std::net::TcpListener;
use zero_to_production::configuration::get_configuration;
use zero_to_production::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let listener =
        TcpListener::bind(format!("127.0.0.1:{}", configuration.application_port)).expect(
            &format!("Failed to bind to port: {}", configuration.application_port),
        );
    run(listener)?.await
}

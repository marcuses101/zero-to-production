#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    zero_to_production::run()?.await
}

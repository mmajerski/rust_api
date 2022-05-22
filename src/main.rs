use rust_api::configuration::get_configuration;
use rust_api::startup::Application;
use rust_api::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("rust_api".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");

    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;

    Ok(())
}

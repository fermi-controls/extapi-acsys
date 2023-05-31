use tracing::Level;

mod g_rpc;
mod graphql;

#[tokio::main]
async fn main() {
    // Set up logging.

    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_target(false)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Unable to set global default subscriber");

    // Start the web server.

    graphql::start_service().await;
}

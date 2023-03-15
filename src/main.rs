mod graphql;

#[tokio::main]
async fn main() {
    graphql::start_service().await;
}

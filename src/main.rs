mod graphql;

#[tokio::main]
async fn main() {
    warp::serve(graphql::dpm::filter())
        .run(([127, 0, 0, 1], 8000))
        .await;
}
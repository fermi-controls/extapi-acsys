use warp::Filter;

pub mod dpm;

// Starts the web server that receives GraphQL queries. The configuration of the server is
// pulled together by obtaining configuration information from the submodules.

pub async fn start_service() {
    let filter = dpm::filter("dpm").with(
        warp::cors()
            .allow_any_origin()
            .allow_headers(vec!["content-type"])
            .allow_methods(vec!["OPTIONS", "GET", "POST"])
            .max_age(tokio::time::Duration::from_secs(3_600)),
    );

    warp::serve(filter).run(([0, 0, 0, 0], 8000)).await;
}

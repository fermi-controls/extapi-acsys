use warp::Filter;

pub mod dpm;

pub async fn start_service() {
    let filter = dpm::filter("dpm").with(
        warp::cors()
            .allow_any_origin()
            .allow_headers(vec!["content-type"])
            .allow_methods(vec!["OPTIONS", "GET", "POST"])
            .max_age(tokio::time::Duration::from_secs(3_600)),
    );

    warp::serve(filter).run(([127, 0, 0, 1], 8000)).await;
}

pub mod dpm;

// Starts the web server that receives GraphQL queries. The
// configuration of the server is pulled together by obtaining
// configuration information from the submodules.

pub async fn start_service() {
    let filter = dpm::filter("dpm");
    //.with(
    //    warp::cors()
    //        .allow_any_origin()
    //        .allow_headers(vec!["content-type", "Access-Control-Allow-Origin"])
    //        .allow_methods(vec!["OPTIONS", "GET", "POST"])
    //        .max_age(tokio::time::Duration::from_secs(3_600)),
    //);

    warp::serve(filter).run(([127, 0, 0, 1], 8000)).await;
}

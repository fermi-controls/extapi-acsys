use warp::Filter;

pub mod acsys;

// Starts the web server that receives GraphQL queries. The
// configuration of the server is pulled together by obtaining
// configuration information from the submodules.

pub async fn start_service() {
    let filter = acsys::filter("acsys").with(
        warp::cors()
            .allow_any_origin()
            .allow_headers(vec!["content-type", "Access-Control-Allow-Origin"])
            .allow_methods(vec!["OPTIONS", "GET", "POST"]),
    );

    warp::serve(filter).run(([127, 0, 0, 1], 8000)).await;
}

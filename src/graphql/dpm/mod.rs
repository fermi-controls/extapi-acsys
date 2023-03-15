use async_graphql::http::GraphiQLSource;
use async_graphql::*;
use async_graphql_warp::graphql_subscription;
use std::convert::Infallible;
use warp::{http::Response as HttpResponse, Filter, Rejection};

mod handlers;

type MySchema = Schema<handlers::QueryRoot, EmptyMutation, handlers::SubscriptionRoot>;

// Returns a Warp Filter that organizes the DPM protion of the web site.

pub fn filter(
    path: &str,
) -> impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone + '_ {
    // Create the schema object which is used to reply to GraphQL queries and subscriptions.

    let schema =
        Schema::build(handlers::QueryRoot, EmptyMutation, handlers::SubscriptionRoot).finish();

    // Build the query portion. The last path segment must be "q" and only POST methods
    // before handing the request to the schema.

    let graphql_query = warp::path("q")
        .and(warp::path::end())
        .and(warp::post())
        .and(async_graphql_warp::graphql(schema.clone()).and_then(
            |(schema, request): (MySchema, async_graphql::Request)| async move {
                let resp = schema.execute(request).await;

                Ok::<_, Infallible>(async_graphql_warp::GraphQLResponse::from(
                    resp,
                ))
            },
        ));

    // Build the subscription portion. The last path segment must be "s".

    let graphql_sub = warp::path("s")
        .and(warp::path::end())
        .and(graphql_subscription(schema));

    // Add diagnostic editor portion. No further path should be found and only GET
    // methods are allowed.

    let graphiql = warp::path::end().and(warp::get()).map(move || {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(
                GraphiQLSource::build()
                    .endpoint(format!("/{}/q", path).as_str())
                    .subscription_endpoint(format!("/{}/s", path).as_str())
                    .finish(),
            )
    });

    // Build the sub-site. Look, first, for the leading path and then look for any of the
    // above services.

    warp::path(path).and(graphiql.or(graphql_query).or(graphql_sub))
}

use async_graphql::http::GraphiQLSource;
use async_graphql::*;
use async_graphql_warp::graphql_subscription;
use futures_util::{stream, Stream};
use std::convert::Infallible;
use warp::{http::Response as HttpResponse, Filter, Rejection};

/// The control system supports several types and this entity can repesent any of them.
#[derive(SimpleObject)]
struct DataType {
    value: f64,
}

/// Holds information for a data return.
#[derive(SimpleObject)]
struct DataInfo {
    /// Timestamp representing when the data was sampled.
    timestamp: u64,

    /// The value of the device when sampled.
    result: DataType,

    /// The device's index (in the device database.)
    di: i32,

    /// The name of the device.
    name: String,

    /// A description of the device.
    description: String,

    /// The engineering units of the device's scaled value.
    units: Option<String>,
}

#[derive(SimpleObject)]
struct DataReply {
    ref_id: i32,
    cycle: u64,
    data: DataInfo,
}

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn accelerator_data(&self, drfs: Vec<String>) -> Vec<DataReply> {
        vec![]
    }
}

struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn accelerator_data(
        &self, drfs: Vec<String>,
    ) -> impl Stream<Item = DataReply> {
        use rand::{rngs::SmallRng, Rng, SeedableRng};

        stream::unfold(
            (SmallRng::from_entropy(), 70.0),
            |(mut rng, value)| async move {
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

                let noise = rng.gen_range(0.0..0.3) - 0.15;

                Some((
                    DataReply {
                        ref_id: 0,
                        cycle: 1,
                        data: DataInfo {
                            timestamp: 100,
                            result: DataType { value },
                            di: 1000,
                            name: "M:OUTTMP".to_string(),
                            description: "Outdoor temperature".to_string(),
                            units: Some("TEMP".to_string()),
                        },
                    },
                    (rng, value + noise),
                ))
            },
        )
    }
}

type MySchema = Schema<QueryRoot, EmptyMutation, SubscriptionRoot>;

pub fn filter(
) -> impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone {
    let schema =
        Schema::build(QueryRoot, EmptyMutation, SubscriptionRoot).finish();

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

    let graphql_sub = warp::path("s")
        .and(warp::path::end())
        .and(graphql_subscription(schema));

    let graphiql = warp::path::end().and(warp::get()).map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(
                GraphiQLSource::build()
                    .endpoint("/dpm/q")
                    .subscription_endpoint("/dpm/s")
                    .finish(),
            )
    });

    warp::path("dpm").and(graphiql.or(graphql_query).or(graphql_sub))
}

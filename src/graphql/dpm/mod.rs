use async_graphql::http::GraphiQLSource;
use async_graphql::*;
use async_graphql_warp::graphql_subscription;
use chrono::*;
use futures_util::{stream, Stream};
use std::convert::Infallible;
use warp::{http::Response as HttpResponse, Filter, Rejection};

#[derive(SimpleObject)]
struct StatusReply {
    status: i16,
}

#[derive(SimpleObject)]
struct Scalar {
    value: f64,
}

#[derive(SimpleObject)]
struct ScalarArray {
    values: Vec<f64>,
}

#[derive(SimpleObject)]
struct Raw {
    value: Vec<u8>,
}

#[derive(SimpleObject)]
struct Text {
    value: String,
}

#[derive(SimpleObject)]
struct TextArray {
    values: Vec<String>,
}

#[derive(SimpleObject)]
struct StructData {
    key: String,
    value: Box<DataType>,
}

/// The control system supports several types and this entity can repesent any of them.
#[derive(Union)]
enum DataType {
    StatusReply(StatusReply),
    Scalar(Scalar),
    ScalarArray(ScalarArray),
    Raw(Raw),
    Text(Text),
    TextArray(TextArray),
    StructData(StructData),
}

/// This structure holds information associated with a device's reading, A "reading" is the latest value of any of a device's properties.
#[derive(SimpleObject)]
struct DataInfo {
    /// Timestamp representing when the data was sampled. This value is provided as milliseconds since 1970, UTC.
    timestamp: DateTime<Utc>,

    /// The value of the device when sampled.
    result: DataType,

    /// The device's index (in the device database.)
    di: i32,

    /// The name of the device.
    name: String,

    /// A short description of the device.
    description: String,

    /// The engineering units of the device's scaled value. Some data types won't have units (asking for raw data, for instance.)
    units: Option<String>,
}

/// This structure wraps a device reading with some routing information: a `refId` to correlate which device, in the array of devices passed, this reply is for. It also has a `cycle` field so that reading from different devices can correlate which cycle they correspond.
#[derive(SimpleObject)]
struct DataReply {
    /// This is an index to indicate which entry, in the passed array of DRF strings, this reply corresponds.
    ref_id: i32,

    /// The cycle number in which the device was sampled. This can be used to correlate readings from several devices.
    cycle: u64,

    /// The returned data.
    data: DataInfo,
}

struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Retrieve the latest data from a set of devices. The returned vector will contain the readings of the devices in the same order as they were specified in the argument list.
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
                            timestamp: std::time::SystemTime::now().into(),
                            result: DataType::Scalar(Scalar { value }),
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

    warp::path("dpm")
        .and(graphiql.or(graphql_query).or(graphql_sub))
        .with(
            warp::cors()
                .allow_any_origin()
                .allow_headers(vec!["content-type"])
                .allow_methods(vec!["OPTIONS", "GET", "POST"])
                .max_age(tokio::time::Duration::from_secs(3_600)),
        )
}

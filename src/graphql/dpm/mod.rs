use std::convert::Infallible;
use async_graphql::http::GraphiQLSource;
use warp::{ Filter, Reply, reject, http::Response as HttpResponse };
use futures_util::{ Stream, stream };
use async_graphql::*;

#[derive(SimpleObject)]
struct DataType {
    value: f64,
}

#[derive(SimpleObject)]
struct DataInfo {
    timestamp: u64,
    result: DataType,
    di: i32,
    name: String,
    description: String,
    units: String,
}

#[derive(SimpleObject)]
struct DataReply {
    refId: i32,
    cycle: u64,
    data: DataInfo,
}

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn acceleratorData(&self, drfs: Vec<String>) -> Vec<DataReply> {
        vec![]
    }
}

struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn acceleratorData(&self, drfs: Vec<String>) -> impl Stream<Item = DataReply> {
        stream::empty()
    }
}

type MySchema = Schema<QueryRoot, EmptyMutation, SubscriptionRoot>;

pub async fn filter() {
    let schema = Schema::build(QueryRoot, EmptyMutation, SubscriptionRoot)
        .finish();
    let filter = async_graphql_warp::graphql(schema)
        .and_then(|(schema, request): (MySchema, async_graphql::Request)| async move {
            let resp = schema.execute(request).await;

            Ok::<_, Infallible>(async_graphql_warp::GraphQLResponse::from(resp))
        }
    );
    let graphiql = warp::path::end().and(warp::get()).map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(GraphiQLSource::build().endpoint("/").finish())
    });
    
    warp::serve(graphiql.or(filter)).run(([127, 0, 0, 1], 8000)).await
}
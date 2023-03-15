use async_graphql::*;
use futures_util::{stream, Stream};
use tokio::time;

mod types;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Retrieve the latest data from a set of devices. The returned vector will contain the readings of the devices in the same order as they were specified in the argument list.
    async fn accelerator_data(&self, drfs: Vec<String>) -> Vec<types::DataReply> {
        vec![]
    }
}

pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn accelerator_data(
        &self, drfs: Vec<String>,
    ) -> impl Stream<Item = types::DataReply> {
        use rand::{rngs::SmallRng, Rng, SeedableRng};

        stream::unfold(
            (SmallRng::from_entropy(), 70.0),
            |(mut rng, value)| async move {
                time::sleep(time::Duration::from_secs(1)).await;

                let noise = rng.gen_range(0.0..0.3) - 0.15;

                Some((
                    types::DataReply {
                        ref_id: 0,
                        cycle: 1,
                        data: types::DataInfo {
                            timestamp: std::time::SystemTime::now().into(),
                            result: types::DataType::Scalar(types::Scalar { value }),
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

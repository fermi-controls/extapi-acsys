use async_graphql::*;
use futures_util::{stream, Stream};
use tokio::time;

mod types;

mod fake {
    use rand::{rngs::SmallRng, Rng, SeedableRng};

    const VERBS: &[&str] = &[
        "Monitors",
        "Rotates",
        "Stabilizes",
        "Reads",
        "Aligns",
        "Engages",
        "Controls",
        "Moves",
        "Shakes",
    ];

    const SPECIFIER: &[&str] = &[
        "Johnson",
        "McCulkan",
        "quantum",
        "spanning",
        "lifting",
        "heating",
        "power",
        "primary",
        "secondary",
    ];

    const NOUN: &[&str] = &[
        "manifold",
        "vacuum valve",
        "horn",
        "scraper",
        "target",
        "wire scanners",
        "cryo-cavity",
        "lcw valve",
        "source",
    ];

    const UNITS: &[Option<&str>] = &[
        Some("mm"),
        Some("V"),
        Some("mA"),
        Some("ns"),
        Some("mL"),
        Some("MW"),
        Some("A"),
        Some("in/gal"),
        None,
        None,
    ];

    fn generate_descr() -> String {
        let mut rng = SmallRng::from_entropy();

        format!(
            "{} the {} {}",
            VERBS[rng.gen_range(0..VERBS.len())],
            SPECIFIER[rng.gen_range(0..SPECIFIER.len())],
            NOUN[rng.gen_range(0..NOUN.len())]
        )
    }

    pub struct DevInfo {
        pub reading: f64,
        pub di: i32,
        pub name: String,
        pub descr: String,
        pub units: Option<String>,
    }

    impl DevInfo {
        pub fn new(name: &str) -> Self {
            let mut rng = SmallRng::from_entropy();

            DevInfo {
                reading: rng.gen_range(0.0..100.0),
                di: rng.gen_range(100_000..300_000),
                name: name.into(),
                descr: generate_descr(),
                units: UNITS[rng.gen_range(0..UNITS.len())].map(String::from),
            }
        }

        pub fn update(&mut self) {
            let mut rng = SmallRng::from_entropy();

            self.reading += rng.gen_range(0.0..0.3) - 0.15;
        }
    }
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Retrieve the latest data from a set of devices. The returned vector will contain the readings of the devices in the same order as they were specified in the argument list.
    async fn accelerator_data(
        &self, drfs: Vec<String>,
    ) -> Vec<types::DataReply> {
        drfs.iter().fold(vec![], |mut acc, e| {
            let tmp = fake::DevInfo::new(&e);

            acc.push(types::DataReply {
                ref_id: acc.len() as i32,
                cycle: acc.len() as u64 + 1476,
                data: types::DataInfo {
                    timestamp: std::time::SystemTime::now().into(),
                    result: types::DataType::Scalar(types::Scalar {
                        scalar_value: tmp.reading,
                    }),
                    di: tmp.di,
                    name: e.clone(),
                    description: tmp.descr.clone(),
                    units: tmp.units.clone(),
                },
            });
            acc
        })
    }
}

pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn accelerator_data(
        &self, drfs: Vec<String>,
    ) -> impl Stream<Item = types::DataReply> {
        let state: Vec<fake::DevInfo> = drfs.iter().map(|e| fake::DevInfo::new(e.as_str())).collect();

        stream::unfold(
            (state, 0),
            |(mut state, count)| async move {
                if count == 0 {
                    time::sleep(time::Duration::from_secs(1)).await;
                }

                let next_count = (count + 1) % state.len();
                let item = &mut state[count];

                item.update();


                Some((
                    types::DataReply {
                        ref_id: count as i32,
                        cycle: 1,
                        data: types::DataInfo {
                            timestamp: std::time::SystemTime::now().into(),
                            result: types::DataType::Scalar(types::Scalar {
                                scalar_value: item.reading
                            }),
                            di: item.di,
                            name: item.name.clone(),
                            description: item.descr.clone(),
                            units: item.units.clone(),
                        },
                    },
                    (state, next_count),
                ))
            },
        )
    }
}

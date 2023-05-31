use crate::g_rpc::dpm;
use crate::g_rpc::devdb;
use async_graphql::*;
use futures_util::{Stream, StreamExt};
use tonic::Status;
use tracing::{error, warn};

// This module contains the GraphQL types that we'll use for the API.

mod types;

fn to_info_result(item: &devdb::proto::InfoEntry) -> types::DeviceInfoResult {
    match &item.result {
        Some(devdb::proto::info_entry::Result::Device(di)) => {
            types::DeviceInfoResult::DeviceInfo(types::DeviceInfo {
                description: di.description.clone(),
                reading: di.reading.as_ref().map(|p| {
                    types::DeviceProperty {
                        primary_units: p.primary_units.clone(),
                        common_units: p.common_units.clone(),
                    }
                }),
                setting: di.setting.as_ref().map(|p| {
                    types::DeviceProperty {
                        primary_units: p.primary_units.clone(),
                        common_units: p.common_units.clone(),
                    }
                }),
            })
        }
        Some(devdb::proto::info_entry::Result::ErrMsg(msg)) => {
            types::DeviceInfoResult::ErrorReply(types::ErrorReply {
                message: format!("{}", &msg),
            })
        }
        None => types::DeviceInfoResult::ErrorReply(types::ErrorReply {
            message: "empty response".into(),
                }),
            }
}

// Create a zero-sized struct to attach the GraphQL handlers.

pub struct QueryRoot;

// Define the schema's query entry points. Any methods defined in this
// section will appear in the schema.

#[Object]
impl QueryRoot {

    /// Retrieve the next data point for the specified devices. Depending upon the event in the DRF string, the data may come back immediately or after a delay.
    async fn accelerator_data(
        &self, _drfs: Vec<String>,
    ) -> Vec<types::DataReply> {
        vec![]
    }

    /// Retrieves device information. The parameter specifies the device. The reply will contain the device's information or an error status indicating why the query failed.
    async fn device_info(&self, devices: Vec<String>) -> types::DeviceInfoReply {
        let result = match devdb::get_device_info(devices).await {
            Ok(s) => {
		s.into_inner().set.iter().map(to_info_result).collect()
},
            Err(e) => {
                error!("gRPC error: {:?}", &e);
                todo!()
            }
        };

        types::DeviceInfoReply { result }
    }
}

fn xlat_type(t: &dpm::proto::Data) -> types::DataType {
    match t.value.as_ref() {
        Some(dpm::proto::data::Value::Scalar(v)) => {
            types::DataType::Scalar(types::Scalar { scalar_value: *v })
        }
        Some(dpm::proto::data::Value::ScalarArr(v)) => {
            types::DataType::ScalarArray(types::ScalarArray {
                scalar_array_value: v.value.clone(),
            })
        }
        Some(dpm::proto::data::Value::Status(v)) => {
            types::DataType::StatusReply(types::StatusReply {
                status: *v as i16,
            })
        }
        _ => todo!(),
    }
}

fn mk_xlater(
    names: Vec<String>,
) -> Box<
    dyn (FnMut(Result<dpm::proto::Reading, Status>) -> types::DataReply)
        + Send
        + Sync,
> {
    Box::new(move |e: Result<dpm::proto::Reading, Status>| {
        let e = e.unwrap();

        if let Some(ref data) = e.data {
            types::DataReply {
                ref_id: e.index as i32,
                cycle: 1,
                data: types::DataInfo {
                    timestamp: std::time::SystemTime::now().into(),
                    result: xlat_type(data),
                    di: 0,
                    name: names[e.index as usize].clone(),
                    description: String::from("n/a"),
                    units: None,
                },
            }
        } else {
            warn!("returned data: {:?}", &e.data);
            unreachable!()
        }
    })
}

pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn accelerator_data(
        &self, drfs: Vec<String>,
    ) -> impl Stream<Item = types::DataReply> {
        use crate::g_rpc::dpm;

        match dpm::acquire_devices(drfs.clone()).await {
            Ok(s) => s.into_inner().map(mk_xlater(drfs)),
            Err(e) => {
                error!("gRPC error: {:?}", &e);
                todo!()
            }
        }
    }
}

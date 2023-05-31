use proto::{dpm_client::DpmClient, AcquisitionList};

pub mod proto {
    tonic::include_proto!("dpm");
}

pub async fn acquire_devices(
    devices: Vec<String>,
) -> Result<tonic::Response<tonic::Streaming<proto::Reading>>, tonic::Status> {
    match DpmClient::connect("http://dce46.fnal.gov:50051/").await {
        Ok(mut client) => {
            let req = AcquisitionList {
                session_id: String::from(""),
                req: devices,
            };

            client.start_acquisition(req).await
        }
        Err(_) => Err(tonic::Status::unavailable("DPM service unavailable")),
    }
}

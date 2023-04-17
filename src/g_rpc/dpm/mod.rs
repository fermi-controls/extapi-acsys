use proto::{dpm_client::DpmClient, AcquisitionList};

pub mod proto {
    tonic::include_proto!("dpm");
}

pub async fn acquire_devices(
    devices: Vec<String>,
) -> Result<tonic::Response<tonic::Streaming<proto::Reading>>, tonic::Status> {
    let mut client = DpmClient::connect("http://dce46.fnal.gov:50051/")
        .await
        .unwrap();

    let req = AcquisitionList {
        session_id: String::from(""),
        req: devices,
    };

    client.start_acquisition(req).await
}

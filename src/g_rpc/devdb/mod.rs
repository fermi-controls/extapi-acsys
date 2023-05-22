use proto::dev_db_client::DevDbClient;

pub mod proto {
    tonic::include_proto!("devdb");
}

pub async fn get_device_info(
    device: String,
) -> Result<tonic::Response<proto::DeviceInfoReply>, tonic::Status> {
    let mut client = DevDbClient::connect("http://clx76.fnal.gov:6802/")
        .await
        .unwrap();

    let req = proto::DeviceList {
        device: vec![device],
    };

    client.get_device_info(req).await
}

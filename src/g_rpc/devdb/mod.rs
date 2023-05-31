use proto::dev_db_client::DevDbClient;

pub mod proto {
    tonic::include_proto!("devdb");
}

pub async fn get_device_info(
    device: &[String],
) -> Result<tonic::Response<proto::DeviceInfoReply>, tonic::Status> {
    match DevDbClient::connect("http://clx76.fnal.gov:6802/").await {
        Ok(mut client) => {
            let req = proto::DeviceList {
                device: device.to_vec(),
            };

            client.get_device_info(req).await
        }
        Err(_) => Err(tonic::Status::unavailable("DevDB service unavailable")),
    }
}

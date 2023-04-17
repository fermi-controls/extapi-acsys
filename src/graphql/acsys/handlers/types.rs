use async_graphql::*;
use chrono::*;

#[derive(SimpleObject)]
pub struct StatusReply {
    pub status: i16,
}

#[derive(SimpleObject)]
pub struct Scalar {
    pub scalar_value: f64,
}

#[derive(SimpleObject)]
pub struct ScalarArray {
    pub scalar_array_value: Vec<f64>,
}

#[derive(SimpleObject)]
pub struct Raw {
    pub raw_value: Vec<u8>,
}

#[derive(SimpleObject)]
pub struct Text {
    pub text_value: String,
}

#[derive(SimpleObject)]
pub struct TextArray {
    pub text_array_value: Vec<String>,
}

#[derive(SimpleObject)]
pub struct StructData {
    pub key: String,
    pub struct_value: Box<DataType>,
}

/// The control system supports several types and this entity can repesent any of them.
#[derive(Union)]
pub enum DataType {
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
pub struct DataInfo {
    /// Timestamp representing when the data was sampled. This value is provided as milliseconds since 1970, UTC.
    pub timestamp: DateTime<Utc>,

    /// The value of the device when sampled.
    pub result: DataType,

    /// The device's index (in the device database.)
    pub di: i32,

    /// The name of the device.
    pub name: String,

    /// A short description of the device.
    pub description: String,

    /// The engineering units of the device's scaled value. Some data types won't have units (asking for raw data, for instance.)
    pub units: Option<String>,
}

/// This structure wraps a device reading with some routing information: a `refId` to correlate which device, in the array of devices passed, this reply is for. It also has a `cycle` field so that reading from different devices can correlate which cycle they correspond.
#[derive(SimpleObject)]
pub struct DataReply {
    /// This is an index to indicate which entry, in the passed array of DRF strings, this reply corresponds.
    pub ref_id: i32,

    /// The cycle number in which the device was sampled. This can be used to correlate readings from several devices.
    pub cycle: u64,

    /// The returned data.
    pub data: DataInfo,
}

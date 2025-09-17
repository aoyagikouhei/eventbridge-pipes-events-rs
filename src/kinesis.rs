use std::ops::Deref;

pub use aws_lambda_events::encodings::{Base64Data, SecondTimestamp};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::error::Error;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KinesisEventRecord {
    /// nolint: stylecheck
    #[serde(default)]
    pub aws_region: Option<String>,
    #[serde(default)]
    #[serde(rename = "eventID")]
    pub event_id: Option<String>,
    #[serde(default)]
    pub event_name: Option<String>,
    #[serde(default)]
    pub event_source: Option<String>,
    /// nolint: stylecheck
    #[serde(default)]
    #[serde(rename = "eventSourceARN")]
    pub event_source_arn: Option<String>,
    #[serde(default)]
    pub event_version: Option<String>,
    /// nolint: stylecheck
    #[serde(default)]
    pub invoke_identity_arn: Option<String>,
    pub approximate_arrival_timestamp: SecondTimestamp,
    pub data: Base64Data,
    pub encryption_type: Option<String>,
    #[serde(default)]
    pub partition_key: Option<String>,
    #[serde(default)]
    pub sequence_number: Option<String>,
    #[serde(default)]
    pub kinesis_schema_version: Option<String>,
}

impl KinesisEventRecord {
    pub fn retrive_data<T>(&self) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let s = String::from_utf8(self.data.deref().to_vec())?.replace("\\u0000", "\\ufffd");
        serde_json::from_str::<T>(&s).map_err(Error::Json)
    }
}

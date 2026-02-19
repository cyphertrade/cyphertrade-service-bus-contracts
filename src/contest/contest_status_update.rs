use serde::{Deserialize, Serialize};
use yft_service_sdk::external::my_service_bus_sdk;
use yft_service_sdk::external::my_service_bus_sdk::macros::my_sb_entity_protobuf_model;

use crate::common::ContestStatus;

#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
#[my_sb_entity_protobuf_model(topic_id = "contest-status-update")]
pub struct ContestStatusUpdateSbEvent {
    #[prost(string, tag = "1")]
    pub contest_id: String,
    #[prost(enumeration = "ContestStatus", tag = "2")]
    pub status: i32,
}

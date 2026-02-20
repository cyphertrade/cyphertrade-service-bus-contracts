use serde::{Deserialize, Serialize};
use yft_service_sdk::external::my_service_bus_sdk;
use yft_service_sdk::external::my_service_bus_sdk::macros::my_sb_entity_protobuf_model;

use crate::common::RegistrationStatus;

#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
#[my_sb_entity_protobuf_model(topic_id = "contest-registration-update")]
pub struct ContestRegistrationUpdateSbEvent {
    #[prost(string, tag = "1")]
    pub user_id: String,
    #[prost(string, tag = "2")]
    pub contest_id: String,
    #[prost(enumeration = "RegistrationStatus", tag = "3")]
    pub status: i32,
    #[prost(string, tag = "4")]
    pub account_id: String,
}

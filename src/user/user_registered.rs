use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use yft_service_sdk::external::my_service_bus_sdk;
use yft_service_sdk::external::my_service_bus_sdk::macros::my_sb_entity_protobuf_model;

#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct UserSbModel {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(string, tag = "2")]
    pub email: String,
}

#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
#[my_sb_entity_protobuf_model(topic_id = "user-registered")]
pub struct UserRegisteredSbEvent {
    #[prost(message, tag = "1")]
    pub user: Option<UserSbModel>,
    #[prost(map = "string, string", tag = "2")]
    pub sources: HashMap<String, String>,
}

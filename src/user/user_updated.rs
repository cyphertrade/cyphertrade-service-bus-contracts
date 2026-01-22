use serde::{Deserialize, Serialize};
use yft_service_sdk::external::my_service_bus_sdk;
use yft_service_sdk::external::my_service_bus_sdk::macros::my_sb_entity_protobuf_model;

#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
#[my_sb_entity_protobuf_model(topic_id = "user-updated")]
pub struct UserUpdatedSbEvent {
    #[prost(string, tag = "1")]
    pub user_id: String,
    #[prost(string, tag = "2")]
    pub user_group: String,
    #[prost(uint64, tag = "3")]
    pub create_date: u64,
    #[prost(uint64, tag = "4")]
    pub last_update_date: u64,
}

use serde::{Deserialize, Serialize};
use yft_service_sdk::external::my_service_bus_sdk;
use yft_service_sdk::external::my_service_bus_sdk::macros::my_sb_entity_protobuf_model;

#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
#[my_sb_entity_protobuf_model(topic_id = "competition-positions-pl-update")]
pub struct CompetitionPositionsPlUpdateSbEvent {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(string, tag = "2")]
    pub contest_id: String,
    #[prost(string, tag = "3")]
    pub contest_account_id: String,
    #[prost(double, tag = "4")]
    pub pl: f64,
}
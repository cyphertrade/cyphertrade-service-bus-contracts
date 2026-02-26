use serde::{Deserialize, Serialize};
use yft_service_sdk::external::my_service_bus_sdk;
use yft_service_sdk::external::my_service_bus_sdk::macros::my_sb_entity_protobuf_model;

#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
#[my_sb_entity_protobuf_model(topic_id = "competition-positions-pl-update-batch")]
pub struct CompetitionPositionsPlUpdateBatchSbEvent {
    #[prost(string, tag = "1")]
    pub contest_id: String,
    #[prost(message, repeated, tag = "2")]
    pub account_updates: Vec<ContestAccountPositionPlUpdate>,
}

#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct ContestAccountPositionPlUpdate {
    #[prost(string, tag = "1")]
    pub contest_account_id: String,
    #[prost(message, repeated, tag = "2")]
    pub positions: Vec<PositionPlUpdate>,
}

#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct PositionPlUpdate {
    #[prost(string, tag = "1")]
    pub id: String,  // position_id
    #[prost(double, tag = "2")]
    pub pl: f64,
}
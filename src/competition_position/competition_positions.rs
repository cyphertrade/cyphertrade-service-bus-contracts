use serde::{Deserialize, Serialize};
use yft_service_sdk::external::my_service_bus_sdk;
use yft_service_sdk::external::my_service_bus_sdk::macros::my_sb_entity_protobuf_model;

#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
#[my_sb_entity_protobuf_model(topic_id = "competition-positions")]
pub struct CompetitionPositionsSbEvent {
    #[prost(oneof = "CompetitionPositionEventSbType", tags = "1, 2, 3")]
    pub event: Option<CompetitionPositionEventSbType>,
}

#[derive(Clone, PartialEq, ::prost::Oneof, Serialize, Deserialize)]
pub enum CompetitionPositionEventSbType {
    #[prost(message, tag = "1")]
    PositionUpdate(CompetitionPositionSbModel),
    #[prost(message, tag = "2")]
    PositionCreated(CompetitionPositionSbModel),
    #[prost(message, tag = "3")]
    PositionClosed(CompetitionPositionSbModel),
}

#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct CompetitionPositionSbModel {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(string, tag = "2")]
    pub contest_id: String,
    #[prost(string, tag = "3")]
    pub contest_account_id: String,
    #[prost(string, tag = "4")]
    pub asset_pair: String,
    #[prost(bool, tag = "5")]
    pub is_buy: bool,
    #[prost(double, tag = "6")]
    pub total_pl: f64,
    #[prost(double, tag = "7")]
    pub open_price: f64,
    #[prost(double, optional, tag = "8")]
    pub close_price: Option<f64>,
    #[prost(uint64, tag = "9")]
    pub create_date: u64,
    #[prost(uint64, optional, tag = "10")]
    pub close_date: Option<u64>,
    #[prost(double, tag = "11")]
    pub lots_amount: f64,
}

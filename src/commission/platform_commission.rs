use serde::{Deserialize, Serialize};
use yft_service_sdk::external::my_service_bus_sdk;
use yft_service_sdk::external::my_service_bus_sdk::macros::my_sb_entity_protobuf_model;

#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
#[my_sb_entity_protobuf_model(topic_id = "platform-commission")]
pub struct PlatformCommissionSbModel {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(string, tag = "2")]
    pub user_id: String,
    #[prost(string, optional, tag = "3")]
    pub reference_operation_id: Option<String>,
    #[prost(oneof = "PlatformCommissionEventSbSource", tags = "4, 5, 6, 7")]
    pub source: Option<PlatformCommissionEventSbSource>,
}

#[derive(Clone, PartialEq, ::prost::Oneof, Serialize, Deserialize)]
pub enum PlatformCommissionEventSbSource {
    #[prost(uint64, tag = "4")]
    ContestBuyIn(u64),
    #[prost(uint64, tag = "5")]
    BattleBuyIn(u64),
    #[prost(message, tag = "6")]
    BrokerRevShare(BrokerRevShareSbModel),
    #[prost(message, tag = "7")]
    BrokerCpa(BrokerCpaSbModel),
}

#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct BrokerRevShareSbModel {
    #[prost(string, tag = "1")]
    pub broker_id: String,
    #[prost(uint64, tag = "2")]
    pub amount: u64,
}

#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct BrokerCpaSbModel {
    #[prost(string, tag = "1")]
    pub broker_id: String,
    #[prost(uint64, tag = "2")]
    pub amount: u64,
}

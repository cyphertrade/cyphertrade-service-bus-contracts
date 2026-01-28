use serde::{Deserialize, Serialize};
use yft_service_sdk::external::my_service_bus_sdk;
use yft_service_sdk::external::my_service_bus_sdk::macros::my_sb_entity_protobuf_model;

#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
#[my_sb_entity_protobuf_model(topic_id = "contest-accounts-updates")]
pub struct ContestAccountsUpdateSbEvent {
    #[prost(string, tag = "1")]
    pub contest_id: String,
    #[prost(message, repeated, tag = "2")]
    pub accounts_updates: Vec<ContestAccountUpdateSbModel>,
}

#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct ContestAccountUpdateSbModel {
    #[prost(string, tag = "1")]
    pub contest_account_id: String,
    #[prost(double, tag = "2")]
    pub equity: f64,
    #[prost(message, repeated, tag = "3")]
    pub active_positions: Vec<ContestAccountUpdatePositionSbModel>,
    #[prost(string, tag = "4")]
    pub user_id: String,
}

#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct ContestAccountUpdatePositionSbModel {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(bool, tag = "2")]
    pub is_buy: bool,
    #[prost(double, tag = "3")]
    pub float_pl: f64,
    #[prost(string, tag = "4")]
    pub asset: String,
}

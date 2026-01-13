use serde::{Deserialize, Serialize};
use yft_service_sdk::external::my_service_bus_sdk;
use yft_service_sdk::external::my_service_bus_sdk::macros::my_sb_entity_protobuf_model;

#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
#[my_sb_entity_protobuf_model(topic_id = "contest-accounts-updates")]
pub struct ContestAccountsUpdateSbEvent {
    #[prost(string, tag = "1")]
    pub contest_id: String,
    #[prost(message, repeated, tag = "2")]
    pub accounts_updates: Vec<ContestAccountUpdateGrpcModel>,
}

#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct ContestAccountUpdateGrpcModel {
    #[prost(string, tag = "1")]
    pub contest_account_id: String,
    #[prost(double, tag = "2")]
    pub balance: f64,
    #[prost(double, tag = "3")]
    pub equity: f64,
    #[prost(double, tag = "4")]
    pub margin: f64,
    #[prost(double, tag = "5")]
    pub free_margin: f64,
}

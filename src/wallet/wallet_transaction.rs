use serde::{Deserialize, Serialize};
use yft_service_sdk::external::my_service_bus_sdk;
use yft_service_sdk::external::my_service_bus_sdk::macros::my_sb_entity_protobuf_model;

use crate::common::WalletTransactionReason;

#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
#[my_sb_entity_protobuf_model(topic_id = "wallet-transaction")]
pub struct WalletTransactionSbEvent {
    #[prost(string, tag = "1")]
    pub transaction_id: String,
    #[prost(string, tag = "2")]
    pub wallet_id: String,
    #[prost(string, tag = "3")]
    pub user_id: String,
    #[prost(int64, tag = "4")]
    pub points_delta: i64,
    #[prost(uint64, tag = "5")]
    pub date: u64,
    #[prost(enumeration = "WalletTransactionReason", tag = "6")]
    pub reason: i32,
}

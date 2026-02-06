use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use yft_service_sdk::external::my_service_bus_sdk;
use yft_service_sdk::external::my_service_bus_sdk::macros::my_sb_entity_protobuf_model;

use super::BrokerAffiliateEventSbType;

#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
#[my_sb_entity_protobuf_model(topic_id = "broker-affiliate-events")]
pub struct BrokerAffiliateEventSbModel {
    #[prost(string, tag = "1")]
    pub affiliate_name: String,
    #[prost(string, tag = "2")]
    pub id: String,
    #[prost(string, optional, tag = "3")]
    pub broker_operation_id: Option<String>,
    #[prost(string, tag = "4")]
    pub broker_id: String,
    #[prost(string, tag = "5")]
    pub user_id: String,
    #[prost(uint64, tag = "6")]
    pub date: u64,
    #[prost(map = "string, string", tag = "7")]
    pub utm_params: HashMap<String, String>,
    #[prost(enumeration = "BrokerAffiliateEventSbType", tag = "8")]
    pub event_type: i32,
    #[prost(double, optional, tag = "9")]
    pub amount: Option<f64>,
}

use serde::{Deserialize, Serialize};
use yft_service_sdk::external::my_service_bus_sdk;
use yft_service_sdk::external::my_service_bus_sdk::macros::my_sb_entity_protobuf_model;

#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
#[my_sb_entity_protobuf_model(topic_id = "personal-data-updated")]
pub struct PersonalDataUpdatedSbEvent {
    #[prost(string, tag = "1")]
    pub user_id: String,
    #[prost(string, tag = "2")]
    pub first_name: String,
    #[prost(string, tag = "3")]
    pub last_name: String,
    #[prost(string, optional, tag = "4")]
    pub country_code: Option<String>,
    #[prost(string, optional, tag = "5")]
    pub nationality_code: Option<String>,
    #[prost(string, optional, tag = "6")]
    pub gender: Option<String>,
    #[prost(string, optional, tag = "7")]
    pub phone_code: Option<String>,
    #[prost(string, optional, tag = "8")]
    pub phone_number: Option<String>,
    #[prost(string, optional, tag = "9")]
    pub address_line1: Option<String>,
    #[prost(string, optional, tag = "10")]
    pub address_line2: Option<String>,
    #[prost(string, optional, tag = "11")]
    pub postal_code: Option<String>,
    #[prost(uint64, optional, tag = "12")]
    pub date_of_birth: Option<u64>,
}

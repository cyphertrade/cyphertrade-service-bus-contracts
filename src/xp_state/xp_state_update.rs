use serde::{Deserialize, Serialize};
use yft_service_sdk::external::my_service_bus_sdk;
use yft_service_sdk::external::my_service_bus_sdk::macros::my_sb_entity_protobuf_model;

/// XP type matches XpEngineXpGrpcType: Partner = 0, Trader = 1, Influencer = 2.
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
#[my_sb_entity_protobuf_model(topic_id = "xp-state-update")]
pub struct XpStateUpdateSbEvent {
    #[prost(string, tag = "1")]
    pub user_id: String,
    #[prost(int32, tag = "2")]
    pub xp_type: i32,
    #[prost(uint32, tag = "3")]
    pub current_level: u32,
    #[prost(uint64, tag = "4")]
    pub current_level_xp: u64,
    #[prost(uint64, tag = "5")]
    pub next_level_required_xp: u64,
}

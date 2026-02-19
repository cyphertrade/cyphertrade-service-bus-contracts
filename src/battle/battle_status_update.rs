use serde::{Deserialize, Serialize};
use yft_service_sdk::external::my_service_bus_sdk;
use yft_service_sdk::external::my_service_bus_sdk::macros::my_sb_entity_protobuf_model;

use crate::common::BattleStatus;

#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
#[my_sb_entity_protobuf_model(topic_id = "battle-status-update")]
pub struct BattleStatusUpdateSbEvent {
    #[prost(string, tag = "1")]
    pub battle_id: String,
    #[prost(enumeration = "BattleStatus", tag = "2")]
    pub status: i32,
}

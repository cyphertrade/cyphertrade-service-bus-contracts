use serde::{Deserialize, Serialize};
use yft_service_sdk::external::my_service_bus_sdk;
use yft_service_sdk::external::my_service_bus_sdk::macros::my_sb_entity_protobuf_model;

#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
#[my_sb_entity_protobuf_model(topic_id = "battle-participation-update")]
pub struct BattleParticipationUpdateSbEvent {
    #[prost(string, tag = "1")]
    pub user_id: String,
    #[prost(string, tag = "2")]
    pub battle_id: String,
    #[prost(string, tag = "3")]
    pub status: String,
    #[prost(bool, optional, tag = "4")]
    pub is_winner: Option<bool>,
    #[prost(double, optional, tag = "5")]
    pub pnl: Option<f64>,
}

use serde::{Deserialize, Serialize};
use yft_service_sdk::external::my_service_bus_sdk;
use yft_service_sdk::external::my_service_bus_sdk::macros::my_sb_entity_protobuf_model;

#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
#[my_sb_entity_protobuf_model(topic_id = "public-profile-update")]
pub struct PublicProfileUpdateSbEvent {
    #[prost(string, tag = "1")]
    pub user_id: String,
    #[prost(string, optional, tag = "2")]
    pub instagram_link: Option<String>,
    #[prost(string, optional, tag = "3")]
    pub discord_link: Option<String>,
    #[prost(string, optional, tag = "4")]
    pub profile_description: Option<String>,
    #[prost(string, optional, tag = "5")]
    pub image_id_from_cdn: Option<String>,
    #[prost(bool, tag = "6")]
    pub show_real_name: bool,
    #[prost(bool, tag = "7")]
    pub show_trading_activity: bool,
}

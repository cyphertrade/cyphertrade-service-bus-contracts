use serde::{Deserialize, Serialize};

/// XP type matches XpEngineXpGrpcType: Partner = 0, Trader = 1, Influencer = 2.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ::prost::Enumeration, Serialize, Deserialize)]
#[repr(i32)]
pub enum XpStateType {
    Trader = 1,
    Influencer = 2,
}

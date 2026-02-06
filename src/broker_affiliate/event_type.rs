use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, ::prost::Enumeration, Serialize, Deserialize)]
#[repr(i32)]
pub enum BrokerAffiliateEventSbType {
    Register = 0,
    Cpa = 1,
    RevShare = 2,
    Deposit = 3,
}

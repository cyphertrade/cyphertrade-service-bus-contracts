use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, ::prost::Enumeration, Serialize, Deserialize)]
#[repr(i32)]
pub enum WalletTransactionReason {
    Deposit = 0,
    Withdrawal = 1,
    Rebait = 2,
    Correction = 3,
    Contest = 4,
}

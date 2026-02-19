use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, ::prost::Enumeration, Serialize, Deserialize)]
#[repr(i32)]
pub enum RegistrationStatus {
    Registered = 0,
    Canceled = 1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ::prost::Enumeration, Serialize, Deserialize)]
#[repr(i32)]
pub enum ParticipationStatus {
    InProcess = 0,
    Finished = 1,
    Disqualified = 2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ::prost::Enumeration, Serialize, Deserialize)]
#[repr(i32)]
pub enum ContestStatus {
    Draft = 0,
    RegistrationOpen = 1,
    InProgress = 2,
    Finished = 3,
    Canceled = 4,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ::prost::Enumeration, Serialize, Deserialize)]
#[repr(i32)]
pub enum BattleStatus {
    Draft = 0,
    RegistrationOpen = 1,
    InProgress = 2,
    Finished = 3,
    Canceled = 4,
}

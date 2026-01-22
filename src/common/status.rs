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

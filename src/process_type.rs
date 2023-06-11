use crate::error::EnumDeserializationFromStrError;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

// TryFrom<String> is needed due to https://github.com/serde-rs/serde/issues/1183
// While still allowing the caller to use .with_process_type(ProcessType::Background)
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String"))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcessType {
    Background,
    Standard,
    Adaptive,
    Interactive,
}

impl TryFrom<String> for ProcessType {
    type Error = EnumDeserializationFromStrError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "Background" => Ok(ProcessType::Background),
            "Standard" => Ok(ProcessType::Standard),
            "Adaptive" => Ok(ProcessType::Adaptive),
            "Interactive" => Ok(ProcessType::Interactive),
            _ => Err(EnumDeserializationFromStrError),
        }
    }
}

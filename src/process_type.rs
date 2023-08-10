use crate::error::Error;

use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

/// Enumeration of different types of process behavior.
///
/// This enum represents different types of process behavior that can be used in configuration.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
// TryFrom<String> is needed due to https://github.com/serde-rs/serde/issues/1183
// While still allowing the caller to use .with_process_type(ProcessType::Background)
#[serde(try_from = "String")]
pub enum ProcessType {
    /// Indicates background process behavior.
    Background,
    /// Indicates standard process behavior.
    Standard,
    /// Indicates adaptive process behavior.
    Adaptive,
    /// Indicates interactive process behavior.
    Interactive,
}

impl TryFrom<String> for ProcessType {
    type Error = Error;

    /// Converts a string into a `ProcessType` instance.
    ///
    /// This conversion is used to parse a string into a corresponding `ProcessType` value.
    /// If the string does not match any of the known process types, an error is returned.
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "Background" => Ok(ProcessType::Background),
            "Standard" => Ok(ProcessType::Standard),
            "Adaptive" => Ok(ProcessType::Adaptive),
            "Interactive" => Ok(ProcessType::Interactive),
            value => Err(Error::EnumDeserialization(value.to_owned())),
        }
    }
}

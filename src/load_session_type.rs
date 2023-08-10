use serde::{Deserialize, Serialize};

/// A type representing session types for load limiting.
///
/// This enum defines various session types that can be used for load limiting purposes.
/// The variants can hold a single string or an array of strings representing session types.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum LoadSessionType {
    /// Represents a string value for `LoadSessionType` configuration.
    BareString(String),
    /// Represents an array of string values for `LoadSessionType` configuration.
    Array(Vec<String>),
}

impl From<String> for LoadSessionType {
    fn from(value: String) -> Self {
        LoadSessionType::BareString(value)
    }
}

impl From<&str> for LoadSessionType {
    fn from(value: &str) -> Self {
        LoadSessionType::BareString(value.to_owned())
    }
}

impl From<Vec<String>> for LoadSessionType {
    fn from(value: Vec<String>) -> Self {
        LoadSessionType::Array(value)
    }
}

impl From<Vec<&str>> for LoadSessionType {
    fn from(value: Vec<&str>) -> Self {
        LoadSessionType::Array(value.into_iter().map(|s| s.to_owned()).collect())
    }
}

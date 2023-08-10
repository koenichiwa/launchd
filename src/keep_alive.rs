use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Enumeration of different types of keep-alive configurations.
///
/// This enum represents the various ways in which a service can be configured for keep-alive behavior.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum KeepAliveType {
    /// Indicates that keep-alive is enabled with a boolean value.
    Enabled(bool),
    /// Represents keep-alive options with specific configurations.
    Options(KeepAliveOptions),
}

impl From<bool> for KeepAliveType {
    fn from(value: bool) -> Self {
        KeepAliveType::Enabled(value)
    }
}

impl From<KeepAliveOptions> for KeepAliveType {
    fn from(value: KeepAliveOptions) -> Self {
        KeepAliveType::Options(value)
    }
}

/// Configuration options for keep-alive behavior.
///
/// This struct holds various options that can be configured for keep-alive behavior of a service.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "PascalCase")]
pub struct KeepAliveOptions {
    successful_exit: Option<bool>,
    network_state: Option<bool>,
    path_state: Option<HashMap<String, bool>>,
    other_job_enabled: Option<HashMap<String, bool>>,
}

impl KeepAliveOptions {
    /// Creates a new `KeepAliveOptions` instance with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the option to specify whether keep-alive is triggered on successful exit.
    pub fn with_successful_exit(mut self, value: bool) -> Self {
        self.successful_exit = Some(value);
        self
    }

    /// Sets the option to specify whether keep-alive is triggered based on network state.
    pub fn with_network_state(mut self, value: bool) -> Self {
        self.network_state = Some(value);
        self
    }

    /// Sets the option to specify keep-alive based on specific path states.
    pub fn with_path_state(mut self, value: HashMap<String, bool>) -> Self {
        self.path_state = Some(value);
        self
    }

    /// Sets the option to specify keep-alive based on other job states.
    pub fn with_other_job_enabled(mut self, value: HashMap<String, bool>) -> Self {
        self.other_job_enabled = Some(value);
        self
    }
}
// See the KeepAlive section in https://www.manpagez.com/man/5/launchd.plist/
//
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[derive(Debug, Clone, PartialEq)]
pub enum KeepAliveType {
    Enabled(bool),
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

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "io", serde(rename_all = "PascalCase"))]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct KeepAliveOptions {
    successful_exit: Option<bool>,
    network_state: Option<bool>,
    path_state: Option<HashMap<String, bool>>,
    other_job_enabled: Option<HashMap<String, bool>>,
}

impl KeepAliveOptions {
    pub fn new() -> Self {
        Self {
            successful_exit: None,
            network_state: None,
            path_state: None,
            other_job_enabled: None,
        }
    }

    pub fn with_successful_exit(mut self, value: bool) -> Self {
        self.successful_exit = Some(value);
        self
    }

    pub fn with_network_state(mut self, value: bool) -> Self {
        self.network_state = Some(value);
        self
    }

    pub fn with_path_state(mut self, value: HashMap<String, bool>) -> Self {
        self.path_state = Some(value);
        self
    }

    pub fn with_other_job_enabled(mut self, value: HashMap<String, bool>) -> Self {
        self.other_job_enabled = Some(value);
        self
    }
}

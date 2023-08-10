// See the KeepAlive section in https://www.manpagez.com/man/5/launchd.plist/
//

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "PascalCase")]
pub struct KeepAliveOptions {
    successful_exit: Option<bool>,
    network_state: Option<bool>,
    path_state: Option<HashMap<String, bool>>,
    other_job_enabled: Option<HashMap<String, bool>>,
}

impl KeepAliveOptions {
    pub fn new() -> Self {
        Self::default()
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

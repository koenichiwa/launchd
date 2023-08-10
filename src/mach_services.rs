// See the MachServices section in https://www.manpagez.com/man/5/launchd.plist/
//

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum MachServiceEntry {
    Boolean(bool),
    Map(MachServiceOptions),
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
#[serde(rename_all = "PascalCase")]
pub struct MachServiceOptions {
    reset_at_close: Option<bool>,
    hide_until_check_in: Option<bool>,
}

impl MachServiceOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_reset_at_close(mut self, value: bool) -> Self {
        self.reset_at_close = Some(value);
        self
    }

    pub fn reset_at_close(self) -> Self {
        self.with_reset_at_close(true)
    }

    pub fn with_hide_until_check_in(mut self, value: bool) -> Self {
        self.hide_until_check_in = Some(value);
        self
    }

    pub fn hide_until_check_in(self) -> Self {
        self.with_hide_until_check_in(true)
    }
}

impl From<MachServiceOptions> for MachServiceEntry {
    fn from(options: MachServiceOptions) -> Self {
        MachServiceEntry::Map(options)
    }
}

impl From<bool> for MachServiceEntry {
    fn from(value: bool) -> Self {
        MachServiceEntry::Boolean(value)
    }
}

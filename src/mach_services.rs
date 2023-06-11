// See the MachServices section in https://www.manpagez.com/man/5/launchd.plist/
//
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MachServiceEntry {
    Boolean(bool),
    Map(MachServiceOptions),
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "io", serde(rename_all = "PascalCase"))]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MachServiceOptions {
    reset_at_close: Option<bool>,
    hide_until_check_in: Option<bool>,
}

impl MachServiceOptions {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
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

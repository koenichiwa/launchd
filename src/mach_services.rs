use serde::{Deserialize, Serialize};

/// Enumeration of different types of Mach service entries.
///
/// A `MachServiceEntry` represents the configuration for a Mach service within a `launchd.plist` file.
/// A Mach service is a type of inter-process communication (IPC) mechanism used in macOS and other Unix-like systems.
/// Mach services allow processes to communicate and share resources across different tasks in a secure and controlled manner.
///
/// In the context of `launchd.plist` files, a Mach service entry can be specified with different options, such as
/// enabling or disabling the service, setting up authorization rights, and more.
///
/// # Examples
///
/// ```rust
/// use std::collections::HashMap;
/// use launchd::{Launchd, MachServiceEntry, MachServiceOptions};
///
/// let mut mach_services = HashMap::<String, MachServiceEntry>::default();
/// mach_services.insert("com.example.my_service".to_string(), true.into());
/// mach_services.insert("com.example.my_other_service".to_string(), MachServiceOptions::new()
///              .reset_at_close()
///              .with_hide_until_check_in(false)
///              .into());
/// let launchd = Launchd::default().with_mach_services(mach_services);
/// ```
///
/// See the MachServices section in [launchd.plist(5)](https://www.manpagez.com/man/5/launchd.plist/) for more information.
#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum MachServiceEntry {
    /// Indicates a Mach service entry with a boolean value.
    Boolean(bool),
    /// Represents a Mach service entry with specific options.
    Map(MachServiceOptions),
}

/// Configuration options for a Mach service entry.
///
/// This struct holds various options that can be configured for a Mach service entry.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
#[serde(rename_all = "PascalCase")]
pub struct MachServiceOptions {
    reset_at_close: Option<bool>,
    hide_until_check_in: Option<bool>,
}

impl MachServiceOptions {
    /// Creates a new `MachServiceOptions` instance with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the option to reset the service at close.
    pub fn with_reset_at_close(mut self, value: bool) -> Self {
        self.reset_at_close = Some(value);
        self
    }

    /// Sets the option to reset the service at close, with a value of `true`.
    pub fn reset_at_close(self) -> Self {
        self.with_reset_at_close(true)
    }

    /// Sets the option to hide the service until check-in.
    pub fn with_hide_until_check_in(mut self, value: bool) -> Self {
        self.hide_until_check_in = Some(value);
        self
    }

    /// Sets the option to hide the service until check-in, with a value of `true`.
    pub fn hide_until_check_in(self) -> Self {
        self.with_hide_until_check_in(true)
    }
}

impl From<MachServiceOptions> for MachServiceEntry {
    /// Converts `MachServiceOptions` into a `MachServiceEntry::Map`.
    fn from(options: MachServiceOptions) -> Self {
        MachServiceEntry::Map(options)
    }
}

impl From<bool> for MachServiceEntry {
    /// Converts a boolean value into a `MachServiceEntry::Boolean`.
    fn from(value: bool) -> Self {
        MachServiceEntry::Boolean(value)
    }
}

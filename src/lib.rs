#![warn(missing_docs)]
//! A Rust library for creating and parsing Launchd files.
//!
//! It's still in early development and all help is welcome.
//!
//! ## Example
//!
//! ``` rust
//! use launchd::{CalendarInterval, Error, Launchd};
//!
//! fn main() -> Result<(), Error> {
//!     let ci = CalendarInterval::default()
//!         .with_hour(12)?
//!         .with_minute(10)?
//!         .with_weekday(7)?;
//!
//!     let launchd = Launchd::new("LABEL".to_string(), "./foo/bar.txt".into())
//!             .with_user_name("Henk".to_string())
//!             .with_program_arguments(vec!["Hello".to_string(), "World!".to_string()])
//!             .with_start_calendar_intervals(vec![ci])
//!             .disabled();
//!
//!     return launchd.to_writer_xml(std::io::stdout());
//! }
//! ```
//!
//! Results in:
//!
//! ``` xml
//! <?xml version="1.0" encoding="UTF-8"?>
//! <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
//! <plist version="1.0">
//! <dict>
//!         <key>Label</key>
//!         <string>LABEL</string>
//!         <key>Disabled</key>
//!         <true />
//!         <key>UserName</key>
//!         <string>Henk</string>
//!         <key>Program</key>
//!         <string>./foo/bar.txt</string>
//!         <key>ProgramArguments</key>
//!         <array>
//!                 <string>Hello</string>
//!                 <string>World!</string>
//!         </array>
//!         <key>StartCalendarIntervals</key>
//!         <array>
//!                 <dict>
//!                         <key>Minute</key>
//!                         <integer>10</integer>
//!                         <key>Hour</key>
//!                         <integer>12</integer>
//!                         <key>Weekday</key>
//!                         <integer>7</integer>
//!                 </dict>
//!         </array>
//! </dict>
//! </plist>
//! ```

mod calendar_interval;
mod error;
mod keep_alive;
mod load_session_type;
mod mach_services;
mod process_type;
mod resource_limits;
mod sockets;

// Re-exporting public items from module files for easy access from the main module.
pub use calendar_interval::CalendarInterval;
pub use error::Error;
pub use keep_alive::{KeepAliveOptions, KeepAliveType};
pub use load_session_type::LoadSessionType;
pub use mach_services::{MachServiceEntry, MachServiceOptions};
pub use process_type::ProcessType;
pub use resource_limits::ResourceLimits;
pub use sockets::{BonjourType, Socket, SocketOptions, Sockets};

use plist::Value;
use plist::{from_bytes, from_file, from_reader, from_reader_xml};
use plist::{to_file_binary, to_file_xml, to_writer_binary, to_writer_xml};
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::io::{Read, Seek, Write};
use std::path::PathBuf;

/// Representation of a launchd.plist file.
/// The definition of which can be found [here](https://www.manpagez.com/man/5/launchd.plist/).
///
/// Usage:
/// ```
/// use launchd::{Launchd, Error, CalendarInterval};
/// use std::path::Path;
///
/// fn example() -> Result<Launchd, Error> {
///     Ok(Launchd::new("LABEL".to_string(), "./foo/bar.txt".into())
///         .with_user_name("Henk".to_string())
///         .with_program_arguments(vec!["Hello".to_string(), "World!".to_string()])
///         .with_start_calendar_intervals(vec![CalendarInterval::default().with_hour(12)?])
///         .disabled()
///         // etc...
///     )
/// }
///
/// let launchd = example();
///
/// ```
/// This will create a launchd representation with the label "LABEL", running "./foo/bar.txt"
/// with the args "Hello" and "World!", for the user "Henk", each day at 12.
///
/// NB: The usage is still subject to change.
// TODO: Fill with all options in https://www.manpagez.com/man/5/launchd.plist/

// Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Display, Default
#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "PascalCase")]
pub struct Launchd {
    label: String,
    disabled: Option<bool>,
    user_name: Option<String>,
    group_name: Option<String>,
    #[serde(rename = "inetdCompatibility")]
    inetd_compatibility: Option<HashMap<InetdCompatibility, bool>>,
    limit_load_to_hosts: Option<Vec<String>>,
    limit_load_from_hosts: Option<Vec<String>>,
    limit_load_to_session_type: Option<LoadSessionType>,
    limit_load_to_hardware: Option<HashMap<String, Vec<String>>>,
    limit_load_from_hardware: Option<HashMap<String, Vec<String>>>,
    program: Option<PathBuf>, // TODO: Ensure this: "NOTE: The Program key must be an absolute path."
    bundle_program: Option<String>,
    program_arguments: Option<Vec<String>>,
    enable_globbing: Option<bool>,
    enable_transactions: Option<bool>,
    enable_pressured_exit: Option<bool>,
    on_demand: Option<bool>, // NB: deprecated (see KeepAlive), but still needed for reading old plists.
    #[serde(rename = "ServiceIPC")]
    service_ipc: Option<bool>, // NB: "Please remove this key from your launchd.plist."
    keep_alive: Option<KeepAliveType>,
    run_at_load: Option<bool>,
    root_directory: Option<PathBuf>,
    working_directory: Option<PathBuf>,
    environment_variables: Option<HashMap<String, String>>,
    umask: Option<u16>, // NB: This is a Unix permission mask. Defined as: typedef __uint16_t __darwin_mode_t;
    time_out: Option<u32>,
    exit_time_out: Option<u32>,
    throttle_interval: Option<u32>,
    init_groups: Option<bool>,
    watch_paths: Option<Vec<String>>,
    queue_directories: Option<Vec<String>>,
    start_on_mount: Option<bool>,
    start_interval: Option<u32>,
    start_calendar_intervals: Option<Vec<CalendarInterval>>,
    standard_in_path: Option<PathBuf>,
    standard_out_path: Option<PathBuf>,
    standard_error_path: Option<PathBuf>,
    debug: Option<bool>,
    wait_for_debugger: Option<bool>,
    soft_resource_limits: Option<ResourceLimits>,
    hard_resource_limits: Option<ResourceLimits>,
    nice: Option<i32>,
    process_type: Option<ProcessType>,
    abandon_process_group: Option<bool>,
    #[serde(rename = "LowPriorityIO")]
    low_priority_io: Option<bool>,
    #[serde(rename = "LowPriorityBackgroundIO")]
    low_priority_background_io: Option<bool>,
    materialize_dataless_files: Option<bool>,
    launch_only_once: Option<bool>,
    mach_services: Option<HashMap<String, MachServiceEntry>>,
    sockets: Option<Sockets>,
    launch_events: Option<LaunchEvents>,
    hopefully_exits_last: Option<bool>, // NB: Deprecated, keep for reading old plists.
    hopefully_exits_first: Option<bool>, // NB: Deprecated, keep for reading old plists.
    session_create: Option<bool>,
    legacy_timers: Option<bool>, // NB: Deprecated, keep for reading old plists.
                                 // associated_bundle_identifiers: Option<<string or array of strings>>
}

// Defined as a "<dictionary of dictionaries of dictionaries>" in launchd.plist(5)
// Use plist::Value as the value can be String, Integer, Boolean, etc.
// Doing this precludes the use of #[derive(Eq)] on the Launchd struct, but in practice "PartialEq" is fine.
type LaunchEvents = HashMap<String, HashMap<String, HashMap<String, Value>>>;

/// A type representing compatibility modes for inetd.
///
/// This enum defines compatibility modes for inetd-style services.
/// The `Wait` variant indicates that the service should wait for the completion of the launched
/// process before continuing.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum InetdCompatibility {
    /// Only valid variant that was found
    Wait, // Exclude a "NoWait" as that is not a valid key.
}

// TODO: This can be generated by a macro (maybe derive_builder?)
impl Launchd {
    // Builder --

    /// Creates a new `Launchd` instance with the specified label and program path.
    ///
    /// # Arguments
    ///
    /// * `label` - The label for the launchd instance.
    /// * `program` - The absolute path to the program associated with the launchd instance.
    pub fn new(label: String, program: PathBuf) -> Self {
        Launchd {
            label,
            program: Some(program),
            ..Default::default()
        }
    }

    /// Sets the `label` for the `Launchd` instance and returns the modified instance.
    pub fn with_label(mut self, label: String) -> Self {
        self.label = label;
        self
    }

    /// Sets the `disabled` variable for the `Launchd` instance and returns the modified instance.
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = Some(disabled);
        self
    }

    /// Sets the `disabled` variable for the `Launchd` instance to true and returns the modified instance.
    pub fn disabled(self) -> Self {
        self.with_disabled(true)
    }

    /// Sets the `user_name` variable for the `Launchd` instance and returns the modified instance.
    pub fn with_user_name(mut self, user_name: String) -> Self {
        self.user_name = Some(user_name);
        self
    }

    /// Sets the `group_name` variable for the `Launchd` instance and returns the modified instance.
    pub fn with_group_name(mut self, group_name: String) -> Self {
        self.group_name = Some(group_name);
        self
    }

    /// Sets the path to the `program` variable for the `Launchd` instance and returns the modified instance.
    pub fn with_program(mut self, path: PathBuf) -> Self {
        self.program = Some(path);
        self
    }

    /// Sets the `bundle_program` variable for the `Launchd` instance and returns the modified instance.
    pub fn with_bundle_program(mut self, bundle: String) -> Self {
        self.bundle_program = Some(bundle);
        self
    }

    /// Sets the `program_arguments` variable for the `Launchd` instance and returns the modified instance.
    pub fn with_program_arguments(mut self, program_arguments: Vec<String>) -> Self {
        self.program_arguments = Some(program_arguments);
        self
    }

    /// Sets the `run_at_load` variable for the `Launchd` instance and returns the modified instance.
    pub fn with_run_at_load(mut self, run_at_load: bool) -> Self {
        self.run_at_load = Some(run_at_load);
        self
    }

    /// Sets the `run_at_load` variable for the `Launchd` instance to true and returns the modified instance.
    pub fn run_at_load(self) -> Self {
        self.with_run_at_load(true)
    }

    /// Sets the `queue_directories` variable for the `Launchd` instance and returns the modified instance.
    pub fn with_queue_directories(mut self, queue_directories: Vec<String>) -> Self {
        self.queue_directories = Some(queue_directories);
        self
    }

    /// Sets the `watch_paths` variable for the `Launchd` instance and returns the modified instance.
    pub fn with_watch_paths(mut self, watch_paths: Vec<String>) -> Self {
        self.watch_paths = Some(watch_paths);
        self
    }

    /// Sets the `start_on_mount` variable for the `Launchd` instance and returns the modified instance.
    pub fn with_start_on_mount(mut self, start_on_mount: bool) -> Self {
        self.start_on_mount = Some(start_on_mount);
        self
    }

    /// Sets the `start_on_mount` variable for the `Launchd` instance to true and returns the modified instance.
    pub fn start_on_mount(self) -> Self {
        self.with_start_on_mount(true)
    }

    /// Sets the `start_interval` variable for the `Launchd` instance and returns the modified instance.
    pub fn with_start_interval(mut self, start_interval: u32) -> Self {
        self.start_interval = Some(start_interval);
        self
    }

    /// Sets the `start_calendar_intervals` variable for the `Launchd` instance and returns the modified instance.
    pub fn with_start_calendar_intervals(
        mut self,
        start_calendar_intervals: Vec<CalendarInterval>,
    ) -> Self {
        self.start_calendar_intervals = Some(start_calendar_intervals);
        self
    }

    /// Sets the `abandon_process_group` variable for the `Launchd` instance and returns the modified instance.
    pub fn with_abandon_process_group(mut self, value: bool) -> Self {
        self.abandon_process_group = Some(value);
        self
    }

    /// Sets the `abandon_process_group` variable for the `Launchd` instance to true and returns the modified instance.
    pub fn abandon_process_group(self) -> Self {
        self.with_abandon_process_group(true)
    }

    /// Sets the debug variable for the `Launchd` instance and returns the modified instance.
    pub fn with_debug(mut self, value: bool) -> Self {
        self.debug = Some(value);
        self
    }

    /// Sets the debug variable for the `Launchd` instance to true and returns the modified instance.
    pub fn debug(self) -> Self {
        self.with_debug(true)
    }

    /// Sets the `enable_globbing` variable for the `Launchd` instance and returns the modified instance.
    pub fn with_enable_globbing(mut self, value: bool) -> Self {
        self.enable_globbing = Some(value);
        self
    }

    /// Sets the `enable_globbing` variable for the `Launchd` instance to true and returns the modified instance.
    pub fn enable_globbing(self) -> Self {
        self.with_enable_globbing(true)
    }

    /// Sets the `enable_transactions` variable for the `Launchd` instance and returns the modified instance.
    pub fn with_enable_transactions(mut self, value: bool) -> Self {
        self.enable_transactions = Some(value);
        self
    }

    /// Sets the `enable_transactions` variable to true for the `Launchd` instance and returns the modified instance.
    pub fn enable_transactions(self) -> Self {
        self.with_enable_transactions(true)
    }

    /// Sets the `enable_pressured_exit` variable for the `Launchd` instance and returns the modified instance.
    pub fn with_enable_pressured_exit(mut self, value: bool) -> Self {
        self.enable_pressured_exit = Some(value);
        self
    }

    /// Sets the `enable_pressured_exit` variable to true for the `Launchd` instance and returns the modified instance.
    pub fn enable_pressured_exit(self) -> Self {
        self.with_enable_pressured_exit(true)
    }

    /// Sets the `environment_variables` variable for the `Launchd` instance and returns the modified instance.
    pub fn with_environment_variables(mut self, env: HashMap<String, String>) -> Self {
        self.environment_variables = Some(env);
        self
    }

    /// Sets the `exit_timeout` variable for the `Launchd` instance and returns the modified instance.
    pub fn with_exit_timeout(mut self, timeout: u32) -> Self {
        self.exit_time_out = Some(timeout);
        self
    }

    /// Sets the `init_groups` variable for the `Launchd` instance and returns the modified instance.
    pub fn with_init_groups(mut self, value: bool) -> Self {
        self.init_groups = Some(value);
        self
    }

    /// Sets the `init_groups` variable for the `Launchd` instance to true and returns the modified instance.
    pub fn init_groups(self) -> Self {
        self.with_init_groups(true)
    }

    /// Sets the `launch_only_once` variable for the `Launchd` instance and returns the modified instance.
    pub fn with_launch_only_once(mut self, value: bool) -> Self {
        self.launch_only_once = Some(value);
        self
    }

    /// Sets the `launch_only_once` variable for the `Launchd` instance to true and returns the modified instance.
    pub fn launch_only_once(self) -> Self {
        self.with_launch_only_once(true)
    }

    /// Sets the `limit_load_from_hosts` variable for the `Launchd` instance and returns the modified instance.
    pub fn with_limit_load_from_hosts(mut self, value: Vec<String>) -> Self {
        self.limit_load_from_hosts = Some(value);
        self
    }

    /// Sets the `limit_to_from_hosts` variable for the `Launchd` instance and returns the modified instance.
    pub fn with_limit_to_from_hosts(mut self, value: Vec<String>) -> Self {
        self.limit_load_to_hosts = Some(value);
        self
    }

    /// Sets the `limit_load_to_session_type` variable for the `Launchd` instance and returns the modified instance.
    pub fn with_limit_load_to_session_type(mut self, value: LoadSessionType) -> Self {
        self.limit_load_to_session_type = Some(value);
        self
    }

    /// Sets the `limit_load_to_hardware` variable for the `Launchd` instance and returns the modified instance.
    pub fn with_limit_load_to_hardware(mut self, value: HashMap<String, Vec<String>>) -> Self {
        self.limit_load_to_hardware = Some(value);
        self
    }

    /// Sets the `limit_load_from_hardware` variable for the `Launchd` instance and returns the modified instance.
    pub fn with_limit_load_from_hardware(mut self, value: HashMap<String, Vec<String>>) -> Self {
        self.limit_load_from_hardware = Some(value);
        self
    }

    /// Sets the `low_priority_io` variable for the `Launchd` instance and returns the modified instance.
    pub fn with_low_priority_io(mut self, value: bool) -> Self {
        self.low_priority_io = Some(value);
        self
    }

    /// Sets the `low_priority_io` variable for the `Launchd` instance to true and returns the modified instance.
    pub fn low_priority_io(self) -> Self {
        self.with_low_priority_io(true)
    }

    /// Sets the `low_priority_background_io` variable for the `Launchd` instance and returns the modified instance.
    pub fn with_low_priority_background_io(mut self, value: bool) -> Self {
        self.low_priority_background_io = Some(value);
        self
    }

    /// Sets the `low_priority_background_io` variable for the `Launchd` instance to true and returns the modified instance.
    pub fn low_priority_background_io(self) -> Self {
        self.with_low_priority_background_io(true)
    }

    /// Sets the `mach_services` variable for the `Launchd` instance and returns the modified instance.
    pub fn with_mach_services(mut self, services: HashMap<String, MachServiceEntry>) -> Self {
        self.mach_services = Some(services);
        self
    }

    /// Sets the nice variable for the `Launchd` instance and returns the modified instance.
    pub fn with_nice(mut self, nice: i32) -> Self {
        self.nice = Some(nice);
        self
    }

    /// Sets the `root_directory` variable for the `Launchd` instance and returns the modified instance.
    pub fn with_root_directory(mut self, path: PathBuf) -> Self {
        self.root_directory = Some(path);
        self
    }

    /// Sets the `standard_error_path` variable for the `Launchd` instance and returns the modified instance.
    pub fn with_standard_error_path(mut self, path: PathBuf) -> Self {
        self.standard_error_path = Some(path);
        self
    }

    /// Sets the `standard_in_path` variable for the `Launchd` instance and returns the modified instance.
    pub fn with_standard_in_path(mut self, path: PathBuf) -> Self {
        self.standard_in_path = Some(path);
        self
    }

    /// Sets the `standard_out_path` variable for the `Launchd` instance and returns the modified instance.
    pub fn with_standard_out_path(mut self, path: PathBuf) -> Self {
        self.standard_out_path = Some(path);
        self
    }

    /// Sets the `throttle_interval` variable for the `Launchd` instance and returns the modified instance.
    pub fn with_throttle_interval(mut self, value: u32) -> Self {
        self.throttle_interval = Some(value);
        self
    }

    /// Sets the timeout variable for the `Launchd` instance and returns the modified instance.
    pub fn with_timeout(mut self, timeout: u32) -> Self {
        self.time_out = Some(timeout);
        self
    }

    /// Sets the umask variable for the `Launchd` instance and returns the modified instance.
    pub fn with_umask(mut self, umask: u16) -> Self {
        self.umask = Some(umask);
        self
    }

    /// Sets the `wait_for_debugger` variable for the `Launchd` instance and returns the modified instance.
    pub fn with_wait_for_debugger(mut self, value: bool) -> Self {
        self.wait_for_debugger = Some(value);
        self
    }

    /// Sets the `wait_for_debugger` variable for the `Launchd` instance to true and returns the modified instance.
    pub fn wait_for_debugger(self) -> Self {
        self.with_wait_for_debugger(true)
    }

    /// Sets the `materialize_dataless_files` variable for the `Launchd` instance and returns the modified instance.
    pub fn with_materialize_dataless_files(mut self, value: bool) -> Self {
        self.materialize_dataless_files = Some(value);
        self
    }

    /// Sets the `materialize_dataless_files` variable for the `Launchd` instance to true and returns the modified instance.
    pub fn materialize_dataless_files(self) -> Self {
        self.with_materialize_dataless_files(true)
    }

    /// Sets the `working_directory` variable for the `Launchd` instance and returns the modified instance.
    pub fn with_working_directory(mut self, path: PathBuf) -> Self {
        self.working_directory = Some(path);
        self
    }

    /// Sets the `inetd_compatibility` variable for the `Launchd` instance and returns the modified instance.
    pub fn with_inetd_compatibility(mut self, wait: bool) -> Self {
        self.inetd_compatibility = Some(HashMap::from([(InetdCompatibility::Wait, wait)]));
        self
    }

    /// Sets the `keep_alive` variable for the `Launchd` instance and returns the modified instance.
    pub fn with_keep_alive(mut self, keep_alive: KeepAliveType) -> Self {
        self.keep_alive = Some(keep_alive);
        self
    }

    /// Sets the `process_type` variable for the `Launchd` instance and returns the modified instance.
    pub fn with_process_type(mut self, process_type: ProcessType) -> Self {
        self.process_type = Some(process_type);
        self
    }

    /// Sets the `hard_resource_limits` variable for the `Launchd` instance and returns the modified instance.
    pub fn with_hard_resource_limits(mut self, limits: ResourceLimits) -> Self {
        self.hard_resource_limits = Some(limits);
        self
    }

    /// Sets the `soft_resource_limits` variable for the `Launchd` instance and returns the modified instance.
    pub fn with_soft_resource_limits(mut self, limits: ResourceLimits) -> Self {
        self.soft_resource_limits = Some(limits);
        self
    }

    /// Sets the socket variable for the `Launchd` instance and returns the modified instance.
    pub fn with_socket(mut self, socket: Sockets) -> Self {
        if let Some(sockets) = self.sockets.take() {
            match (sockets, socket) {
                (Sockets::Array(mut arr), Sockets::Array(mut new_arr)) => {
                    arr.append(&mut new_arr);
                    self.sockets = Some(Sockets::Array(arr));
                }
                (Sockets::Array(mut arr), Sockets::Dictionary(new_dict)) => {
                    arr.push(new_dict);
                    self.sockets = Some(Sockets::Array(arr));
                }
                (Sockets::Dictionary(dict), Sockets::Dictionary(new_dict)) => {
                    self.sockets = Some(Sockets::Array(vec![dict, new_dict]));
                }
                (Sockets::Dictionary(dict), Sockets::Array(mut new_arr)) => {
                    new_arr.insert(0, dict);
                    self.sockets = Some(Sockets::Array(new_arr));
                }
            }
        } else {
            self.sockets = Some(socket);
        }
        self
    }

    /// Sets the `launch_events` variable for the `Launchd` instance and returns the modified instance.
    pub fn with_launch_events(mut self, value: LaunchEvents) -> Self {
        self.launch_events = Some(value);
        self
    }

    /// Sets the `session_create` variable for the `Launchd` instance and returns the modified instance.
    pub fn with_session_create(mut self, value: bool) -> Self {
        self.session_create = Some(value);
        self
    }

    /// Sets the `session_create` variable for the `Launchd` instance to true and returns the modified instance.
    pub fn session_create(self) -> Self {
        self.with_session_create(true)
    }

    // Write --

    /// Writes the `Launchd` instance to an XML writer.
    ///
    /// # Arguments
    ///
    /// * `writer` - The XML writer to which the instance will be written.
    ///
    /// # Errors
    ///
    /// This function can return an error if writing the XML fails.
    pub fn to_writer_xml<W: Write>(&self, writer: W) -> Result<(), Error> {
        to_writer_xml(writer, self).map_err(Error::Write)
    }

    /// Writes the `Launchd` instance to an XML file.
    ///
    /// # Arguments
    ///
    /// * `file` - The path to the XML file to which the instance will be written.
    ///
    /// # Errors
    ///
    /// This function can return an error if writing the XML file fails.
    pub fn to_file_xml(&self, file: PathBuf) -> Result<(), Error> {
        to_file_xml(file, self).map_err(Error::Write)
    }

    /// Writes the `Launchd` instance to a binary writer.
    ///
    /// # Arguments
    ///
    /// * `writer` - The binary writer to which the instance will be written.
    ///
    /// # Errors
    ///
    /// This function can return an error if writing the binary format fails.
    pub fn to_writer_binary<W: Write>(&self, writer: W) -> Result<(), Error> {
        to_writer_binary(writer, self).map_err(Error::Write)
    }

    /// Writes the `Launchd` instance to a binary file.
    ///
    /// # Arguments
    ///
    /// * `file` - The path to the binary file to which the instance will be written.
    ///
    /// # Errors
    ///
    /// This function can return an error if writing the binary file fails.
    pub fn to_file_binary(&self, file: PathBuf) -> Result<(), Error> {
        to_file_binary(file, self).map_err(Error::Write)
    }

    // Read --

    /// Creates a `Launchd` instance from a byte slice containing binary data.
    ///
    /// # Arguments
    ///
    /// * `bytes` - The byte slice containing the binary data to be deserialized.
    ///
    /// # Errors
    ///
    /// This function can return an error if deserialization fails.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        from_bytes(bytes).map_err(Error::Read)
    }

    /// Creates a `Launchd` instance by reading from an XML file.
    ///
    /// # Arguments
    ///
    /// * `file` - The path to the XML file from which the instance will be read.
    ///
    /// # Errors
    ///
    /// This function can return an error if reading the XML file fails or if deserialization fails.
    pub fn from_file(file: PathBuf) -> Result<Self, Error> {
        from_file(file).map_err(Error::Read)
    }

    /// Creates a `Launchd` instance by reading from a reader of binary data.
    ///
    /// # Arguments
    ///
    /// * `reader` - The reader from which the binary data will be read and deserialized.
    ///
    /// # Errors
    ///
    /// This function can return an error if reading from the reader or deserialization fails.
    pub fn from_reader<R: Read + Seek>(reader: R) -> Result<Self, Error> {
        from_reader(reader).map_err(Error::Read)
    }

    /// Creates a `Launchd` instance by reading from an XML reader.
    ///
    /// # Arguments
    ///
    /// * `reader` - The XML reader from which the instance will be read and deserialized.
    ///
    /// # Errors
    ///
    /// This function can return an error if reading from the XML reader or deserialization fails.
    pub fn from_reader_xml<R: Read + Seek>(reader: R) -> Result<Self, Error> {
        from_reader_xml(reader).map_err(Error::Read)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_case {
        ($fname:expr) => {
            PathBuf::from(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/resources/",
                $fname
            ))
            // PathBuf::from_iter([env!("CARGO_MANIFEST_DIR"),"/tests/resources/"])
        };
    }

    #[test]
    fn create_valid_launchd() {
        let check = Launchd {
            abandon_process_group: None,
            debug: None,
            disabled: None,
            enable_globbing: None,
            enable_transactions: None,
            enable_pressured_exit: None,
            on_demand: None,
            service_ipc: None,
            environment_variables: None,
            exit_time_out: None,
            group_name: None,
            inetd_compatibility: None,
            init_groups: None,
            hard_resource_limits: None,
            keep_alive: None,
            label: "Label".to_string(),
            launch_only_once: None,
            launch_events: None,
            legacy_timers: None,
            limit_load_from_hosts: None,
            limit_load_to_hosts: None,
            limit_load_to_session_type: None,
            limit_load_to_hardware: None,
            limit_load_from_hardware: None,
            low_priority_io: None,
            low_priority_background_io: None,
            hopefully_exits_first: None,
            hopefully_exits_last: None,
            mach_services: None,
            materialize_dataless_files: None,
            session_create: None,
            nice: None,
            process_type: None,
            program_arguments: None,
            program: Some(PathBuf::from("./henk.sh")),
            bundle_program: None,
            queue_directories: None,
            root_directory: None,
            run_at_load: None,
            sockets: None,
            soft_resource_limits: None,
            standard_error_path: None,
            standard_in_path: None,
            standard_out_path: None,
            start_calendar_intervals: None,
            start_interval: None,
            start_on_mount: None,
            throttle_interval: None,
            time_out: None,
            umask: None,
            user_name: None,
            wait_for_debugger: None,
            watch_paths: None,
            working_directory: None,
        };
        let test = Launchd::new("Label".to_string(), "./henk.sh".into());
        assert_eq!(test, check);
    }

    #[test]
    fn load_complex_launch_events_1_plist() {
        eprintln!("{:?}", test_case!("launchevents-1.plist"));
        let test = Launchd::from_file(test_case!("launchevents-1.plist")).unwrap();

        match test.launch_events {
            Some(events) => assert!(events.contains_key("com.apple.distnoted.matching")),
            _ => panic!("No launch events found"),
        };
    }

    #[test]
    fn load_complex_launch_events_2_plist() {
        let check: LaunchEvents = vec![(
            "com.apple.iokit.matching".to_string(),
            vec![(
                "com.apple.device-attach".to_string(),
                vec![
                    ("IOMatchLaunchStream".to_string(), Value::from(true)),
                    ("idProduct".to_string(), Value::from("*")),
                    ("idVendor".to_string(), Value::from(4176)),
                    ("IOProviderClass".to_string(), Value::from("IOUSBDevice")),
                ]
                .into_iter()
                .collect(),
            )]
            .into_iter()
            .collect(),
        )]
        .into_iter()
        .collect();

        let test = Launchd::from_file(test_case!("launchevents-2.plist")).unwrap();

        match test.launch_events {
            Some(events) => assert_eq!(events, check),
            _ => panic!("No launch events found"),
        };
    }

    #[test]
    fn load_complex_machservices_1_plist() {
        let check = vec![
            (
                "com.apple.private.alloy.accessibility.switchcontrol-idswake".to_string(),
                MachServiceEntry::from(true),
            ),
            (
                "com.apple.AssistiveControl.startup".to_string(),
                MachServiceEntry::from(MachServiceOptions::new().reset_at_close()),
            ),
            (
                "com.apple.AssistiveControl.running".to_string(),
                MachServiceEntry::from(
                    MachServiceOptions::new()
                        .hide_until_check_in()
                        .reset_at_close(),
                ),
            ),
        ]
        .into_iter()
        .collect();

        let test = Launchd::from_file(test_case!("machservices-1.plist")).unwrap();

        match test.mach_services {
            Some(events) => assert_eq!(events, check),
            _ => panic!("No launch events found"),
        };
    }
}

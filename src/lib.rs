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
//!     let launchd = Launchd::new("LABEL", "./foo/bar.txt")?
//!             .with_user_name("Henk")
//!             .with_program_arguments(vec!["Hello".to_string(), "World!".to_string()])
//!             .with_start_calendar_intervals(vec![ci])
//!             .disabled();
//!
//!     #[cfg(feature="io")] // Default
//!     return launchd.to_writer_xml(std::io::stdout());
//!     
//!     #[cfg(not(feature="io"))] // If you don't want to build any optional dependencies
//!     return Ok(());
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

mod error;

pub use self::error::Error;
#[cfg(feature = "cron")]
use cron::{Schedule, TimeUnitSpec};
#[cfg(feature = "io")]
use plist::{from_bytes, from_file, from_reader, from_reader_xml};
#[cfg(feature = "io")]
use plist::{to_file_binary, to_file_xml, to_writer_binary, to_writer_xml};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "cron")]
use std::convert::TryInto;
#[cfg(feature = "io")]
use std::io::{Read, Seek, Write};
use std::path::Path;

/// Representation of a launchd.plist file.
/// The definition of which can be found [here](https://www.manpagez.com/man/5/launchd.plist/).
///
/// Usage:
/// ```
/// use launchd::{Launchd, Error, CalendarInterval};
/// use std::path::Path;
///
/// fn example() -> Result<Launchd, Error> {
///     Ok(Launchd::new("LABEL", Path::new("./foo/bar.txt"))?
///         .with_user_name("Henk")
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
// TODO: remove owned Strings (?)
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "io", serde(rename_all = "PascalCase"))]
#[derive(Debug, PartialEq, Eq)]
pub struct Launchd {
    label: String,
    disabled: Option<bool>,
    user_name: Option<String>,
    group_name: Option<String>,
    // inetdCompatibility: Option<(?)>,
    // LimitLoadToHosts: Option<Vec<String>>,
    // LimitLoadFromHosts: Option<Vec<String>>,
    // LimitLoadToSessionType: Option<String>,
    program: String,
    program_arguments: Option<Vec<String>>,
    // EnableGlobbing: Option<bool>,
    // EnableTransactions: Option<bool>,
    // OnDemand: Option<bool>, NB: deprecated (see KeepAlive)
    // KeepAlive: Option<(?)>,
    run_at_load: Option<bool>,
    // RootDirectory: Option<String>, NB: from path
    // WorkingDirectory: Option<String>, NB: from path
    // EnvironmentVariables: Option<String>
    // Unmask: Option<u32> NB: check mode_t size in <sys/types.h>
    // TimeOut: Option<u32>
    // ExitTimeOut: Option<u32>
    // ThrottleInterval: Option<u32>
    // InitGroups: Option<bool>
    watch_paths: Option<Vec<String>>,
    queue_directories: Option<Vec<String>>,
    start_on_mount: Option<bool>,
    start_interval: Option<u32>,
    start_calendar_intervals: Option<Vec<CalendarInterval>>,
    // StandardInPath: Option<String> NB: from path
    // StandardOutPath: Option<String> NB: from path
    // StandardErrorPath: Option<String> NB: from path
    // ...
}

/// Representation of a CalendarInterval
///
/// Usage:
/// ```
/// use launchd::{CalendarInterval, Error};
/// fn example() -> Result<(), Error> {
///     let calendarinterval = CalendarInterval::default()
///             .with_hour(12)?
///             .with_minute(0)?
///             .with_weekday(7);
///     Ok(())
/// }
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "io", serde(rename_all = "PascalCase"))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct CalendarInterval {
    minute: Option<u8>,
    hour: Option<u8>,
    day: Option<u8>,
    weekday: Option<u8>,
    month: Option<u8>,
}

// TODO: This can be generated by a macro (maybe derive_builder?)
impl Launchd {
    pub fn new<S: AsRef<str>, P: AsRef<Path>>(label: S, program: P) -> Result<Self, Error> {
        let pathstr = program
            .as_ref()
            .to_str()
            .ok_or(Error::PathConversion)?
            .to_owned();
        Ok(Launchd {
            label: String::from(label.as_ref()),
            disabled: None,
            user_name: None,
            group_name: None,
            program: pathstr,
            program_arguments: None,
            run_at_load: None,
            watch_paths: None,
            queue_directories: None,
            start_on_mount: None,
            start_interval: None,
            start_calendar_intervals: None,
        })
    }

    pub fn with_label<S: AsRef<str>>(mut self, label: S) -> Self {
        self.label = String::from(label.as_ref());
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = Some(disabled);
        self
    }

    pub fn disabled(self) -> Self {
        self.with_disabled(true)
    }

    pub fn with_user_name<S: AsRef<str>>(mut self, user_name: S) -> Self {
        self.user_name = Some(String::from(user_name.as_ref()));
        self
    }

    pub fn with_group_name<S: AsRef<str>>(mut self, group_name: S) -> Self {
        self.group_name = Some(String::from(group_name.as_ref()));
        self
    }

    pub fn with_program<P: AsRef<Path>>(mut self, program: P) -> Result<Self, Error> {
        let pathstr = program
            .as_ref()
            .to_str()
            .ok_or(Error::PathConversion)?
            .to_owned();
        self.program = pathstr;
        Ok(self)
    }

    pub fn with_program_arguments(mut self, program_arguments: Vec<String>) -> Self {
        self.program_arguments = Some(program_arguments);
        self
    }

    pub fn with_run_at_load(mut self, run_at_load: bool) -> Self {
        self.run_at_load = Some(run_at_load);
        self
    }

    pub fn run_at_load(self) -> Self {
        self.with_run_at_load(true)
    }

    pub fn with_queue_directories(mut self, queue_directories: Vec<String>) -> Self {
        self.queue_directories = Some(queue_directories);
        self
    }

    pub fn with_watch_paths(mut self, watch_paths: Vec<String>) -> Self {
        self.watch_paths = Some(watch_paths);
        self
    }

    pub fn with_start_on_mount(mut self, start_on_mount: bool) -> Self {
        self.start_on_mount = Some(start_on_mount);
        self
    }

    pub fn start_on_mount(self) -> Self {
        self.with_start_on_mount(true)
    }

    pub fn with_start_interval(mut self, start_interval: u32) -> Self {
        self.start_interval = Some(start_interval);
        self
    }

    pub fn with_start_calendar_intervals(
        mut self,
        start_calendar_intervals: Vec<CalendarInterval>,
    ) -> Self {
        self.start_calendar_intervals = Some(start_calendar_intervals);
        self
    }
}

#[cfg(feature = "io")]
impl Launchd {
    // Write --
    pub fn to_writer_xml<W: Write>(&self, writer: W) -> Result<(), Error> {
        to_writer_xml(writer, self).map_err(Error::Write)
    }

    pub fn to_file_xml<P: AsRef<Path>>(&self, file: P) -> Result<(), Error> {
        to_file_xml(file, self).map_err(Error::Write)
    }

    pub fn to_writer_binary<W: Write>(&self, writer: W) -> Result<(), Error> {
        to_writer_binary(writer, self).map_err(Error::Write)
    }

    pub fn to_file_binary<P: AsRef<Path>>(&self, file: P) -> Result<(), Error> {
        to_file_binary(file, self).map_err(Error::Write)
    }

    // Read --
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        from_bytes(bytes).map_err(Error::Read)
    }

    pub fn from_file<P: AsRef<Path>>(file: P) -> Result<Self, Error> {
        from_file(file).map_err(Error::Read)
    }

    pub fn from_reader<R: Read + Seek>(reader: R) -> Result<Self, Error> {
        from_reader(reader).map_err(Error::Read)
    }

    pub fn from_reader_xml<R: Read + Seek>(reader: R) -> Result<Self, Error> {
        from_reader_xml(reader).map_err(Error::Read)
    }
}

impl CalendarInterval {
    #[cfg(feature = "cron")] // This has some use for launchd::with_start_calendar_intervals as well
    fn is_initialized(&self) -> bool {
        self.minute.is_some()
            || self.hour.is_some()
            || self.day.is_some()
            || self.weekday.is_some()
            || self.month.is_some()
    }

    pub fn with_minute(self, minute: u8) -> Result<Self, Error> {
        if minute > 59 {
            Err(Error::CalendarFieldOutOfBounds(0..=59, minute))
        } else {
            let mut result = self;
            result.minute = Some(minute);
            Ok(result)
        }
    }

    pub fn with_hour(self, hour: u8) -> Result<Self, Error> {
        if hour > 23 {
            Err(Error::CalendarFieldOutOfBounds(0..=23, hour))
        } else {
            let mut result = self;
            result.hour = Some(hour);
            Ok(result)
        }
    }

    pub fn with_day(self, day: u8) -> Result<Self, Error> {
        if day == 0 || day > 31 {
            Err(Error::CalendarFieldOutOfBounds(1..=31, day))
        } else {
            let mut result = self;
            result.day = Some(day);
            Ok(result)
        }
    }

    pub fn with_weekday(self, weekday: u8) -> Result<Self, Error> {
        if weekday > 7 {
            Err(Error::CalendarFieldOutOfBounds(0..=7, weekday))
        } else {
            let mut result = self;
            result.weekday = Some(weekday);
            Ok(result)
        }
    }

    pub fn with_month(self, month: u8) -> Result<Self, Error> {
        if month == 0 || month > 12 {
            Err(Error::CalendarFieldOutOfBounds(1..=12, month))
        } else {
            let mut result = self;
            result.month = Some(month);
            Ok(result)
        }
    }

    #[cfg(feature = "cron")]
    pub fn from_cron_schedule(schedule: Schedule) -> Result<Vec<Self>, Error> {
        let mut result_vec = Vec::new();
        for month in schedule.months().iter() {
            for weekday in schedule.days_of_week().iter() {
                for day in schedule.days_of_month().iter() {
                    for hour in schedule.hours().iter() {
                        for minute in schedule.minutes().iter() {
                            let result = Self::default();

                            // TODO: clean this mess up (thiserror + anyhow ?)
                            if !schedule.months().is_all() {
                                result.with_month(
                                    month
                                        .try_into()
                                        .map_err(|_| Error::InvalidCronField(month))?,
                                )?;
                            }
                            if !schedule.days_of_week().is_all() {
                                result.with_weekday(
                                    weekday
                                        .try_into()
                                        .map_err(|_| Error::InvalidCronField(weekday))?,
                                )?;
                            }
                            if !schedule.days_of_month().is_all() {
                                result.with_day(
                                    day.try_into().map_err(|_| Error::InvalidCronField(day))?,
                                )?;
                            }
                            if !schedule.hours().is_all() {
                                result.with_hour(
                                    hour.try_into().map_err(|_| Error::InvalidCronField(hour))?,
                                )?;
                            }
                            if !schedule.minutes().is_all() {
                                result.with_minute(
                                    minute
                                        .try_into()
                                        .map_err(|_| Error::InvalidCronField(minute))?,
                                )?;
                            }

                            if result.is_initialized() {
                                result_vec.push(result);
                            }

                            if schedule.minutes().is_all() {
                                break;
                            }
                        }
                        if schedule.hours().is_all() {
                            break;
                        }
                    }
                    if schedule.days_of_month().is_all() {
                        break;
                    }
                }
                if schedule.days_of_week().is_all() {
                    break;
                }
            }
            if schedule.months().is_all() {
                break;
            }
        }
        Ok(result_vec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_valid_launchd() {
        let check = Launchd {
            label: "Label".to_string(),
            disabled: None,
            user_name: None,
            group_name: None,
            program: "./henk.sh".to_string(),
            program_arguments: None,
            run_at_load: None,
            watch_paths: None,
            queue_directories: None,
            start_on_mount: None,
            start_interval: None,
            start_calendar_intervals: None,
        };
        let test = Launchd::new("Label", "./henk.sh");
        assert!(test.is_ok());
        assert_eq!(test.unwrap(), check);
    }

    #[test]
    fn create_valid_calendar_interval() {
        let check = CalendarInterval {
            minute: Some(5),
            hour: Some(5),
            day: Some(5),
            weekday: Some(5),
            month: Some(5),
        };

        let test = CalendarInterval::default()
            .with_day(5)
            .and_then(|ci| ci.with_minute(5))
            .and_then(|ci| ci.with_hour(5))
            .and_then(|ci| ci.with_weekday(5))
            .and_then(|ci| ci.with_month(5));

        assert!(test.is_ok());
        assert_eq!(test.unwrap(), check);
    }

    #[test]
    fn create_invalid_calendar_interval() {
        let test = CalendarInterval::default()
            .with_day(32)
            .and_then(|ci| ci.with_minute(5))
            .and_then(|ci| ci.with_hour(5))
            .and_then(|ci| ci.with_weekday(5))
            .and_then(|ci| ci.with_month(5));
        assert!(test.is_err());
        eprintln!("{}", test.unwrap_err());
    }
}

//! A Rust library for creating and parsing Launchd files.
//! It's still in early development, all help is welcome.
//!
//! ## Example
//!
//! ``` rust
//! use std::path::Path;
//! use launchd::{CalendarInterval, Error, Launchd};
//! fn main() -> Result<(), Error> {
//!     let ci = CalendarInterval::default()
//!         .with_hour(12)?
//!         .with_minute(10)?
//!         .with_weekday(7)?;
//!
//!     let launchd = Launchd::new("LABEL".to_string(), Path::new("./foo/bar.txt"))?
//!             .with_user_name("Henk".to_string())
//!             .with_program_arguments(vec!["Hello".to_string(), "World!".to_string()])
//!             .with_start_calendar_intervals(vec![ci])
//!             .disabled();
//!     
//!     launchd.to_writer_xml(std::io::stdout())
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

use std::path::Path;
use thiserror::Error;
#[cfg(feature="cron")]
use std::convert::TryInto;
#[cfg(feature="cron")]
use cron::{Schedule, TimeUnitSpec};
#[cfg(feature="io")]
use plist::*;
#[cfg(feature="io")]
use std::io::{Read, Write, Seek};
#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};

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
#[derive(Debug, PartialEq)]
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
///             .with_minute(60)?
///             .with_weekday(7);
///     Ok(())
/// }
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "io", serde(rename_all = "PascalCase"))]
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct CalendarInterval {
    minute: Option<u8>,
    hour: Option<u8>,
    day: Option<u8>,
    weekday: Option<u8>,
    month: Option<u8>,
    #[serde(skip_serializing)]
    initialized: bool,
}

// TODO implement debug
#[derive(Debug, Error)]
pub enum Error {
    #[error("CalendarField {0:?} is an invalid value: {1}")]
    CalendarFieldOutOfBounds(CalendarIntervalField, u8),
    #[error("The path could not be parsed")] // TODO: Show path. Is this really needed (invalid paths are not rejected)
    PathConversion,

    #[cfg(feature="cron")]
    #[error("The crontab generated an invalid value for {0:?}: {1}")]
    InvalidCronField(CalendarIntervalField, u32), // TODO: Change u32 to cron::Ordinal when possible. See: https://github.com/zslayton/cron/issues/82

    #[cfg(feature="io")]
    #[error(transparent)]
    Read(plist::Error),
    #[cfg(feature="io")]
    #[error(transparent)]
    Write(plist::Error),
}

#[derive(Debug)]
pub enum CalendarIntervalField {
    Minute,
    Hour,
    Day,
    Weekday,
    Month,
}

// TODO: This can be generated by a macro (maybe derive_builder?)
impl Launchd {
    pub fn new<S:AsRef<str>, P: AsRef<Path>>(label: S, program: P) -> Result<Self, Error> {
        let pathstr = program.as_ref()
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

    pub fn with_disabled(mut self, disabled: bool) -> Self{
        self.disabled = Some(disabled);
        self
    }

    pub fn disabled(self) -> Self {
        self.with_disabled(true)
    }

   pub fn with_user_name<S:AsRef<str>>(mut self, user_name: S) -> Self {
       self.user_name = Some(String::from(user_name.as_ref()));
       self
   }

   pub fn with_group_name<S:AsRef<str>>(mut self, group_name: S) -> Self {
       self.group_name = Some(String::from(group_name.as_ref()));
       self
   }

   pub fn with_program<P: AsRef<Path>>(mut self, program: P) -> Result<Self, Error> {
       let pathstr = program.as_ref().to_str().ok_or(Error::PathConversion)?
       .to_owned();
       self.program = pathstr;
       Ok(self)
   }

    pub fn with_program_arguments(mut self, program_arguments: Vec<String>) -> Self {
        self.program_arguments = Some(program_arguments);
        self
    }

    pub fn run_at_load(mut self) -> Self {
        self.run_at_load = Some(true);
        self
    }

    pub fn with_run_at_load(mut self, run_at_load: bool) -> Self {
        self.run_at_load = Some(run_at_load);
        self
    }

    pub fn with_queue_directories(
        mut self, 
        queue_directories: Vec<String>
    ) -> Self {
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

    pub fn with_start_calendar_intervals(mut self, start_calendar_intervals: Vec<CalendarInterval>) -> Self {
        self.start_calendar_intervals = Some(start_calendar_intervals);
        self
    }
}

#[cfg(feature="io")]
impl Launchd {
    // Write --
    pub fn to_writer_xml<W: Write>(&self, writer: W) -> Result<(), Error>{
        to_writer_xml(writer, self).map_err(|e| Error::Write(e) )
    }

    pub fn to_file_xml<P: AsRef<Path>>(&self, file: P) -> Result<(), Error>{
        to_file_xml(file, self).map_err(|e| Error::Write(e) )
    }

    pub fn to_writer_binary<W: Write>(&self, writer: W) -> Result<(), Error>{
        to_writer_binary(writer, self).map_err(|e| Error::Write(e) )
    }

    pub fn to_file_binary<P: AsRef<Path>>(&self, file: P) -> Result<(), Error>{
        to_file_binary(file, self).map_err(|e| Error::Write(e) )
    }

    // Read --
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error>{
        from_bytes(bytes).map_err(|e| Error::Read(e) )
    }

    pub fn from_file<P: AsRef<Path>>(file: P) -> Result<Self, Error>{
        from_file(file).map_err(|e| Error::Read(e) )
    }

    pub fn from_reader<R: Read + Seek>(reader: R) -> Result<Self, Error>{
        from_reader(reader).map_err(|e| Error::Read(e) )
    }

    pub fn from_reader_xml<R: Read + Seek>(reader: R) -> Result<Self, Error>{
        from_reader_xml(reader).map_err(|e| Error::Read(e) )
    }
}

impl CalendarInterval {
    pub fn with_minute(mut self, minute: u8) -> Result<Self, Error> {
        if minute > 59 {
            Err(Error::CalendarFieldOutOfBounds(CalendarIntervalField::Minute, minute))
        } else {
            self.minute = Some(minute);
            self.initialized = true;
            Ok(self)
        }
    }

    pub fn with_hour(self, hour: u8) -> Result<Self, Error> {
        if hour > 23 {
            Err(Error::CalendarFieldOutOfBounds(CalendarIntervalField::Hour, hour))
        } else {
            let mut result = self;
            result.hour = Some(hour);
            result.initialized = true;
            Ok(result)
        }
    }

    pub fn with_day(self, day: u8) -> Result<Self, Error> {
        if day == 0 || day > 31 {
            Err(Error::CalendarFieldOutOfBounds(CalendarIntervalField::Day, day))
        } else {
            let mut result = self;
            result.day = Some(day);
            result.initialized = true;
            Ok(result)
        }
    }

    pub fn with_weekday(self, weekday: u8) -> Result<Self, Error> {
        if weekday > 7 {
            Err(Error::CalendarFieldOutOfBounds(CalendarIntervalField::Weekday, weekday))
        } else {
            let mut result = self;
            result.weekday = Some(weekday);
            result.initialized = true;
            Ok(result)
        }
    }

    pub fn with_month(self, month: u8) -> Result<Self, Error> {
        if month == 0 || month > 12 {
            Err(Error::CalendarFieldOutOfBounds(CalendarIntervalField::Month, month))
        } else {
            let mut result = self;
            result.month = Some(month);
            result.initialized = true;
            Ok(result)
        }
    }

    #[cfg(feature="cron")]
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
                                    month.try_into().map_err(|_| 
                                        Error::InvalidCronField(CalendarIntervalField::Month, month) 
                                    )?
                                )?; 
                            }
                            if !schedule.days_of_week().is_all() { 
                                result.with_weekday(
                                    weekday.try_into().map_err(|_| 
                                        Error::InvalidCronField(CalendarIntervalField::Weekday, weekday) 
                                    )?
                                )?; 
                            }
                            if !schedule.days_of_month().is_all() { 
                                result.with_day(
                                    day.try_into().map_err(|_| 
                                        Error::InvalidCronField(CalendarIntervalField::Day, day) 
                                    )?
                                )?; 
                            }
                            if !schedule.hours().is_all() { 
                                result.with_hour(
                                    hour.try_into().map_err(|_| 
                                        Error::InvalidCronField(CalendarIntervalField::Hour, hour) 
                                    )?
                                )?; 
                            }
                            if !schedule.minutes().is_all() { 
                                result.with_minute(
                                    minute.try_into().map_err(|_| 
                                        Error::InvalidCronField(CalendarIntervalField::Minute, minute) 
                                    )?
                                )?; 
                            }

                            if result.initialized { result_vec.push(result); }

                            if schedule.minutes().is_all() { break;}
                        }
                        if schedule.hours().is_all() { break; }
                    }
                    if schedule.days_of_month().is_all() { break; }
                }
                if schedule.days_of_week().is_all() {break;}
            }
            if schedule.months().is_all() { break; }
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
        let test = Launchd::new("Label".to_string(), "./henk.sh");
        assert!(test.is_ok());
        assert_eq!(test.unwrap(), check);
    }

    #[test]
    fn create_valid_calendar_interval(){
        let check = CalendarInterval {
            minute: Some(5),
            hour: Some(5),
            day: Some(5),
            weekday: Some(5),
            month: Some(5),
            initialized: true,
        };

        let test = CalendarInterval::default()
            .with_day(5)
            .and_then(|ci| ci.with_minute(5))
            .and_then(|ci| ci.with_day(5))
            .and_then(|ci| ci.with_hour(5))
            .and_then(|ci| ci.with_weekday(5))
            .and_then(|ci| ci.with_month(5));
        
        assert!(test.is_ok());
        assert_eq!(test.unwrap(), check);
    }
}

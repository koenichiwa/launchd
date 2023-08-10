use std::{ops::RangeInclusive, path::PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("CalendarField field with value: {1} should lie in inclusive range: {0:?}")]
    CalendarFieldOutOfBounds(RangeInclusive<u8>, u8),
    #[error("The path could not be parsed {0:?}")]
    // TODO: Show path. Is this really needed (invalid paths are not rejected)
    PathConversion(PathBuf),
    #[error(transparent)]
    Read(plist::Error),
    #[error(transparent)]
    Write(plist::Error),
    #[error("Could not read value {0}")]
    EnumDeserialization(String),
    #[cfg(feature = "cron")]
    #[error("The crontab generated an invalid value: {0}")]
    InvalidCronField(u32),
}

use std::fmt;
use std::ops::RangeInclusive;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("CalendarField field with value: {1} should lie in inclusive range: {0:?}")]
    CalendarFieldOutOfBounds(RangeInclusive<u8>, u8),
    #[error("The path could not be parsed")]
    // TODO: Show path. Is this really needed (invalid paths are not rejected)
    PathConversion,

    #[cfg(feature = "cron")]
    #[error("The crontab generated an invalid value: {0}")]
    InvalidCronField(u32), // TODO: Change u32 to cron::Ordinal when possible. See: https://github.com/zslayton/cron/issues/82

    #[cfg(feature = "io")]
    #[error(transparent)]
    Read(plist::Error),
    #[cfg(feature = "io")]
    #[error(transparent)]
    Write(plist::Error),
}

// Errors for deserializing Strings into enums that have invalid values.
pub struct EnumDeserializationFromStrError;

impl fmt::Display for EnumDeserializationFromStrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("EnumDeserializationFromStrError")
    }
}

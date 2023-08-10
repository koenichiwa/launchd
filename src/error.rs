use std::ops::RangeInclusive;
use thiserror::Error;

/// Custom error types for handling various errors related to configuration parsing and processing.
#[derive(Debug, Error)]
pub enum Error {
    /// Indicates that a calendar field value is out of the specified inclusive range.
    #[error("CalendarField value: {1} is out of the inclusive range: {0:?}")]
    CalendarFieldOutOfBounds(RangeInclusive<u8>, u8),

    /// Represents errors that occur during reading operations.
    #[error(transparent)]
    Read(plist::Error),

    /// Represents errors that occur during writing operations.
    #[error(transparent)]
    Write(plist::Error),

    /// Indicates a failed deserialization attempt for an enumeration value.
    #[error("Failed to deserialize the enumeration value: {0}")]
    EnumDeserialization(String),

    /// Represents an error when the crontab generates an invalid value.
    #[cfg(feature = "cron")]
    #[error("The generated crontab value is invalid: {0}")]
    InvalidCronField(u32),
}

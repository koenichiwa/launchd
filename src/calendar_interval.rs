use serde::{Deserialize, Serialize};

#[cfg(feature = "cron")]
use cron::{Schedule, TimeUnitSpec};
#[cfg(feature = "cron")]
use std::convert::TryInto;

use crate::Error;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Default, Hash)]
#[serde(rename_all = "PascalCase")]
pub struct CalendarInterval {
    minute: Option<u8>,
    hour: Option<u8>,
    day: Option<u8>,
    weekday: Option<u8>,
    month: Option<u8>,
}

impl CalendarInterval {
    pub fn new() -> Self {
        Self::default()
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
}

#[cfg(feature = "cron")]
impl CalendarInterval {
    // This has some use for launchd::with_start_calendar_intervals as well
    fn is_initialized(&self) -> bool {
        self.minute.is_some()
            || self.hour.is_some()
            || self.day.is_some()
            || self.weekday.is_some()
            || self.month.is_some()
    }

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
mod test {
    use super::CalendarInterval;
    use super::Error;

    #[test]
    fn create_valid_calendar_interval() -> Result<(), Error> {
        let check = CalendarInterval {
            minute: Some(5),
            hour: Some(5),
            day: Some(5),
            weekday: Some(5),
            month: Some(5),
        };

        let test = CalendarInterval::default()
            .with_day(5)?
            .with_minute(5)?
            .with_hour(5)?
            .with_weekday(5)?
            .with_month(5)?;

        assert_eq!(test, check);
        Ok(())
    }

    #[test]
    fn create_invalid_calendar_interval() {
        let exp_value = 32;
        let test = CalendarInterval::default()
            .with_day(exp_value)
            .and_then(|ci| ci.with_minute(5))
            .and_then(|ci| ci.with_hour(5))
            .and_then(|ci| ci.with_weekday(5))
            .and_then(|ci| ci.with_month(5));
        assert!(test.is_err());
        eprintln!("{}", test.unwrap_err());
    }
}

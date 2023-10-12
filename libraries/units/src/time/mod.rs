// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::fmt::{Display, Formatter};

pub use datetime::*;
pub use duration::*;
pub use epoch::*;
pub use gregorian::*;
pub use julian::*;

use crate::bounds::{GreaterThanEqualToValueError, LessThanValue, Range};

mod datetime;
mod duration;
mod epoch;
mod gregorian;
mod julian;

///
/// Represents a time of the day, an offset into the day from midnight.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Time {
    second_of_day: u32,
}

impl Time {
    pub fn new(second_of_day: u32) -> Result<Time, GreaterThanEqualToValueError<u32>> {
        LessThanValue::new(86401).check_value_is_valid(&second_of_day)?;
        Ok(Time { second_of_day })
    }

    ///
    /// Returns the number of seconds into the "current day"
    #[must_use]
    pub fn as_seconds(&self) -> u32 {
        self.second_of_day
    }

    ///
    /// Converts this time into a duration
    #[must_use]
    pub fn as_duration(&self) -> Duration {
        Duration::new(self.second_of_day as f64, DurationUnit::Second)
    }

    ///
    /// Returns the number of hours represented by this time.
    #[must_use]
    pub fn as_hours(&self) -> u32 {
        (self.second_of_day as f64 / SECONDS_IN_HOUR as f64) as u32
    }

    ///
    /// Returns the number minutes
    #[must_use]
    pub fn as_minutes(&self) -> u32 {
        (self.second_of_day as f64 / SECONDS_IN_MINUTE as f64) as u32
    }

    ///
    /// Returns a triplet, (hours, minutes, seconds) representing this time
    #[must_use]
    pub fn as_hms(&self) -> (u32, u32, u32) {
        let hours = self.as_hours();
        let minutes = self.as_minutes() - hours * MINUTES_IN_HOUR;
        let seconds = self.as_seconds() - hours * SECONDS_IN_HOUR - minutes * SECONDS_IN_MINUTE;
        (hours, minutes, seconds)
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (h, m, s) = self.as_hms();
        f.write_fmt(format_args!("{h:0}:{m:02}:{s:02}"))
    }
}

impl From<Time> for Duration {
    fn from(value: Time) -> Self {
        value.as_duration()
    }
}

pub const HOURS_IN_DAY: u32 = 24;
pub const MINUTES_IN_HOUR: u32 = 60;
pub const SECONDS_IN_MINUTE: u32 = 60;
pub const MINUTES_IN_DAY: u32 = 1440;
pub const SECONDS_IN_HOUR: u32 = 3600;

///
/// Generally 86400, but occasionally 86401 for leap seconds.
pub const SECONDS_IN_DAY: u32 = 86400;

///
/// 32 Bit Fixed Precision Time Format, storing 16 bits of Seconds, and 16 bits
/// of Fractional Seconds.  This is the equivalent of Q16.16, and is semantically
/// equivalent to the NTP Short Format if using the NTP Epoch.
///
/// The 16-bit seconds field can resolve a little over 18 hours, and the
/// 16-bit fractional seconds field can resolve a little over 15 microseconds.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Time32 {
    /// The Reference Epoch
    epoch: Epoch,

    /// The number of seconds into the reference epoch
    seconds: u16,

    /// The fractional number of seconds into the current second.  Divide this
    /// number by 2^16 to get the actual fractional component.
    fractional_seconds: u16,
}
impl Time32 {
    #[must_use]
    pub fn new(epoch: Epoch, seconds: u16, fractional_seconds: u16) -> Self {
        Self {
            epoch,
            seconds,
            fractional_seconds,
        }
    }

    ///
    /// Returns the value of this Time32 as a Q16.16
    #[must_use]
    pub fn as_u32(&self) -> u32 {
        ((self.seconds as u32) << 16) | (self.fractional_seconds as u32)
    }
}

///
/// 64 Bit Fixed Precision Time Format, storing 32 bits of Seconds, and 32 bits
/// of Fractional Seconds.  This is the equivalent of Q32.32, and is semantically
/// equivalent to the NTP Timestamp Format if using the NTP Epoch.
///
/// The 32-bit seconds field can resolve 136 years, and the 32-bit fractional field
/// can resolve down to 232 picoseconds.
///
/// The raw value is 64 bits wide, if you take the middle 32
/// bits, this is identical to a [`Time32`] - (lower 16 of `seconds`, upper 16 of
/// `fractional_seconds`).
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Time64 {
    /// The Reference Epoch
    epoch: Epoch,

    /// The number of seconds into the current reference epoch
    seconds: u32,

    /// The fractional element into the current second.  Divide this number by
    /// 2^32 to get the actual fractional component.
    fractional_seconds: u32,
}

impl Time64 {
    #[must_use]
    pub fn new(epoch: Epoch, seconds: u32, fractional_seconds: u32) -> Self {
        Self {
            epoch,
            seconds,
            fractional_seconds,
        }
    }

    ///
    /// Returns the value of this Time64 as a Q32.32
    #[must_use]
    pub fn as_u64(&self) -> u64 {
        ((self.seconds as u64) << 32) | (self.fractional_seconds as u64)
    }

    ///
    /// Returns the reference epoch of this Time64
    #[must_use]
    pub fn get_epoch(&self) -> Epoch {
        self.epoch
    }
}

///
/// 128 Bit Fixed Precision Time Format, storing 64 bits of Seconds, and 64 bits
/// of Fractional Seconds.  This is the equivalent of Q64.64, and is semantically
/// equivalent to the NTP Datestamp Format if using the [`NTP_EPOCH`].
///
/// The 64-bit seconds field can resolve 584 million years, and the 64-bit
/// fractional field can resolve down to 54 zepto-seconds (5.4e-20).
///
/// 580 million years ago, multicellular life started.  580 million years from,
/// now, the average temperature of the Earth will be 25C higher - 40C.
///
/// The raw value is 128 bits wide, if you take the middle 64 bits, this is
/// identical to a [`Time64`] - (lower 32 of `seconds`, upper 32 of
/// `fractional_seconds`).
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Time128 {
    ///
    /// Reference Epoch Date
    epoch: Epoch,

    /// The number of seconds into the reference epoch
    seconds: u64,

    /// The fractional element into the current second.  Divide this number by
    /// 2^64 to get the actual fractional component.
    fractional_seconds: u64,
}

impl Time128 {
    #[must_use]
    pub fn new(epoch: Epoch, seconds: u64, fractional_seconds: u64) -> Self {
        Self {
            epoch,
            seconds,
            fractional_seconds,
        }
    }

    ///
    /// Returns the value of this Time128 as a Q64.64
    #[must_use]
    pub fn as_u128(&self) -> u128 {
        ((self.seconds as u128) << 64) | (self.fractional_seconds as u128)
    }

    ///
    /// Returns the reference epoch of this Time128
    #[must_use]
    pub fn get_epoch(&self) -> Epoch {
        self.epoch
    }
}

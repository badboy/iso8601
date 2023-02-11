//! ISO8601 is a parser library for the
//! [ISO8601](https://en.wikipedia.org/wiki/ISO_8601) format
//! and partially RFC3339.
//!
//! Validity of a given date is not guaranteed, this parser will happily parse
//! `"2015-02-29"` as a valid date,
//! even though 2015 was no leap year.
//!
//! # Example
//!
//! ```rust
//! let datetime = iso8601::datetime("2015-06-26T16:43:23+0200").unwrap();
//! ```

#![allow(clippy::uninlined_format_args)]
#![deny(
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces,
    unused_qualifications,
    missing_docs
)]
#![warn(clippy::doc_markdown)]
#![no_std]

#[cfg(any(feature = "std", test))]
#[macro_use]
extern crate std;

#[macro_use]
extern crate alloc;

use alloc::string::String;
use core::default::Default;
use core::str::FromStr;

mod display;
pub mod parsers;

#[cfg(feature = "chrono")]
mod chrono;

#[cfg(feature = "serde")]
mod serde;

#[cfg(test)]
mod assert;

/// A date, can hold three different formats.
#[allow(missing_docs)]
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Date {
    /// consists of year, month and day of month
    YMD { year: i32, month: u32, day: u32 },
    /// consists of year, week and day of week
    Week { year: i32, ww: u32, d: u32 },
    /// consists of year and day of year
    Ordinal { year: i32, ddd: u32 },
}

/// A time object.
#[derive(Eq, PartialEq, Debug, Copy, Clone, Default)]
pub struct Time {
    /// a 24th of a day
    pub hour: u32,
    /// 60 discrete parts of an hour
    pub minute: u32,
    /// a minute are 60 of these
    pub second: u32,
    /// everything after a `.`
    pub millisecond: u32,
    /// the hour part of the timezone offset from UTC
    pub tz_offset_hours: i32,
    /// the minute part of the timezone offset from UTC
    pub tz_offset_minutes: i32,
}

/// Compound struct, holds Date and Time.
#[derive(Eq, PartialEq, Debug, Copy, Clone, Default)]
pub struct DateTime {
    /// The date part
    pub date: Date,
    /// The time part
    pub time: Time,
}

/// A time duration.

/// Durations:
/// <https://www.rfc-editor.org/rfc/rfc3339#page-13>
///    dur-second        = 1*DIGIT "S"
///    dur-minute        = 1*DIGIT "M" [dur-second]
///    dur-hour          = 1*DIGIT "H" [dur-minute]
///    dur-time          = "T" (dur-hour / dur-minute / dur-second)
///    dur-day           = 1*DIGIT "D"
///    dur-week          = 1*DIGIT "W"
///    dur-month         = 1*DIGIT "M" [dur-day]
///    dur-year          = 1*DIGIT "Y" [dur-month]
///    dur-date          = (dur-day / dur-month / dur-year) [dur-time]
///    duration          = "P" (dur-date / dur-time / dur-week)
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Duration {
    /// A duration specified by year, month, day, hour, minute and second units
    YMDHMS {
        /// Number of calendar years
        year: u32,
        /// Number of months
        month: u32,
        /// Number of days
        day: u32,
        /// Number of hours
        hour: u32,
        /// Number of minutes
        minute: u32,
        /// Number of seconds
        second: u32,
        /// Number of milliseconds
        millisecond: u32,
    },
    /// consists of week units
    Weeks(u32),
}

impl Duration {
    /// Whether this duration represents a zero duration.
    pub fn is_zero(&self) -> bool {
        *self
            == Duration::YMDHMS {
                year: 0,
                month: 0,
                day: 0,
                hour: 0,
                minute: 0,
                second: 0,
                millisecond: 0,
            }
            || *self == Duration::Weeks(0)
    }
}

impl Time {
    /// Change this time's timezone offset.
    ///
    /// # Arguments
    ///
    /// * `tzo` - A tuple of `(hours, minutes)` specifying the timezone offset from UTC.
    pub fn set_tz(&self, tzo: (i32, i32)) -> Time {
        let mut t = *self;
        t.tz_offset_hours = tzo.0;
        t.tz_offset_minutes = tzo.1;
        t
    }
}

impl Default for Date {
    fn default() -> Date {
        Date::YMD {
            year: 0,
            month: 0,
            day: 0,
        }
    }
}

impl FromStr for Date {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        date(s)
    }
}

impl FromStr for Time {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        time(s)
    }
}

impl FromStr for DateTime {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        datetime(s)
    }
}

impl Default for Duration {
    fn default() -> Duration {
        Duration::YMDHMS {
            year: 0,
            month: 0,
            day: 0,
            hour: 0,
            minute: 0,
            second: 0,
            millisecond: 0,
        }
    }
}

impl FromStr for Duration {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        duration(s)
    }
}

impl From<Duration> for ::core::time::Duration {
    fn from(duration: Duration) -> Self {
        match duration {
            Duration::YMDHMS {
                year,
                month,
                day,
                hour,
                minute,
                second,
                millisecond,
            } => {
                let secs = u64::from(year) * 365 * 86_400
                    + u64::from(month) * 30 * 86_400
                    + u64::from(day) * 86_400
                    + u64::from(hour) * 3600
                    + u64::from(minute) * 60
                    + u64::from(second);
                let nanos = millisecond * 1_000_000;
                Self::new(secs, nanos)
            }
            Duration::Weeks(week) => {
                let secs = u64::from(week) * 7 * 86_400;
                Self::from_secs(secs)
            }
        }
    }
}

/// Parses a date string.
///
/// A string can have one of the following formats:
///
/// * `2015-11-02` or `20151102`
/// * `2015-W45-01` or `2015W451`
/// * `2015-306` or `2015306`
///
/// ## Example
///
/// ```rust
/// let date = iso8601::date("2015-11-02").unwrap();
/// ```
pub fn date(string: &str) -> Result<Date, String> {
    if let Ok((_, parsed)) = parsers::parse_date(string.as_bytes()) {
        Ok(parsed)
    } else {
        Err(format!("Failed to parse date: {}", string))
    }
}

/// Parses a time string.
///
/// A string can have one of the following formats:
///
/// * `07:35:[00][.123]` or `0735[00][.123]`
/// * `07:35:[00][.123][(Z|(+|-)00:00)]`
/// * `0735[00][.123][(Z|(+|-)00:00)]`
/// * `0735[00][.123][(Z|(+|-)0000)]`
///
/// ## Example
///
/// ```rust
/// let time = iso8601::time("21:56:42").unwrap();
/// ```
pub fn time(string: &str) -> Result<Time, String> {
    if let Ok((_, parsed)) = parsers::parse_time(string.as_bytes()) {
        Ok(parsed)
    } else {
        Err(format!("Failed to parse time: {}", string))
    }
}

/// Parses a datetime string.
///
/// A datetime string is a combination of the valid formats for the date and time,
/// separated by a literal `T`.
/// See the respective functions for the correct format.
///
/// ## Example
///
/// ```rust
/// let dt = iso8601::datetime("2015-11-03T21:56").unwrap();
/// ```
pub fn datetime(string: &str) -> Result<DateTime, String> {
    if let Ok((_left_overs, parsed)) = parsers::parse_datetime(string.as_bytes()) {
        Ok(parsed)
    } else {
        Err(format!("Failed to parse datetime: {}", string))
    }
}

/// Parses a duration string.
///
/// A string starts with `P` and can have one of the following formats:
///
/// * Fully-specified duration: `P1Y2M3DT4H5M6S`
/// * Duration in weekly intervals: `P1W`
/// * Fully-specified duration in [`DateTime`] format: `P<datetime>`
///
/// Both fully-specified formats get parsed into the YMDHMS Duration variant.
/// The weekly interval format gets parsed into the Weeks Duration variant.
///
/// The ranges for each of the individual units are not expected to exceed
/// the next largest unit.
///
/// These ranges (inclusive) are as follows:
///
/// * Year (any valid u32)
/// * Month 0 - 12
/// * Week 0 - 52
/// * Day 0 - 31
/// * Hour 0 - 24
/// * Minute 0 - 60
/// * Second 0 - 60
///
/// ## Examples
///
/// ```rust
/// let duration = iso8601::duration("P1Y2M3DT4H5M6S").unwrap();
/// let duration = iso8601::duration("P1W").unwrap();
/// let duration = iso8601::duration("P2015-11-03T21:56").unwrap();
/// ```
pub fn duration(string: &str) -> Result<Duration, String> {
    if let Ok((_left_overs, parsed)) = parsers::parse_duration(string.as_bytes()) {
        Ok(parsed)
    } else {
        Err(format!("Failed to parse duration: {}", string))
    }
}

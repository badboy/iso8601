use crate::{parsers, Date, Time};
use alloc::string::String;
use core::str::FromStr;

/// Compound struct, holds Date and Time.
/// ```
/// # use std::str::FromStr;
/// use winnow_iso8601::Timezone;
/// assert_eq!(
///     winnow_iso8601::DateTime::from_str("2023-02-18T17:08:08.793Z"),
///     Ok(winnow_iso8601::DateTime {
///         date: winnow_iso8601::Date::YMD{ year: 2023, month: 2, day: 18},
///         time: winnow_iso8601::Time{ hour: 17, minute: 8, second: 8, millisecond: 793, timezone: Timezone { offset_hours: 0, offset_minutes: 00 }}
///     })
/// )
/// ```
#[derive(Eq, PartialEq, Debug, Copy, Clone, Default)]
pub struct DateTime {
    /// The date part
    pub date: Date,
    /// The time part
    pub time: Time,
}

impl FromStr for DateTime {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        datetime(s)
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
/// let dt = winnow_iso8601::datetime("2015-11-03T21:56").unwrap();
/// ```
pub fn datetime(string: &str) -> Result<DateTime, String> {
    if let Ok(parsed) = parsers::parse_datetime(&mut string.as_bytes()) {
        Ok(parsed)
    } else {
        Err(format!("Failed to parse datetime: {}", string))
    }
}

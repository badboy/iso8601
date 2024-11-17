use core::str::FromStr;
use std::prelude::rust_2015::String;

/// Struct holding timezone offsets
/// ```
/// # use std::str::FromStr;
/// use winnow_iso8601::Timezone;
/// assert_eq!(
///     winnow_iso8601::Timezone::from_str("Z"),
///     Ok(Timezone {offset_hours: 0, offset_minutes: 00})
/// )
/// ```
#[derive(Eq, PartialEq, Debug, Copy, Clone, Default)]
pub struct Timezone {
    /// hour timezone offset
    pub offset_hours: i32,
    /// minute timezone offset
    pub offset_minutes: i32,
}

impl FromStr for Timezone {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        timezone(s)
    }
}

/// Parses a timezone offset string.
///
/// A timezone offset string is a combination of the valid formats specifying a time's UTC offset
///
/// This will accept (Z|+...|-...) as offsets
///
/// ## Example
///
/// ```rust
/// let dt = winnow_iso8601::timezone("Z").unwrap();
/// ```
pub fn timezone(string: &str) -> Result<Timezone, String> {
    if let Ok(parsed) = crate::parsers::parse_timezone(&mut string.as_bytes()) {
        Ok(parsed)
    } else {
        Err(format!("Failed to parse datetime: {}", string))
    }
}

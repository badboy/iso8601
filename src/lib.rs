//! ISO8601 is a parser library for the for [ISO8601](https://en.wikipedia.org/wiki/ISO_8601) and partially RFC3339.
//!
//! Validity of a given date is not guaranteed, this parser will happily 2015.02.29 as a valid
//! date, even though 2015 was no leap year.

#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

#[macro_use]
extern crate nom;
use nom::IResult::*;

#[macro_use]
mod helper;
pub mod parsers;

/// A date, can hold three different formats.
#[derive(Eq,PartialEq,Debug,Copy,Clone)]
pub enum Date {
    /// consists of year, month and day of month
    YMD{
        year:  i32,
        month: u32,
        day:   u32
    },
    /// consists of year, week and day of week
    Week{
        year:  i32,
        ww:    u32,
        d:     u32
    },
    /// consists of year and day of year
    Ordinal{
        year: i32,
        ddd: u32
    }
}

/// A time object
#[derive(Eq,PartialEq,Debug,Copy,Clone)]
pub struct Time {
    /// a 24th of a day
    pub hour: u32,
    /// 60 discrete parts of an hour 
    pub minute: u32,
    /// a minute are 60 of these
    pub second: u32,
    /// everything after a `.`
    pub millisecond: u32,
    /// depends on where you're at
    pub tz_offset_hours: i32,
    pub tz_offset_minutes: i32,
}

/// Compound struct, hold Date and Time
///
/// duh!
#[derive(Eq,PartialEq,Debug,Copy,Clone)]
pub struct DateTime {
    pub date: Date,
    pub time: Time,
}

impl Time {
    pub fn set_tz(&self, tzo: (i32,i32)) -> Time {
        let mut t = self.clone();
        t.tz_offset_hours = tzo.0;
        t.tz_offset_hours = tzo.1;
        t
    }
}


/// Parses a date string.
///
/// A string can have either of the following formats:
///
/// 1. `2015-11-02` or `20151102`
/// 2. `2015-W45-01` or `2015W451`
/// 3. `2015-306` or `2015306`
///
pub fn date(string:&str) -> Result<Date,String> {
    if let Done(_,parsed) =  parsers::parse_date(string.as_bytes()){
        Ok(parsed)
    }
    else {
        Err(format!("Parser Error: {}", string))
    }
}


/// Parses a time string.
///
/// A string can have either of the following formats:
///
/// 1. `07:35:[00][.123]` or `0735[00][.123]`
/// 1. `07:35:[00][.123][(Z|(+|-)00:00)]`
/// 1. `0735[00][.123][(Z|(+|-)00:00)]`
/// 1. `0735[00][.123][(Z|(+|-)0000)]`
pub fn time(string:&str) -> Result<Time,String> {
    if let Done(_,parsed) =  parsers::parse_time(string.as_bytes()){
        Ok(parsed)
    }
    else {
        Err(format!("Parser Error: {}", string))
    }
}


/// This parses a datetime string.
///
/// A string can have either of the following formats:
///
/// *A Date* `T` *a time* ( see `date()` and `time()` )
///
pub fn datetime(string:&str) -> Result<DateTime,String> {
    if let Done(_left_overs,parsed) = parsers::parse_datetime(string.as_bytes()){
        Ok(parsed)
    }
    else {
        Err(format!("Parser Error: {}", string))
    }
}

#![allow(dead_code)]

#[macro_use]
extern crate nom;
use nom::IResult::*;

#[macro_use]
pub mod macros;
mod helper;

#[derive(Eq,PartialEq,Debug)]
pub enum Date {
    YMD{
        year:  i32,
        month: u32,
        day:   u32
    },
    Week{
        year:  i32,
        ww:    u32,
        d:     u32
    },
    Ordinal{
        year: i32,
        ddd: u32
    }
}

#[derive(Clone,Eq,PartialEq,Debug)]
pub struct Time {
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
    pub tz_offset: i32,
}

#[derive(Eq,PartialEq,Debug)]
pub struct DateTime {
    pub date: Date,
    pub time: Time,
}

impl Time {
    pub fn set_tz(&self, tzo: i32) -> Time {
        let mut t = self.clone();
        t.tz_offset = tzo;
        t
    }
}


/// This parses a date string.
/// A string can have either of the following formats:
///
/// 1. YYYY-MM-DD or YYYYMMDD
/// 2. YYYY-*W*ww-DD or YYYY*W*wwDD
/// 3. YYYY-DDD or YYYYDDD
///
pub fn date(string:&str) -> Result<Date,String> {
    if let Done(_,parsed) =  macros::parse_date(string.as_bytes()){
        Ok(parsed)
    }
    else {
        Err(format!("Parser Error: {}", string))
    }
}


/// This parses a time string.
/// `HH:MM:[SS]`
pub fn time(string:&str) -> Result<Time,String> {
    if let Done(_,parsed) =  macros::parse_time(string.as_bytes()){
        Ok(parsed)
    }
    else {
        Err(format!("Parser Error: {}", string))
    }
}


/// This parses a datetime string.
pub fn datetime(string:&str) -> Result<DateTime,String> {
    if let Done(_,parsed) =  macros::parse_datetime(string.as_bytes()){
        Ok(parsed)
    }
    else {
        Err(format!("Parser Error: {}", string))
    }
}

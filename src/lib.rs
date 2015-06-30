#[macro_use]
extern crate nom;

use nom::IResult::*;
use nom::Err::*;
use std::fmt;

mod helper;
use helper::*;

#[macro_use] mod macros;

mod easy;

#[derive(Eq,PartialEq)]
pub struct Date {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

#[derive(Clone,Eq,PartialEq)]
pub struct Time {
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
    pub tz_offset: i32,
}

#[derive(Eq,PartialEq)]
pub struct DateTime {
    pub date: Date,
    pub time: Time,
}

impl fmt::Debug for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:0>4?}-{:0>2?}-{:0>2?}",
               self.year, self.month, self.day)
    }
}
impl fmt::Debug for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:0>2?}:{:0>2?}:{:0>2?}Z{:0>4?}",
               self.hour, self.minute, self.second, self.tz_offset*100)
    }
}
impl fmt::Debug for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}T{:?}", self.date, self.time)
    }
}

impl Time {
    pub fn set_tz(&self, tzo: i32) -> Time {
        let mut t = self.clone();
        t.tz_offset = tzo;
        t
    }
}

named!(year_prefix, alt!(tag!("+") | tag!("-")));

named!(pub year <&[u8], i32>, chain!(
        pref: opt!(year_prefix) ~
        year: take_n_between!(4, '0', '9')
        ,
        || {
            match pref {
                Some(b"-") => -buf_to_i32(year),
                _ => buf_to_i32(year)
            }
        }));

named!(lower_month <&[u8], u32>, chain!(tag!("0") ~ s:char_between!('1', '9') , || buf_to_u32(s)));
named!(upper_month <&[u8], u32>, chain!(tag!("1") ~ s:char_between!('0', '2') , || 10+buf_to_u32(s)));

named!(pub month <&[u8], u32>, alt!(lower_month | upper_month));

named!(day_zero <&[u8], u32>,  chain!(tag!("0") ~ s:char_between!('1', '9') , || buf_to_u32(s)));
named!(day_one <&[u8], u32>,   chain!(tag!("1") ~ s:char_between!('0', '9') , || 10+buf_to_u32(s)));
named!(day_two <&[u8], u32>,   chain!(tag!("2") ~ s:char_between!('0', '9') , || 20+buf_to_u32(s)));
named!(day_three <&[u8], u32>, chain!(tag!("3") ~ s:char_between!('0', '1') , || 30+buf_to_u32(s)));

named!(pub day <&[u8], u32>, alt!(day_zero | day_one | day_two | day_three));

named!(pub date <&[u8], Date>, chain!(
        y: year ~
        tag!("-") ~
        m: month ~
        tag!("-") ~
        d: day
        ,
        || { Date{ year: y, month: m, day: d } }
        ));


named!(lower_hour <&[u8], u32>, chain!(f:char_between!('0','1') ~ s:char_between!('0','9') ,
                                       || { buf_to_u32(f)*10 + buf_to_u32(s) } ));
named!(upper_hour <&[u8], u32>, chain!(tag!("2") ~ s:char_between!('0','4') , || 20+buf_to_u32(s)));
named!(pub hour <&[u8], u32>, alt!(lower_hour | upper_hour));

named!(below_sixty <&[u8], u32>, chain!(f:char_between!('0','5') ~ s:char_between!('0','9') ,
                                       || { buf_to_u32(f)*10 + buf_to_u32(s) } ));

named!(pub minute <&[u8], u32>, call!(below_sixty));
named!(pub second <&[u8], u32>, call!(below_sixty));

named!(pub time <&[u8], Time>, chain!(
        h: hour ~
        tag!(":") ~
        m: minute ~
        s: empty_or!(
            chain!(tag!(":") ~ s:second , || s)
            )
        ,
        || {
            Time {
                hour: h,
                minute: m,
                second: s.unwrap_or(0),
                tz_offset: 0
            }
        }
        ));

named!(timezone <&[u8], u32>, chain!(
        tag!("+") ~
        h: hour ~
        empty_or!(
            chain!(
                tag!(":")? ~ m: minute , || { m }
            ))
        ,
        || { h }));

named!(tz_z, tag!("Z"));
named!(timezone_utc <&[u8], u32>, map!(tz_z, |_| 0));

named!(pub time_with_timezone <&[u8], Time>, chain!(
        t: time ~
        s: empty_or!(alt!(timezone_utc | timezone))
        ,
        || {
            Time {
                hour: t.hour,
                minute: t.minute,
                second: t.second,
                tz_offset: s.unwrap_or(0) as i32
            }
        }
        ));

named!(pub datetime <&[u8], DateTime>, chain!(
        d: date ~
        tag!("T") ~
        t: time_with_timezone
        ,
        || {
            DateTime{
                date: d,
                time: t,
            }
        }
        ));

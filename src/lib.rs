#[macro_use]
extern crate nom;

use nom::IResult::*;
use nom::Err::*;
use std::fmt;

mod helper;
use helper::*;

#[derive(Eq,PartialEq)]
pub struct Date {
    year: i32,
    month: u32,
    day: u32,
}

#[derive(Eq,PartialEq)]
pub struct Time {
    hour: u32,
    minute: u32,
    second: u32,
    tz_offset: i32,
}

#[derive(Eq,PartialEq)]
pub struct DateTime {
    date: Date,
    time: Time,
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

/// Take n bytes and ensure that they are only in the provided range of bytes
macro_rules! take_n_between(
    ($input:expr, $count:expr, $min:expr, $max:expr) => (
        {
            let new_min = $min as u8;
            let new_max = $max as u8;
            let cnt = $count as usize;
            if $input.len() < cnt {
                nom::IResult::Incomplete(nom::Needed::Size(cnt))
            } else {
                for idx in 0..$count {
                    if $input[idx] < new_min || $input[idx] > new_max {
                        return nom::IResult::Error(nom::Err::Position(42 as u32,$input));
                    }
                }

                nom::IResult::Done(&$input[$count..], &$input[0..$count])
            }
        }
        );
    );

macro_rules! char_between(
    ($input:expr, $min:expr, $max:expr) => (
        take_n_between!($input, 1, $min, $max)
    );
);

macro_rules! empty_or(
    ($i:expr, $submac:ident!( $($args:tt)* )) => (
        if $i.len() == 0 {
            nom::IResult::Done($i, None)
        } else {
            match $submac!($i, $($args)*) {
                nom::IResult::Done(i,o)     => nom::IResult::Done(i, Some(o)),
                nom::IResult::Error(_)      => nom::IResult::Done($i, None),
                nom::IResult::Incomplete(i) => nom::IResult::Incomplete(i)

            }
        }
    );
);

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

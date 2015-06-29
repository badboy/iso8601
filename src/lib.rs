#[macro_use]
extern crate nom;

use nom::IResult;
use nom::IResult::*;
use nom::Err::*;
use std::fmt;

mod helper;
use helper::*;

macro_rules! errprint {
    ($($arg:tt)*) => (
        std::io::stderr().write_fmt(format_args!($($arg)*))
        );
}

macro_rules! errln {
    ($fmt:expr) => (errprint!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (errprint!(concat!($fmt, "\n"), $($arg)*));
}

#[allow(unused_imports)] use std::io::Write;


#[derive(Eq,PartialEq)]
struct Date {
    year: i32,
    month: u32,
    day: u32,
}

#[derive(Eq,PartialEq)]
struct Time {
    hour: u32,
    minute: u32,
    second: u32,
    tz_offset: i32,
}

#[derive(Eq,PartialEq)]
struct DateTime {
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
named!(upper_month <&[u8], u32>, chain!(tag!("1") ~ s:char_between!('0', '2') ,       || 10+buf_to_u32(s)));

named!(month <&[u8], u32>, alt!(lower_month | upper_month));

named!(day_zero <&[u8], u32>,  chain!(tag!("0") ~ s:char_between!('1', '9') ,  || buf_to_u32(s)));
named!(day_one <&[u8], u32>,   chain!(tag!("1") ~ s:char_between!('0', '9') , || 10+buf_to_u32(s)));
named!(day_two <&[u8], u32>,   chain!(tag!("2") ~ s:char_between!('0', '9') , || 20+buf_to_u32(s)));
named!(day_three <&[u8], u32>, chain!(tag!("3") ~ s:char_between!('0', '1') ,         || 30+buf_to_u32(s)));

named!(day <&[u8], u32>, alt!(day_zero | day_one | day_two | day_three));

named!(date <&[u8], Date>, chain!(
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
named!(hour <&[u8], u32>, alt!(lower_hour | upper_hour));

named!(below_sixty <&[u8], u32>, chain!(f:char_between!('0','5') ~ s:char_between!('0','9') ,
                                       || { buf_to_u32(f)*10 + buf_to_u32(s) } ));

named!(minute <&[u8], u32>, call!(below_sixty));
named!(second <&[u8], u32>, call!(below_sixty));

named!(append_seconds <&[u8], u32>, chain!(tag!(":") ~ sec:second, || sec));

named!(time <&[u8], Time>, chain!(
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
        m: empty_or!(
            chain!(
                tag!(":")? ~ m: minute , || { m }
            ))
        ,
        || { h }));

named!(tz_z, tag!("Z"));
named!(timezone_utc <&[u8], u32>, map!(tz_z, |_| 0));

named!(time_with_timezone <&[u8], Time>, chain!(
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

#[test]
fn parse_year() {
    assert_eq!(Done(&[][..], 2015), year(b"2015"));
    assert_eq!(Done(&[][..], -0333), year(b"-0333"));

    assert_eq!(Done(&b"-"[..], 2015), year(b"2015-"));

    assert!(year(b"abcd").is_err());
    assert!(year(b"2a03").is_err());
}

#[test]
fn parse_month() {
    assert_eq!(Done(&[][..], 1), month(b"01"));
    assert_eq!(Done(&[][..], 6), month(b"06"));
    assert_eq!(Done(&[][..], 12), month(b"12"));
    assert_eq!(Done(&b"-"[..], 12), month(b"12-"));

    assert!(month(b"13").is_err());
    assert!(month(b"00").is_err());
}

#[test]
fn parse_day() {
    assert_eq!(Done(&[][..], 1), day(b"01"));
    assert_eq!(Done(&[][..], 12), day(b"12"));
    assert_eq!(Done(&[][..], 20), day(b"20"));
    assert_eq!(Done(&[][..], 28), day(b"28"));
    assert_eq!(Done(&[][..], 30), day(b"30"));
    assert_eq!(Done(&[][..], 31), day(b"31"));
    assert_eq!(Done(&b"-"[..], 31), day(b"31-"));

    assert!(day(b"00").is_err());
    assert!(day(b"32").is_err());
}

#[test]
fn parse_date() {
    assert_eq!(Done(&[][..], Date{ year: 2015, month: 6, day: 26 }), date(b"2015-06-26"));
    assert_eq!(Done(&[][..], Date{ year: -333, month: 7, day: 11 }), date(b"-0333-07-11"));

    assert!(date(b"201").is_incomplete());
    assert!(date(b"2015p00p00").is_err());
    assert!(date(b"pppp").is_err());
}

#[test]
fn parse_hour() {
    assert_eq!(Done(&[][..], 0), hour(b"00"));
    assert_eq!(Done(&[][..], 1), hour(b"01"));
    assert_eq!(Done(&[][..], 6), hour(b"06"));
    assert_eq!(Done(&[][..], 12), hour(b"12"));
    assert_eq!(Done(&[][..], 13), hour(b"13"));
    assert_eq!(Done(&[][..], 20), hour(b"20"));
    assert_eq!(Done(&[][..], 24), hour(b"24"));

    assert!(hour(b"25").is_err());
    assert!(hour(b"30").is_err());
    assert!(hour(b"ab").is_err());
}

#[test]
fn parse_minute() {
    assert_eq!(Done(&[][..], 0), minute(b"00"));
    assert_eq!(Done(&[][..], 1), minute(b"01"));
    assert_eq!(Done(&[][..], 30), minute(b"30"));
    assert_eq!(Done(&[][..], 59), minute(b"59"));

    assert!(minute(b"60").is_err());
    assert!(minute(b"61").is_err());
    assert!(minute(b"ab").is_err());
}

#[test]
fn parse_second() {
    assert_eq!(Done(&[][..], 0), second(b"00"));
    assert_eq!(Done(&[][..], 1), second(b"01"));
    assert_eq!(Done(&[][..], 30), second(b"30"));
    assert_eq!(Done(&[][..], 59), second(b"59"));

    assert!(second(b"60").is_err());
    assert!(second(b"61").is_err());
    assert!(second(b"ab").is_err());
}

#[test]
fn parse_time() {
    assert_eq!(Done(&[][..], Time{ hour: 16, minute: 43, second: 16, tz_offset: 0}), time(b"16:43:16"));
    assert_eq!(Done(&[][..], Time{ hour: 16, minute: 43, second:  0, tz_offset: 0}), time(b"16:43"));

    assert!(time(b"20:").is_incomplete());
    assert!(time(b"20p42p16").is_err());
    assert!(time(b"pppp").is_err());
}

#[test]
fn parse_time_with_timezone() {
    assert_eq!(Done(&[][..], Time{ hour: 16, minute: 43, second: 16, tz_offset: 0}),
               time_with_timezone(b"16:43:16"));
    assert_eq!(Done(&[][..], Time{ hour: 16, minute: 43, second: 16, tz_offset: 0}),
               time_with_timezone(b"16:43:16Z"));
    assert_eq!(Done(&[][..], Time{ hour: 16, minute: 43, second: 16, tz_offset: 0}),
               time_with_timezone(b"16:43:16+00:00"));
    assert_eq!(Done(&[][..], Time{ hour: 16, minute: 43, second: 16, tz_offset: 5}),
               time_with_timezone(b"16:43:16+05:00"));

    assert!(time_with_timezone(b"20:").is_incomplete());
    assert!(time_with_timezone(b"20p42p16").is_err());
    assert!(time_with_timezone(b"pppp").is_err());

    assert!(time_with_timezone(b"16:43:16+").is_err());
    assert!(time_with_timezone(b"16:43:16+0").is_err());
    assert!(time_with_timezone(b"16:43:16+05:").is_err());
}

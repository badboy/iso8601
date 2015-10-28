#![allow(dead_code)]
#[macro_use]
extern crate nom;

use nom::IResult::*;
use nom::Err::*;

mod helper;
use helper::*;

#[macro_use] mod macros;
use macros::take_4_digits;

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

// year
named!(year_prefix, alt!(tag!("+") | tag!("-")));
named!(pub year <i32>, chain!(
        pref: opt!(year_prefix) ~
        year: call!(take_4_digits)
        ,
        || {
            match pref {
                Some(b"-") => -buf_to_i32(year),
                _ => buf_to_i32(year)
            }
        }));

// MM
named!(lower_month <u32>, chain!(tag!("0") ~ s:char_between!('1', '9') , || buf_to_u32(s)));
named!(upper_month <u32>, chain!(tag!("1") ~ s:char_between!('0', '2') , || 10+buf_to_u32(s)));
named!(pub month <u32>, alt!(lower_month | upper_month));


// DD
named!(day_zero  <u32>, chain!(tag!("0") ~ s:char_between!('1', '9') , || buf_to_u32(s)));
named!(day_one   <u32>, chain!(tag!("1") ~ s:char_between!('0', '9') , || 10+buf_to_u32(s)));
named!(day_two   <u32>, chain!(tag!("2") ~ s:char_between!('0', '9') , || 20+buf_to_u32(s)));
named!(day_three <u32>, chain!(tag!("3") ~ s:char_between!('0', '1') , || 30+buf_to_u32(s)));
named!(pub day <u32>, alt!(day_zero | day_one | day_two | day_three));

// WW
// reusing day_N parsers, sorry
named!(week_three <u32>, chain!(tag!("3") ~ s:char_between!('0', '9') , || 30+buf_to_u32(s)));
named!(week_four  <u32>, chain!(tag!("4") ~ s:char_between!('0', '9') , || 40+buf_to_u32(s)));
named!(week_five  <u32>, chain!(tag!("5") ~ s:char_between!('0', '3') , || 50+buf_to_u32(s)));

named!(week <u32>, alt!(day_zero | day_one | day_two | week_three| week_four | week_five ));
named!(week_day <u32>, chain!(s:char_between!('1', '7') , || buf_to_u32(s)));

// ordinal DDD
named!(ord_day <u32>, chain!(
        a:char_between!('0','3') ~
        b:char_between!('0','9') ~
        c:char_between!('0','9')
        ,
        || { buf_to_u32(a)*100 + buf_to_u32(b)*10 + buf_to_u32(c) }
        ));

// YYYY-MM-DD
named!(pub ymd_date <Date>, chain!(
        y: year ~
        opt!(tag!("-")) ~
        m: month ~
        opt!(tag!("-")) ~
        d: day
        ,
        || { Date::YMD{ year: y, month: m, day: d } }
        ));

// YYYY-MM-DD
named!(pub ordinal_date <Date>, chain!(
        y: year ~
        opt!(tag!("-")) ~
        d: ord_day
        ,
        || { Date::Ordinal{ year: y, ddd: d } }
        ));

// YYYY-"W"WW-D
named!(pub iso_week_date <Date>, chain!(
        y: year ~
        opt!(tag!("-")) ~
        tag!("W") ~
        w: week ~
        opt!(tag!("-")) ~
        d: week_day
        ,
        || { Date::Week{ year: y, ww: w, d: d } }
        ));

named!(pub date <Date>, alt!( ymd_date | iso_week_date | ordinal_date ) );

//    TIME

// HH
named!(lower_hour <u32>, chain!(f:char_between!('0','1') ~ s:char_between!('0','9') ,
                                       || { buf_to_u32(f)*10 + buf_to_u32(s) } ));
named!(upper_hour <u32>, chain!(tag!("2") ~ s:char_between!('0','4') , || 20+buf_to_u32(s)));
named!(pub hour <u32>, alt!(lower_hour | upper_hour));

// MM
named!(below_sixty <u32>, chain!(f:char_between!('0','5') ~ s:char_between!('0','9') ,
                                       || { buf_to_u32(f)*10 + buf_to_u32(s) } ));
named!(upto_sixty <u32>, alt!(below_sixty | map!(tag!("60"), |_| 60)));

named!(pub minute <u32>, call!(below_sixty));
named!(pub second <u32>, call!(upto_sixty));

// HH:MM:[SS]
named!(pub time <Time>, chain!(
        h: hour ~
        opt!(tag!(":")) ~
        m: minute ~
        s: empty_or!(
            chain!(
                tag!(":") ~
                s:second , || s) // TODO does this require the chain?
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

named!(sign <i32>, alt!(
        tag!("-") => { |_| -1 } |
        tag!("+") => { |_| 1 }
        )
    );

named!(timezone_hour <i32>, chain!(
        s: sign ~
        h: hour ~
        m: empty_or!(
            chain!(
                tag!(":")? ~ m: minute , || { m }
            ))
        ,
        || { (s * (h as i32) * 3600) + (m.unwrap_or(0) * 60) as i32 }
        ));

named!(tz_z, tag!("Z")); // TODO inline below
named!(timezone_utc <i32>, map!(tz_z, |_| 0));

named!(pub time_with_timezone <Time>, chain!(
        t: time ~
        s: opt!(
            alt!(
                timezone_hour | timezone_utc
                )
            )
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

// Full ISO8601
named!(pub datetime <DateTime>, chain!(
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

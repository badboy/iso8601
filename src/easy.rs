use nom;
use nom::IResult::*;

use helper::*;
use super::{Date,Time,DateTime};

use macros::{take_2_digits,take_4_digits};

named!(sign, alt!(tag!("+") | tag!("-")));
named!(numeric_sign <&[u8], i32>, map!(sign, |s: &[u8]| {
    match s {
        b"-" => -1,
        _ => 1,
    }
}));

named!(positive_year  <&[u8], i32>, map!(call!(take_4_digits), buf_to_i32));
named!(pub year <&[u8], i32>, chain!(
        pref: opt!(numeric_sign) ~
        y: positive_year
        ,
        || {
            pref.unwrap_or(1) * y
        }));

named!(pub month <&[u8], u32>, map!(call!(take_2_digits), buf_to_u32));
named!(pub day   <&[u8], u32>, map!(call!(take_2_digits), buf_to_u32));

named!(pub date <&[u8], Date>, chain!(
        y: year ~ tag!("-") ~ m: month ~ tag!("-") ~ d: day
        , || { Date{ year: y, month: m, day: d }
        }
        ));

named!(pub hour   <&[u8], u32>, map!(call!(take_2_digits), buf_to_u32));
named!(pub minute <&[u8], u32>, map!(call!(take_2_digits), buf_to_u32));
named!(pub second <&[u8], u32>, map!(call!(take_2_digits), buf_to_u32));

named!(pub time <&[u8], Time>, chain!(
        h: hour ~
        tag!(":") ~
        m: minute ~
        s: empty_or!(chain!(
                tag!(":") ~ s:second , || { s }))
        ,
        || { Time{ hour: h,
                   minute: m,
                   second: s.unwrap_or(0),
                   tz_offset: 0 }
           }
        ));

named!(timezone_hour <&[u8], i32>, chain!(
        s: numeric_sign ~
        h: hour ~
        m: empty_or!(
            chain!(
                tag!(":")? ~ m: minute , || { m }
            ))
        ,
        || { (s * (h as i32) * 3600) + (m.unwrap_or(0) * 60) as i32 }
        ));

named!(tag_z, tag!("Z"));
named!(timezone_utc <&[u8], i32>, map!(tag_z, |_| 0));
named!(timezone <&[u8], i32>, alt!(timezone_utc | timezone_hour));

named!(pub datetime <&[u8], DateTime>, chain!(
        d: date ~
        tag!("T") ~
        t: time ~
        tzo: empty_or!(call!(timezone))
        ,
        || {
            DateTime{
                date: d,
                time: t.set_tz(tzo.unwrap_or(0)),
            }
        }
        ));

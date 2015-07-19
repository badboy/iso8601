use nom;
use nom::IResult::*;

use helper::*;
use super::{Date,Time,DateTime};

use macros::{take_2_digits,take_4_digits};

named!(sign <i32>, alt!(
        tag!("-") => { |_| -1 } |
        tag!("+") => { |_| 1 }
        )
    );

named!(positive_year  <i32>, map!(call!(take_4_digits), buf_to_i32));
named!(pub year <i32>, chain!(
        pref: opt!(sign) ~
        y:    positive_year
        ,
        || {
            pref.unwrap_or(1) * y
        }));

named!(pub month <u32>, map!(call!(take_2_digits), buf_to_u32));
named!(pub day   <u32>, map!(call!(take_2_digits), buf_to_u32));

named!(pub date <Date>, chain!(
        y: year ~ tag!("-") ~ m: month ~ tag!("-") ~ d: day
        , || { Date{ year: y, month: m, day: d }
        }
        ));

named!(pub hour   <u32>, map!(call!(take_2_digits), buf_to_u32));
named!(pub minute <u32>, map!(call!(take_2_digits), buf_to_u32));
named!(pub second <u32>, map!(call!(take_2_digits), buf_to_u32));

named!(pub time <Time>, chain!(
        h: hour      ~
           tag!(":") ~
        m: minute    ~
        s: empty_or!(chain!(
                tag!(":") ~ s:second , || { s }))
        ,
        || { Time{ hour: h,
                   minute: m,
                   second: s.unwrap_or(0),
                   tz_offset: 0 }
           }
        ));

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

named!(timezone_utc <i32>, map!(tag!("Z"), |_| 0));
named!(timezone <i32>, alt!(timezone_utc | timezone_hour));

named!(pub datetime <DateTime>, chain!(
        d:   date      ~
             tag!("T") ~
        t:   time      ~
        tzo: empty_or!(call!(timezone))
        ,
        || {
            DateTime{
                date: d,
                time: t.set_tz(tzo.unwrap_or(0)),
            }
        }
        ));

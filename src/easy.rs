use nom::{self,Needed,is_digit};
use nom::IResult::*;
use nom::Err::*;

use helper::*;
use super::{Date,Time};

named!(take_4_digits, take_n_filter!(4, is_digit));
named!(take_2_digits, take_n_filter!(2, is_digit));

named!(year_prefix, alt!(tag!("+") | tag!("-")));
named!(positive_year  <&[u8], i32>, map!(call!(take_4_digits), buf_to_i32));
named!(pub year <&[u8], i32>, chain!(
        pref: opt!(year_prefix) ~
        y: positive_year
        ,
        || {
            match pref {
                Some(b"-") => -y,
                _ => y
            }
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

#[test]
fn easy_parse_date() {
    assert_eq!(Done(&[][..], Date{ year: 2015, month: 6, day: 26 }), date(b"2015-06-26"));
    assert_eq!(Done(&[][..], Date{ year: -333, month: 7, day: 11 }), date(b"-0333-07-11"));

    assert!(date(b"201").is_incomplete());
    assert!(date(b"2015p00p00").is_err());
    assert!(date(b"pppp").is_err());
}

#[test]
fn easy_parse_time() {
    assert_eq!(Done(&[][..], Time{ hour: 16, minute: 43, second: 16, tz_offset: 0}), time(b"16:43:16"));
    assert_eq!(Done(&[][..], Time{ hour: 16, minute: 43, second:  0, tz_offset: 0}), time(b"16:43"));

    assert!(time(b"20:").is_incomplete());
    assert!(time(b"20p42p16").is_err());
    assert!(time(b"pppp").is_err());
}

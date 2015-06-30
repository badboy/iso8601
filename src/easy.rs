use nom::{self,Needed,is_digit};
use nom::IResult::*;
use nom::Err::*;

use helper::*;
use super::{Date,Time,DateTime};

named!(take_4_digits, take_n_filter!(4, is_digit));
named!(take_2_digits, take_n_filter!(2, is_digit));

named!(sign, alt!(tag!("+") | tag!("-")));
named!(positive_year  <&[u8], i32>, map!(call!(take_4_digits), buf_to_i32));
named!(pub year <&[u8], i32>, chain!(
        pref: opt!(sign) ~
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

named!(timezone_hour <&[u8], i32>, chain!(
        s: sign ~
        h: hour ~
        empty_or!(
            chain!(
                tag!(":")? ~ m: minute , || { m }
            ))
        ,
        || {
            match s {
                b"-" => -(h as i32),
                _ => h as i32
            }
        }));

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

#[test]
fn easy_parse_datetime_correct() {
    fn make_datetime((year, month, day, hour, minute, second, tz_offset): (i32, u32, u32, u32, u32, u32, i32)) -> DateTime {
        DateTime {
            date: Date{ year: year, month: month, day: day },
            time: Time{ hour: hour, minute: minute, second: second, tz_offset: tz_offset },
        }
    }

    let test_datetimes = vec![
        ("2007-08-31T16:47+00:00",     (2007,  08,  31,  16,  47,  0,   0)),
        ("2007-12-24T18:21Z",          (2007,  12,  24,  18,  21,  0,   0)),
        ("2008-02-01T09:00:22+05",     (2008,  02,  01,  9,   0,   22,  5)),
        ("2009-01-01T12:00:00+01:00",  (2009,  1,   1,   12,  0,   0,   1)),
        ("2009-06-30T18:30:00+02:00",  (2009,  06,  30,  18,  30,  0,   2)),
        ("2015-06-29T23:07+02:00",     (2015,  06,  29,  23,  07,  0,   2)),
        ("2015-06-26T16:43:16",        (2015,  06,  26,  16,  43, 16,   0)),
    ];

    for (iso_string, data) in test_datetimes {
        assert_eq!(Done(&[][..], make_datetime(data)), datetime(iso_string.as_bytes()));
    }
}

#[test]
fn easy_parse_datetime_error() {
    let test_datetimes = vec![
        "ppp",
        "dumd-di-duTmd:iu:m"
    ];

    for iso_string in test_datetimes {
        let res = datetime(iso_string.as_bytes());
        assert!(res.is_err() || res.is_incomplete());
    }
}

//! This module is strictly internal.
//!
//! These functions are used by `date()`, `time()` and `datetime()`.
//! They are currently not private, because the need to be accessible,
//! but are not useful by themselves.
//!
//! Please refer to the top-level functions instead, as they offer a better abstraction.
//!
//! **These functions may be made private later.**

use helper::*;
use nom::{self, is_digit, digit};
use super::{Time, DateTime, Date};
use std::str::{self, FromStr};

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

macro_rules! check(
  ($input:expr, $submac:ident!( $($args:tt)* )) => (

    {
      let mut failed = false;
      for &idx in $input {
        if !$submac!(idx, $($args)*) {
            failed = true;
            break;
        }
      }
      if failed {
        nom::IResult::Error(nom::ErrorKind::Custom(20))
      } else {
        nom::IResult::Done(&b""[..], $input)
      }
    }
  );
  ($input:expr, $f:expr) => (
    check!($input, call!($f));
  );
);

macro_rules! char_between(
    ($input:expr, $min:expr, $max:expr) => (
        {
        fn f(c: u8) -> bool { c >= ($min as u8) && c <= ($max as u8)}
        flat_map!($input, take!(1), check!(f))
        }
    );
);

named!(take_4_digits, flat_map!(take!(4), check!(is_digit)));

// year
named!(year_prefix, alt!(tag!("+") | tag!("-")));
named!(year <i32>, do_parse!(
        pref: opt!(complete!(year_prefix)) >>
        year: call!(take_4_digits) >>
        (
            match pref {
                Some(b"-") => -buf_to_i32(year),
                _ => buf_to_i32(year)
            }
        )));

// MM
named!(lower_month <u32>, do_parse!(tag!("0") >> s:char_between!('1', '9') >> (buf_to_u32(s))));
named!(upper_month <u32>, do_parse!(tag!("1") >> s:char_between!('0', '2') >> (10+buf_to_u32(s))));
named!(month <u32>, alt!(lower_month | upper_month));


// DD
named!(day_zero  <u32>, do_parse!(tag!("0") >> s:char_between!('1', '9') >> (buf_to_u32(s))));
named!(day_one   <u32>, do_parse!(tag!("1") >> s:char_between!('0', '9') >> (10+buf_to_u32(s))));
named!(day_two   <u32>, do_parse!(tag!("2") >> s:char_between!('0', '9') >> (20+buf_to_u32(s))));
named!(day_three <u32>, do_parse!(tag!("3") >> s:char_between!('0', '1') >> (30+buf_to_u32(s))));
named!(day <u32>, alt!(day_zero | day_one | day_two | day_three));

// WW
// reusing day_N parsers, sorry
named!(week_three <u32>, do_parse!(tag!("3") >> s:char_between!('0', '9') >> (30+buf_to_u32(s))));
named!(week_four  <u32>, do_parse!(tag!("4") >> s:char_between!('0', '9') >> (40+buf_to_u32(s))));
named!(week_five  <u32>, do_parse!(tag!("5") >> s:char_between!('0', '3') >> (50+buf_to_u32(s))));

named!(week <u32>, alt!(day_zero | day_one | day_two | week_three| week_four | week_five ));
named!(week_day <u32>, map!(char_between!('1', '7') , |s| buf_to_u32(s)));

// ordinal DDD
named!(ord_day <u32>, do_parse!(
        a:char_between!('0','3') >>
        b:char_between!('0','9') >>
        c:char_between!('0','9') >>
        ( buf_to_u32(a)*100 + buf_to_u32(b)*10 + buf_to_u32(c) )
        ));

// YYYY-MM-DD
named!(pub ymd_date <Date>, do_parse!(
        y: year >>
        opt!(complete!(tag!("-"))) >>
        m: month >>
        opt!(complete!(tag!("-"))) >>
        d: day >>
        ( Date::YMD{ year: y, month: m, day: d } )
        ));

// YYYY-MM-DD
named!(pub ordinal_date <Date>, do_parse!(
        y: year >>
        opt!(complete!(tag!("-"))) >>
        d: ord_day >>
        ( Date::Ordinal{ year: y, ddd: d } )
        ));

// YYYY-"W"WW-D
named!(pub iso_week_date <Date>, do_parse!(
        y: year >>
        opt!(complete!(tag!("-"))) >>
        tag!("W") >>
        w: week >>
        opt!(complete!(tag!("-"))) >>
        d: week_day >>
        ( Date::Week{ year: y, ww: w, d: d } )
        ));

named!(pub parse_date <Date>, alt!( ymd_date | iso_week_date | ordinal_date ) );

// TIME

// HH
named!(lower_hour <u32>, do_parse!(f:char_between!('0','1') >>
                                   s:char_between!('0','9') >>
                                   ( buf_to_u32(f)*10 + buf_to_u32(s) )));
named!(upper_hour <u32>, do_parse!(tag!("2") >>
                                   s:char_between!('0','4') >>
                                   (20+buf_to_u32(s))));
named!(hour <u32>, alt!(lower_hour | upper_hour));

// MM
named!(below_sixty <u32>, do_parse!(f:char_between!('0','5') >>
                                    s:char_between!('0','9') >>
                                    ( buf_to_u32(f)*10 + buf_to_u32(s) ) ));
named!(upto_sixty <u32>, alt!(below_sixty | map!(tag!("60"), |_| 60)));

fn into_fraction_string(digits: &[u8]) -> Result<f32, ::std::num::ParseFloatError> {
    let mut s = String::from("0.");
    s += str::from_utf8(digits).unwrap();
    FromStr::from_str(&s)
}


named!(minute <u32>, call!(below_sixty));
named!(second <u32>, call!(upto_sixty));
named!(fractions <f32>, map_res!(digit, into_fraction_string));

fn millisecond(fraction: f32) -> u32 {
    (1000.0 * fraction) as u32
}

// HH:MM:[SS][.(m*)][(Z|+...|-...)]
named!(pub parse_time <Time>, do_parse!(
        h: hour >>
        opt!(complete!(tag!(":"))) >>
        m: minute >>
        s:  opt!(complete!( preceded!(opt!(tag!(":")), second))) >>
        ms: opt!(complete!( map!(preceded!(one_of!(",."), fractions), millisecond))) >>
        z:  opt!(complete!( alt!( timezone_hour | timezone_utc) )) >>
        (
            Time {
                hour: h,
                minute: m,
                second: s.unwrap_or(0),
                millisecond: ms.unwrap_or(0),
                tz_offset_hours: z.unwrap_or((0,0)).0,
                tz_offset_minutes: z.unwrap_or((0,0)).1
            }
        )
        ));

named!(sign <i32>, alt!(
        tag!("-") => { |_| -1 } |
        tag!("+") => { |_| 1 }
        )
    );

named!(timezone_hour <(i32,i32)>, do_parse!(
        s: sign >>
        h: hour >>
        m: empty_or!(
            preceded!(opt!(tag!(":")), minute)
           ) >>
        ( (s * (h as i32) , s * (m.unwrap_or(0) as i32)) )
        ));

named!(timezone_utc <(i32,i32)>, map!(tag!("Z"), |_| (0,0)));

// Full ISO8601
named!(pub parse_datetime <DateTime>, do_parse!(
        d: parse_date >>
        tag!("T") >>
        t: parse_time >>
        (
            DateTime{
                date: d,
                time: t,
            }
        )
        ));

#[cfg(test)]
mod tests{

    use super::{year, month, day};
    use super::{hour, minute, second};
    use nom::IResult::*;

    #[test]
    fn test_year() {
        assert_eq!(Done(&[][..],   2015), year(b"2015"));
        assert_eq!(Done(&[][..],  -0333), year(b"-0333"));
        assert_eq!(Done(&b"-"[..], 2015), year(b"2015-"));
        assert!(year(b"abcd").is_err());
        assert!(year(b"2a03").is_err());
    }

    #[test]
    fn test_month() {
        assert_eq!(Done(&[][..], 1),    month(b"01"));
        assert_eq!(Done(&[][..], 6),    month(b"06"));
        assert_eq!(Done(&[][..], 12),   month(b"12"));
        assert_eq!(Done(&b"-"[..], 12), month(b"12-"));

        assert!(month(b"13").is_err());
        assert!(month(b"00").is_err());
    }

    #[test]
    fn test_day() {
        assert_eq!(Done(&[][..], 1),    day(b"01"));
        assert_eq!(Done(&[][..], 12),   day(b"12"));
        assert_eq!(Done(&[][..], 20),   day(b"20"));
        assert_eq!(Done(&[][..], 28),   day(b"28"));
        assert_eq!(Done(&[][..], 30),   day(b"30"));
        assert_eq!(Done(&[][..], 31),   day(b"31"));
        assert_eq!(Done(&b"-"[..], 31), day(b"31-"));

        assert!(day(b"00").is_err());
        assert!(day(b"32").is_err());
    }

    #[test]
    fn test_hour() {
        assert_eq!(Done(&[][..], 0),  hour(b"00"));
        assert_eq!(Done(&[][..], 1),  hour(b"01"));
        assert_eq!(Done(&[][..], 6),  hour(b"06"));
        assert_eq!(Done(&[][..], 12), hour(b"12"));
        assert_eq!(Done(&[][..], 13), hour(b"13"));
        assert_eq!(Done(&[][..], 20), hour(b"20"));
        assert_eq!(Done(&[][..], 24), hour(b"24"));

        assert!(hour(b"25").is_err());
        assert!(hour(b"30").is_err());
        assert!(hour(b"ab").is_err());
    }

    #[test]
    fn test_minute() {
        assert_eq!(Done(&[][..], 0),  minute(b"00"));
        assert_eq!(Done(&[][..], 1),  minute(b"01"));
        assert_eq!(Done(&[][..], 30), minute(b"30"));
        assert_eq!(Done(&[][..], 59), minute(b"59"));

        assert!(minute(b"60").is_err());
        assert!(minute(b"61").is_err());
        assert!(minute(b"ab").is_err());
    }

    #[test]
    fn test_second() {
        assert_eq!(Done(&[][..], 0),  second(b"00"));
        assert_eq!(Done(&[][..], 1),  second(b"01"));
        assert_eq!(Done(&[][..], 30), second(b"30"));
        assert_eq!(Done(&[][..], 59), second(b"59"));
        assert_eq!(Done(&[][..], 60), second(b"60"));

        assert!(second(b"61").is_err());
        assert!(second(b"ab").is_err());
    }

}

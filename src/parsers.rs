//! This module is strictly internal.
//!
//! These functions are used by `date()`, `time()` and `datetime()`.
//! They are currently not private, because the need to be accessible,
//! but are not useful by themselves.
//!
//! Please refer to the top-level functions instead, as they offer a better abstraction.
//!
//! **These functions may be made private later.**

use super::{Date, DateTime, Time};
use helper::*;
use nom::types::CompleteByteSlice;
use nom::{self, digit, is_digit};
use std::str::{self, FromStr};

#[cfg(test)]
mod tests;

macro_rules! empty_or(
    ($i:expr, $submac:ident!( $($args:tt)* )) => ({
        use nom::InputLength;
        if $i.input_len() == 0 {
            Ok(($i, None))
        } else {
            match $submac!($i, $($args)*) {
                Ok((i,o))     => Ok((i, Some(o))),
                Err(nom::Err::Error(_))      => Ok(($i, None)),
                Err(nom::Err::Failure(_))    => Ok(($i, None)),
                Err(nom::Err::Incomplete(i)) => Err(nom::Err::Incomplete(i))

            }
        }
    });
);

macro_rules! check(
  ($input:expr, $submac:ident!( $($args:tt)* )) => (

    {
      let mut failed = false;
      for &idx in $input.0 {
        if !$submac!(idx, $($args)*) {
            failed = true;
            break;
        }
      }
      if failed {
        Err(nom::Err::Error(error_position!($input, nom::ErrorKind::Custom(20u32))))
      } else {
        Ok((CompleteByteSlice(&b""[..]), $input))
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

named!(take_4_digits<CompleteByteSlice,CompleteByteSlice>, flat_map!(take!(4), check!(is_digit)));

// year
named!(year_prefix<CompleteByteSlice, CompleteByteSlice>, alt!(tag!("+") | tag!("-")));
named!(year <CompleteByteSlice, i32>, do_parse!(
        pref: opt!(year_prefix) >>
        year: call!(take_4_digits) >>
        (
            match pref {
                Some(CompleteByteSlice(b"-")) => -buf_to_i32(year.0),
                _ => buf_to_i32(year.0)
            }
        )));

// MM
named!(lower_month <CompleteByteSlice,u32>, do_parse!(tag!("0") >> s:char_between!('1', '9') >> (buf_to_u32(s.0))));
named!(upper_month <CompleteByteSlice,u32>, do_parse!(tag!("1") >> s:char_between!('0', '2') >> (10+buf_to_u32(s.0))));
named!(month <CompleteByteSlice,u32>, alt!(lower_month | upper_month));

// DD
named!(day_zero  <CompleteByteSlice,u32>, do_parse!(tag!("0") >> s:char_between!('1', '9') >> (buf_to_u32(s.0))));
named!(day_one   <CompleteByteSlice,u32>, do_parse!(tag!("1") >> s:char_between!('0', '9') >> (10+buf_to_u32(s.0))));
named!(day_two   <CompleteByteSlice,u32>, do_parse!(tag!("2") >> s:char_between!('0', '9') >> (20+buf_to_u32(s.0))));
named!(day_three <CompleteByteSlice,u32>, do_parse!(tag!("3") >> s:char_between!('0', '1') >> (30+buf_to_u32(s.0))));
named!(day <CompleteByteSlice,u32>, alt!(day_zero | day_one | day_two | day_three));

// WW
// reusing day_N parsers, sorry
named!(week_three <CompleteByteSlice,u32>, do_parse!(tag!("3") >> s:char_between!('0', '9') >> (30+buf_to_u32(s.0))));
named!(week_four  <CompleteByteSlice,u32>, do_parse!(tag!("4") >> s:char_between!('0', '9') >> (40+buf_to_u32(s.0))));
named!(week_five  <CompleteByteSlice,u32>, do_parse!(tag!("5") >> s:char_between!('0', '3') >> (50+buf_to_u32(s.0))));

named!(week <CompleteByteSlice,u32>, alt!(day_zero | day_one | day_two | week_three| week_four | week_five ));
named!(week_day <CompleteByteSlice,u32>, map!(char_between!('1', '7') , |s| buf_to_u32(s.0)));

// ordinal DDD
named!(ord_day <CompleteByteSlice,u32>, do_parse!(
        a:char_between!('0','3') >>
        b:char_between!('0','9') >>
        c:char_between!('0','9') >>
        ( buf_to_u32(a.0)*100 + buf_to_u32(b.0)*10 + buf_to_u32(c.0) )
        ));

// YYYY-MM-DD
named!(pub ymd_date <CompleteByteSlice,Date>, do_parse!(
        y: year >>
        opt!(tag!("-")) >>
        m: month >>
        opt!(tag!("-")) >>
        d: day >>
        ( Date::YMD{ year: y, month: m, day: d } )
        ));

// YYYY-MM-DD
named!(pub ordinal_date <CompleteByteSlice,Date>, do_parse!(
        y: year >>
        opt!(tag!("-")) >>
        d: ord_day >>
        ( Date::Ordinal{ year: y, ddd: d } )
        ));

// YYYY-"W"WW-D
named!(pub iso_week_date <CompleteByteSlice,Date>, do_parse!(
        y: year >>
        opt!(tag!("-")) >>
        tag!("W") >>
        w: week >>
        opt!(tag!("-")) >>
        d: week_day >>
        ( Date::Week{ year: y, ww: w, d: d } )
        ));

named!(pub parse_date <CompleteByteSlice,Date>, alt!( ymd_date | iso_week_date | ordinal_date ) );

// TIME

// HH
named!(lower_hour <CompleteByteSlice,u32>, do_parse!(f:char_between!('0','1') >>
                                   s:char_between!('0','9') >>
                                   ( buf_to_u32(f.0)*10 + buf_to_u32(s.0) )));
named!(upper_hour <CompleteByteSlice,u32>, do_parse!(tag!("2") >>
                                   s:char_between!('0','4') >>
                                   (20+buf_to_u32(s.0))));
named!(hour <CompleteByteSlice,u32>, alt!(lower_hour | upper_hour));

// MM
named!(below_sixty <CompleteByteSlice,u32>, do_parse!(f:char_between!('0','5') >>
                                    s:char_between!('0','9') >>
                                    ( buf_to_u32(f.0)*10 + buf_to_u32(s.0) ) ));
named!(upto_sixty <CompleteByteSlice,u32>, alt!(below_sixty | map!(tag!("60"), |_| 60)));

fn into_fraction_string(digits: CompleteByteSlice) -> Result<f32, ::std::num::ParseFloatError> {
    let mut s = String::from("0.");
    s += str::from_utf8(digits.0).unwrap();
    FromStr::from_str(&s)
}

named!(minute <CompleteByteSlice,u32>, call!(below_sixty));
named!(second <CompleteByteSlice,u32>, call!(upto_sixty));
named!(fractions <CompleteByteSlice,f32>, map_res!(digit, into_fraction_string));

fn millisecond(fraction: f32) -> u32 {
    (1000.0 * fraction) as u32
}

// HH:MM:[SS][.(m*)][(Z|+...|-...)]
named!(pub parse_time <CompleteByteSlice, Time>, do_parse!(
        h: hour >>
        opt!(tag!(":")) >>
        m: minute >>
        s:  opt!(preceded!(opt!(tag!(":")), second)) >>
        ms: opt!( map!(preceded!(one_of!(",."), fractions), millisecond)) >>
        z:  opt!( alt!( timezone_hour | timezone_utc) ) >>
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

named!(sign <CompleteByteSlice,i32>, alt!(
        tag!("-") => { |_| -1 } |
        tag!("+") => { |_| 1 }
        )
    );

named!(timezone_hour <CompleteByteSlice,(i32,i32)>, do_parse!(
        s: sign >>
        h: hour >>
        m: empty_or!(
            preceded!(opt!(tag!(":")), minute)
           ) >>
        ( (s * (h as i32) , s * (m.unwrap_or(0) as i32)) )
        ));

named!(timezone_utc <CompleteByteSlice,(i32,i32)>, map!(tag!("Z"), |_| (0,0)));

// Full ISO8601
named!(pub parse_datetime <CompleteByteSlice,DateTime>, do_parse!(
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

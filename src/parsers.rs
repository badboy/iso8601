//! This module is strictly internal.
//!
//! These functions are used by `date()`, `time()` and `datetime()`.
//! They are currently not private, because the need to be accessible,
//! but are not useful by themselves.
//!
//! Please refer to the top-level functions instead, as they offer a better abstraction.
//!
//! **These functions may be made private later.**

use std::str::{self, FromStr};

use nom::{
    IResult,
    bytes::complete::{tag, take_while, take_while_m_n},
    character::is_digit,
    branch::alt,
    combinator::{opt, map},
    sequence::preceded,
    character::complete::one_of,
};

use crate::{Date, DateTime, Time};
use crate::helper::*;

#[cfg(test)]
mod tests;

fn take_n_digits(i: &[u8], n: usize) -> IResult<&[u8], u32> {
    let (i, digits) = take_while_m_n(n, n, is_digit)(i)?;
    Ok((i, buf_to_u32(digits)))
}

// year
fn year_prefix(i: &[u8]) -> IResult<&[u8], &[u8]> {
    alt((tag(b"+"), tag(b"-")))(i)
}
fn year(i: &[u8]) -> IResult<&[u8], i32> {
    let (i, is_pos) = match opt(year_prefix)(i) {
        Ok((i, Some(b))) if b == b"-" => (i, false),
        Ok((i, _)) => (i, true),
        Err(e) => return Err(e),
    };
    let (i, year) = take_n_digits(i, 4)?;
    let mut year = year as i32;
    if !is_pos { year = -year }

    Ok((i, year))
}

// MM
fn month(i: &[u8]) -> IResult<&[u8], u32> {
    let (new_i, m) = take_n_digits(i, 2)?;

    if m >= 1 && m <= 12 {
        Ok((new_i, m))
    } else {
        Err(nom::Err::Error((i, nom::error::ErrorKind::Eof)))
    }
}

// DD
fn day(i: &[u8]) -> IResult<&[u8], u32> {
    let (new_i, d) = take_n_digits(i, 2)?;

    if d >= 1 && d <= 31 {
        Ok((new_i, d))
    } else {
        Err(nom::Err::Error((i, nom::error::ErrorKind::Eof)))
    }
}

// WW
fn week(i: &[u8]) -> IResult<&[u8], u32> {
    let (new_i, w) = take_n_digits(i, 2)?;

    if w >= 1 && w <= 52 {
        Ok((new_i, w))
    } else {
        Err(nom::Err::Error((i, nom::error::ErrorKind::Eof)))
    }
}

fn week_day(i: &[u8]) -> IResult<&[u8], u32> {
    let (new_i, d) = take_n_digits(i, 1)?;

    if d >= 1 && d <= 7 {
        Ok((new_i, d))
    } else {
        Err(nom::Err::Error((i, nom::error::ErrorKind::Eof)))
    }
}

// ordinal DDD
fn ord_day(i: &[u8]) -> IResult<&[u8], u32> {
    let (new_i, d) = take_n_digits(i, 3)?;

    if d >= 1 && d <= 366 {
        Ok((new_i, d))
    } else {
        Err(nom::Err::Error((i, nom::error::ErrorKind::Eof)))
    }
}

// YYYY-MM-DD
fn ymd_date(i: &[u8]) -> IResult<&[u8], Date> {
    let (i, y) = year(i)?;
    let (i, _) = opt(tag(b"-"))(i)?;
    let (i, m) = month(i)?;
    let (i, _) = opt(tag(b"-"))(i)?;
    let (i, d) = day(i)?;
    Ok((i, Date::YMD { year: y, month: m, day: d }))
}

// YYYY-DDD
fn ordinal_date(i: &[u8]) -> IResult<&[u8], Date> {
    let (i, y) = year(i)?;
    let (i, _) = opt(tag(b"-"))(i)?;
    let (i, d) = ord_day(i)?;
    Ok((i, Date::Ordinal { year: y, ddd: d }))
}

// YYYY-"W"WW-D
fn iso_week_date(i: &[u8]) -> IResult<&[u8], Date> {
    let (i, y) = year(i)?;
    let (i, _) = opt(tag(b"-"))(i)?;
    let (i, _) = tag(b"W")(i)?;
    let (i, w) = week(i)?;
    let (i, _) = opt(tag(b"-"))(i)?;
    let (i, d) = week_day(i)?;
    Ok((i, Date::Week { year: y, ww: w, d: d }))
}

pub fn parse_date(i: &[u8]) -> IResult<&[u8], Date> {
    alt((
            ymd_date,
            iso_week_date,
            ordinal_date
    ))(i)
}

// TIME

// HH
fn hour(i: &[u8]) -> IResult<&[u8], u32> {
    let (i, m) = take_n_digits(i, 2)?;

    if m <= 24 {
        Ok((i, m))
    } else {
        Err(nom::Err::Error((i, nom::error::ErrorKind::Eof)))
    }
}

// MM
fn minute(i: &[u8]) -> IResult<&[u8], u32> {
    let (i, m) = take_n_digits(i, 2)?;

    if m <= 59 {
        Ok((i, m))
    } else {
        Err(nom::Err::Error((i, nom::error::ErrorKind::Eof)))
    }
}

fn second(i: &[u8]) -> IResult<&[u8], u32> {
    let (i, m) = take_n_digits(i, 2)?;

    if m <= 60 {
        Ok((i, m))
    } else {
        Err(nom::Err::Error((i, nom::error::ErrorKind::Eof)))
    }
}

fn into_fraction_string(digits: &[u8]) -> Result<f32, ::std::num::ParseFloatError> {
    let mut s = String::from("0.");
    s += str::from_utf8(digits).unwrap();
    FromStr::from_str(&s)
}
fn fractions(i: &[u8]) -> IResult<&[u8], f32> {
    let (i, f) = take_while(is_digit)(i)?;
    let f = into_fraction_string(f).unwrap();

    Ok((i, f))
}

fn millisecond(fraction: f32) -> u32 {
    (1000.0 * fraction) as u32
}

// HH:MM:[SS][.(m*)][(Z|+...|-...)]
pub fn parse_time(i: &[u8]) -> IResult<&[u8], Time> {
    let (i, h) = hour(i)?;
    let (i, _) = opt(tag(b":"))(i)?;
    let (i, m) = minute(i)?;
    let (i, s) = opt(preceded(opt(tag(b":")), second))(i)?;
    let (i, ms) = opt(map(preceded(one_of(",."), fractions), millisecond))(i)?;
    let (i, z) = match opt(alt((timezone_hour, timezone_utc)))(i) {
        Ok(ok) => ok,
        Err(nom::Err::Incomplete(_)) => (i, None),
        Err(e) => return Err(e),
    };

    Ok((i, Time {
        hour: h,
        minute: m,
        second: s.unwrap_or(0),
        millisecond: ms.unwrap_or(0),
        tz_offset_hours: z.unwrap_or((0, 0)).0,
        tz_offset_minutes: z.unwrap_or((0, 0)).1,
    }))
}

fn sign(i: &[u8]) -> IResult<&[u8], i32> {
    map(
        alt((tag(b"-"), tag(b"+"))),
        |s: &[u8]| {
            match s {
                b"-" => -1,
                _ => 1
            }
        })(i)
}

fn timezone_hour(i: &[u8]) -> IResult<&[u8], (i32, i32)> {
    let (i, s) = sign(i)?;
    let (i, h) = hour(i)?;
    let (i, m) = if i.is_empty() {
        (i, 0)
    } else {
        let (i, _) = opt(tag(b":"))(i)?;
        minute(i)?
    };

    Ok((i,
        ( (s * (h as i32) ,
           s * (m as i32))
        )
    ))
}

fn timezone_utc(i: &[u8]) -> IResult<&[u8], (i32, i32)> {
    map(tag(b"Z"), |_| (0, 0))(i)
}

// Full ISO8601
pub fn parse_datetime(i: &[u8]) -> IResult<&[u8], DateTime> {
    let (i, d) = parse_date(i)?;
    let (i, _) = tag(b"T")(i)?;
    let (i, t) = parse_time(i)?;

    Ok((i, DateTime { date: d, time: t }))
}

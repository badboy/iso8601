//! This module is strictly internal.
//!
//! These functions are used by `date()`, `time()` and `datetime()`.
//! They are currently not private, because the need to be accessible,
//! but are not useful by themselves.
//!
//! Please refer to the top-level functions instead, as they offer a better abstraction.
//!
//! **These functions may be made private later.**

use std::str;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, take_while_m_n},
    character::complete::one_of,
    character::is_digit,
    combinator::{map, opt},
    sequence::preceded,
    IResult,
};

use crate::{Date, DateTime, Time};

#[cfg(test)]
mod tests;

fn take_n_digits(i: &[u8], n: usize) -> IResult<&[u8], u32> {
    let (i, digits) = take_while_m_n(n, n, is_digit)(i)?;

    let s = str::from_utf8(digits).expect("Invalid data, expected UTF-8 string");
    let res = s
        .parse()
        .expect("Invalid string, expected ASCII reprensation of a number");
    Ok((i, res))
}

fn sign(i: &[u8]) -> IResult<&[u8], i32> {
    map(alt((tag(b"-"), tag(b"+"))), |s: &[u8]| match s {
        b"-" => -1,
        _ => 1,
    })(i)
}

// [+/-] year
fn year(i: &[u8]) -> IResult<&[u8], i32> {
    // The sign is optional, but defaults to `+`
    let (i, s) = sign(i).unwrap_or((i, 1));
    let (i, year) = take_n_digits(i, 4)?;
    let year = s * year as i32;

    Ok((i, year))
}

fn n_digit_in_range(
    i: &[u8],
    n: usize,
    range: impl std::ops::RangeBounds<u32>,
) -> IResult<&[u8], u32> {
    let (new_i, number) = take_n_digits(i, n)?;

    if range.contains(&number) {
        Ok((new_i, number))
    } else {
        Err(nom::Err::Error((i, nom::error::ErrorKind::Eof)))
    }
}

// MM
fn month(i: &[u8]) -> IResult<&[u8], u32> {
    n_digit_in_range(i, 2, 1..=12)
}

// DD
fn day(i: &[u8]) -> IResult<&[u8], u32> {
    n_digit_in_range(i, 2, 1..=31)
}

// WW
fn week(i: &[u8]) -> IResult<&[u8], u32> {
    n_digit_in_range(i, 2, 1..=52)
}

fn week_day(i: &[u8]) -> IResult<&[u8], u32> {
    n_digit_in_range(i, 1, 1..=7)
}

// ordinal DDD
fn ord_day(i: &[u8]) -> IResult<&[u8], u32> {
    n_digit_in_range(i, 3, 1..=366)
}

// YYYY-MM-DD
fn ymd_date(i: &[u8]) -> IResult<&[u8], Date> {
    let (i, y) = year(i)?;
    let (i, _) = opt(tag(b"-"))(i)?;
    let (i, m) = month(i)?;
    let (i, _) = opt(tag(b"-"))(i)?;
    let (i, d) = day(i)?;
    Ok((
        i,
        Date::YMD {
            year: y,
            month: m,
            day: d,
        },
    ))
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
    Ok((
        i,
        Date::Week {
            year: y,
            ww: w,
            d: d,
        },
    ))
}

pub fn parse_date(i: &[u8]) -> IResult<&[u8], Date> {
    alt((ymd_date, iso_week_date, ordinal_date))(i)
}

// TIME

// HH
fn hour(i: &[u8]) -> IResult<&[u8], u32> {
    n_digit_in_range(i, 2, 0..=24)
}

// MM
fn minute(i: &[u8]) -> IResult<&[u8], u32> {
    n_digit_in_range(i, 2, 0..=59)
}

fn second(i: &[u8]) -> IResult<&[u8], u32> {
    n_digit_in_range(i, 2, 0..=60)
}

fn fractions(i: &[u8]) -> IResult<&[u8], f32> {
    let (i, digits) = take_while(is_digit)(i)?;
    let digits = str::from_utf8(digits).unwrap(); // This can't panic, `digits` will only include digits.
    let f = format!("0.{}", digits).parse().unwrap(); // This can't panic, the string is a valid `f32`.

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

    Ok((
        i,
        Time {
            hour: h,
            minute: m,
            second: s.unwrap_or(0),
            millisecond: ms.unwrap_or(0),
            tz_offset_hours: z.unwrap_or((0, 0)).0,
            tz_offset_minutes: z.unwrap_or((0, 0)).1,
        },
    ))
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

    Ok((i, ((s * (h as i32), s * (m as i32)))))
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

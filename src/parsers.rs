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
    combinator::{map, not, opt},
    sequence::{preceded, terminated},
    IResult,
};

use crate::{Date, DateTime, Duration, Time};

#[cfg(test)]
mod tests;

// UTILITY

fn take_n_digits(i: &[u8], n: usize) -> IResult<&[u8], u32> {
    let (i, digits) = take_while_m_n(n, n, is_digit)(i)?;

    let s = str::from_utf8(digits).expect("Invalid data, expected UTF-8 string");
    let res = s
        .parse()
        .expect("Invalid string, expected ASCII reprensentation of a number");

    Ok((i, res))
}

fn take_m_to_n_digits(i: &[u8], m: usize, n: usize) -> IResult<&[u8], u32> {
    let (i, digits) = take_while_m_n(m, n, is_digit)(i)?;

    let s = str::from_utf8(digits).expect("Invalid data, expected UTF-8 string");
    let res = s
        .parse()
        .expect("Invalid string, expected ASCII representation of a number");

    Ok((i, res))
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

fn m_to_n_digit_in_range(
    i: &[u8],
    m: usize,
    n: usize,
    range: impl std::ops::RangeBounds<u32>,
) -> IResult<&[u8], u32> {
    let (new_i, number) = take_m_to_n_digits(i, m, n)?;

    if range.contains(&number) {
        Ok((new_i, number))
    } else {
        Err(nom::Err::Error((i, nom::error::ErrorKind::Eof)))
    }
}

fn sign(i: &[u8]) -> IResult<&[u8], i32> {
    map(alt((tag(b"-"), tag(b"+"))), |s: &[u8]| match s {
        b"-" => -1,
        _ => 1,
    })(i)
}

fn fractions(i: &[u8]) -> IResult<&[u8], f32> {
    let (i, digits) = take_while(is_digit)(i)?;
    let digits = str::from_utf8(digits).unwrap(); // This can't panic, `digits` will only include digits.
    let f = format!("0.{}", digits).parse().unwrap(); // This can't panic, the string is a valid `f32`.

    Ok((i, f))
}

// DATE

// [+/-]YYYY
fn date_year(i: &[u8]) -> IResult<&[u8], i32> {
    // The sign is optional, but defaults to `+`
    let (i, s) = sign(i).unwrap_or((i, 1));
    let (i, year) = take_n_digits(i, 4)?;
    let year = s * year as i32;

    Ok((i, year))
}

// MM
fn date_month(i: &[u8]) -> IResult<&[u8], u32> {
    n_digit_in_range(i, 2, 1..=12)
}

// DD
fn date_day(i: &[u8]) -> IResult<&[u8], u32> {
    n_digit_in_range(i, 2, 1..=31)
}

// WW
fn date_week(i: &[u8]) -> IResult<&[u8], u32> {
    n_digit_in_range(i, 2, 1..=52)
}

fn date_week_day(i: &[u8]) -> IResult<&[u8], u32> {
    n_digit_in_range(i, 1, 1..=7)
}

// ordinal DDD
fn date_ord_day(i: &[u8]) -> IResult<&[u8], u32> {
    n_digit_in_range(i, 3, 1..=366)
}

// YYYY-MM-DD
fn date_ymd(i: &[u8]) -> IResult<&[u8], Date> {
    let (i, y) = date_year(i)?;
    let (i, _) = opt(tag(b"-"))(i)?;
    let (i, m) = date_month(i)?;
    let (i, _) = opt(tag(b"-"))(i)?;
    let (i, d) = date_day(i)?;

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
fn date_ordinal(i: &[u8]) -> IResult<&[u8], Date> {
    let (i, y) = date_year(i)?;
    let (i, _) = opt(tag(b"-"))(i)?;
    let (i, d) = date_ord_day(i)?;

    Ok((i, Date::Ordinal { year: y, ddd: d }))
}

// YYYY-"W"WW-D
fn date_iso_week(i: &[u8]) -> IResult<&[u8], Date> {
    let (i, y) = date_year(i)?;
    let (i, _) = opt(tag(b"-"))(i)?;
    let (i, _) = tag(b"W")(i)?;
    let (i, w) = date_week(i)?;
    let (i, _) = opt(tag(b"-"))(i)?;
    let (i, d) = date_week_day(i)?;

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
    alt((date_ymd, date_iso_week, date_ordinal))(i)
}

// TIME

// HH
fn time_hour(i: &[u8]) -> IResult<&[u8], u32> {
    n_digit_in_range(i, 2, 0..=24)
}

// MM
fn time_minute(i: &[u8]) -> IResult<&[u8], u32> {
    n_digit_in_range(i, 2, 0..=59)
}

// SS
fn time_second(i: &[u8]) -> IResult<&[u8], u32> {
    n_digit_in_range(i, 2, 0..=60)
}

fn time_millisecond(fraction: f32) -> u32 {
    (1000.0 * fraction) as u32
}

// HH:MM:[SS][.(m*)][(Z|+...|-...)]
pub fn parse_time(i: &[u8]) -> IResult<&[u8], Time> {
    let (i, h) = time_hour(i)?;
    let (i, _) = opt(tag(b":"))(i)?;
    let (i, m) = time_minute(i)?;
    let (i, s) = opt(preceded(opt(tag(b":")), time_second))(i)?;
    let (i, ms) = opt(map(preceded(one_of(",."), fractions), time_millisecond))(i)?;
    let (i, z) = match opt(alt((timezone_hour, timezone_utc)))(i) {
        Ok(ok) => ok,
        Err(nom::Err::Incomplete(_)) => (i, None),
        Err(e) => return Err(e),
    };

    let (tz_offset_hours, tz_offset_minutes) = z.unwrap_or((0, 0));

    Ok((
        i,
        Time {
            hour: h,
            minute: m,
            second: s.unwrap_or(0),
            millisecond: ms.unwrap_or(0),
            tz_offset_hours,
            tz_offset_minutes,
        },
    ))
}

fn timezone_hour(i: &[u8]) -> IResult<&[u8], (i32, i32)> {
    let (i, s) = sign(i)?;
    let (i, h) = time_hour(i)?;
    let (i, m) = if i.is_empty() {
        (i, 0)
    } else {
        let (i, _) = opt(tag(b":"))(i)?;
        time_minute(i)?
    };

    Ok((i, ((s * (h as i32), s * (m as i32)))))
}

fn timezone_utc(i: &[u8]) -> IResult<&[u8], (i32, i32)> {
    map(tag(b"Z"), |_| (0, 0))(i)
}

// Full ISO8601 datetime
pub fn parse_datetime(i: &[u8]) -> IResult<&[u8], DateTime> {
    let (i, d) = parse_date(i)?;
    let (i, _) = tag(b"T")(i)?;
    let (i, t) = parse_time(i)?;

    Ok((i, DateTime { date: d, time: t }))
}

// DURATION

// Y[YYY]
fn duration_year(i: &[u8]) -> IResult<&[u8], u32> {
    m_to_n_digit_in_range(i, 1, 4, 0..=9999)
}

// M[M]
fn duration_month(i: &[u8]) -> IResult<&[u8], u32> {
    m_to_n_digit_in_range(i, 1, 2, 0..=12)
}

// W[W]
fn duration_week(i: &[u8]) -> IResult<&[u8], u32> {
    m_to_n_digit_in_range(i, 1, 2, 0..=52)
}

// D[D]
fn duration_day(i: &[u8]) -> IResult<&[u8], u32> {
    m_to_n_digit_in_range(i, 1, 2, 0..=31)
}

// H[H]
fn duration_hour(i: &[u8]) -> IResult<&[u8], u32> {
    m_to_n_digit_in_range(i, 1, 2, 0..=24)
}

// M[M]
fn duration_minute(i: &[u8]) -> IResult<&[u8], u32> {
    m_to_n_digit_in_range(i, 1, 2, 0..=60)
}

// S[S][[,.][MS]]
fn duration_second_and_millisecond(i: &[u8]) -> IResult<&[u8], (u32, u32)> {
    let (i, s) = m_to_n_digit_in_range(i, 1, 2, 0..=60)?;
    let (i, ms) = opt(map(preceded(one_of(",."), fractions), duration_millisecond))(i)?;

    Ok((i, (s, ms.unwrap_or(0))))
}

fn duration_millisecond(fraction: f32) -> u32 {
    (1000.0 * fraction) as u32
}

fn duration_time(i: &[u8]) -> IResult<&[u8], (u32, u32, u32, u32)> {
    let (i, h) = opt(terminated(duration_hour, tag(b"H")))(i)?;
    let (i, m) = opt(terminated(duration_minute, tag(b"M")))(i)?;
    let (i, s) = opt(terminated(duration_second_and_millisecond, tag(b"S")))(i)?;
    let (s, ms) = s.unwrap_or((0, 0));

    Ok((i, (h.unwrap_or(0), m.unwrap_or(0), s, ms)))
}

fn duration_ymdhms(i: &[u8]) -> IResult<&[u8], Duration> {
    let (i, _) = tag(b"P")(i)?;
    let (i, y) = opt(terminated(duration_year, tag(b"Y")))(i)?;
    let (i, mo) = opt(terminated(duration_month, tag(b"M")))(i)?;
    let (i, d) = opt(terminated(duration_day, tag(b"D")))(i)?;
    let (i, time) = opt(preceded(tag(b"T"), duration_time))(i)?;

    // at least one element must be present for a valid duration representation
    if y.is_none() && mo.is_none() && d.is_none() && time.is_none() {
        return Err(nom::Err::Error((i, nom::error::ErrorKind::Eof)));
    }

    let (h, mi, s, ms) = time.unwrap_or((0, 0, 0, 0));

    Ok((
        i,
        Duration::YMDHMS {
            year: y.unwrap_or(0),
            month: mo.unwrap_or(0),
            day: d.unwrap_or(0),
            hour: h,
            minute: mi,
            second: s,
            millisecond: ms,
        },
    ))
}

fn duration_weeks(i: &[u8]) -> IResult<&[u8], Duration> {
    let (i, _) = tag(b"P")(i)?;
    let (i, w) = terminated(duration_week, tag(b"W"))(i)?;

    Ok((i, Duration::Weeks(w)))
}

fn duration_datetime(i: &[u8]) -> IResult<&[u8], Duration> {
    let (i, _) = tag(b"P")(i)?;
    let (i, _) = not(sign)(i)?;
    let (i, dt) = parse_datetime(i)?;

    Ok((i, Duration::DateTime(dt)))
}

pub fn parse_duration(i: &[u8]) -> IResult<&[u8], Duration> {
    alt((duration_ymdhms, duration_weeks, duration_datetime))(i)
}

//! This module is strictly internal.
//!
//! These functions are used by `date()`, `time()` and `datetime()`.
//! They are currently not private, because the need to be accessible,
//! but are not useful by themselves.
//!
//! Please refer to the top-level functions instead, as they offer a better abstraction.
//!
//! **These functions may be made private later.**

use core::str;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, take_while_m_n},
    character::complete::one_of,
    character::is_digit,
    combinator::{map, map_res, not, opt},
    error::Error,
    sequence::{preceded, separated_pair, terminated, tuple},
    Err, IResult,
};

use crate::{Date, DateTime, Duration, Time};

#[cfg(test)]
mod tests;

// UTILITY

fn take_digits(i: &[u8]) -> IResult<&[u8], u32> {
    let (i, digits) = take_while(is_digit)(i)?;

    if digits.is_empty() {
        return Err(Err::Error(Error::new(i, nom::error::ErrorKind::Eof)));
    }

    let s = str::from_utf8(digits).expect("Invalid data, expected UTF-8 string");
    let res = s
        .parse()
        .expect("Invalid string, expected ASCII representation of a number");

    Ok((i, res))
}

fn take_n_digits(i: &[u8], n: usize) -> IResult<&[u8], u32> {
    let (i, digits) = take_while_m_n(n, n, is_digit)(i)?;

    let s = str::from_utf8(digits).expect("Invalid data, expected UTF-8 string");
    let res = s
        .parse()
        .expect("Invalid string, expected ASCII representation of a number");

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
    range: impl core::ops::RangeBounds<u32>,
) -> IResult<&[u8], u32> {
    let (new_i, number) = take_n_digits(i, n)?;

    if range.contains(&number) {
        Ok((new_i, number))
    } else {
        Err(Err::Error(Error::new(i, nom::error::ErrorKind::Eof)))
    }
}

fn m_to_n_digit_in_range(
    i: &[u8],
    m: usize,
    n: usize,
    range: impl core::ops::RangeBounds<u32>,
) -> IResult<&[u8], u32> {
    let (new_i, number) = take_m_to_n_digits(i, m, n)?;

    if range.contains(&number) {
        Ok((new_i, number))
    } else {
        Err(Err::Error(Error::new(i, nom::error::ErrorKind::Eof)))
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
    map(
        tuple((
            opt(sign),               // [+/-]
            |i| take_n_digits(i, 4), // year
        )),
        |(s, year)| s.unwrap_or(1) * year as i32,
    )(i)
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
    map(
        tuple((
            date_year,      // YYYY
            opt(tag(b"-")), // -
            date_month,     // MM
            opt(tag(b"-")), // -
            date_day,       //DD
        )),
        |(year, _, month, _, day)| Date::YMD { year, month, day },
    )(i)
}

// YYYY-DDD
fn date_ordinal(i: &[u8]) -> IResult<&[u8], Date> {
    map(
        separated_pair(date_year, opt(tag(b"-")), date_ord_day),
        |(year, ddd)| Date::Ordinal { year, ddd },
    )(i)
}

// YYYY-"W"WW-D
fn date_iso_week(i: &[u8]) -> IResult<&[u8], Date> {
    map(
        tuple((
            date_year,                          // y
            tuple((opt(tag(b"-")), tag(b"W"))), // [-]W
            date_week,                          // w
            opt(tag(b"-")),                     // [-]
            date_week_day,                      // d
        )),
        |(year, _, ww, _, d)| Date::Week { year, ww, d },
    )(i)
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
    map(
        tuple((
            time_hour,                                                     // HH
            opt(tag(b":")),                                                // :
            time_minute,                                                   // MM
            opt(preceded(opt(tag(b":")), time_second)),                    // [SS]
            opt(map(preceded(one_of(",."), fractions), time_millisecond)), // [.(m*)]
            opt(alt((timezone_hour, timezone_utc))),                       // [(Z|+...|-...)]
        )),
        |(h, _, m, s, ms, z)| {
            let (tz_offset_hours, tz_offset_minutes) = z.unwrap_or((0, 0));

            Time {
                hour: h,
                minute: m,
                second: s.unwrap_or(0),
                millisecond: ms.unwrap_or(0),
                tz_offset_hours,
                tz_offset_minutes,
            }
        },
    )(i)
}

fn timezone_hour(i: &[u8]) -> IResult<&[u8], (i32, i32)> {
    map(
        tuple((sign, time_hour, opt(preceded(opt(tag(b":")), time_minute)))),
        |(s, h, m)| (s * (h as i32), s * (m.unwrap_or(0) as i32)),
    )(i)
}

fn timezone_utc(i: &[u8]) -> IResult<&[u8], (i32, i32)> {
    map(tag(b"Z"), |_| (0, 0))(i)
}

// Full ISO8601 datetime
pub fn parse_datetime(i: &[u8]) -> IResult<&[u8], DateTime> {
    map(
        separated_pair(parse_date, tag(b"T"), parse_time),
        |(d, t)| DateTime { date: d, time: t },
    )(i)
}

// DURATION

// Y[YYY...]
fn duration_year(i: &[u8]) -> IResult<&[u8], u32> {
    take_digits(i)
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
    map(
        tuple((
            opt(terminated(duration_hour, tag(b"H"))),
            opt(terminated(duration_minute, tag(b"M"))),
            opt(terminated(duration_second_and_millisecond, tag(b"S"))),
        )),
        |(h, m, s)| {
            let (s, ms) = s.unwrap_or((0, 0));

            (h.unwrap_or(0), m.unwrap_or(0), s, ms)
        },
    )(i)
}

fn duration_ymdhms(i: &[u8]) -> IResult<&[u8], Duration> {
    map_res(
        preceded(
            tag(b"P"),
            tuple((
                opt(terminated(duration_year, tag(b"Y"))),
                opt(terminated(duration_month, tag(b"M"))),
                opt(terminated(duration_day, tag(b"D"))),
                opt(preceded(tag(b"T"), duration_time)),
            )),
        ),
        |(y, mo, d, time)| {
            // at least one element must be present for a valid duration representation
            if y.is_none() && mo.is_none() && d.is_none() && time.is_none() {
                return Err(Err::Error((i, nom::error::ErrorKind::Eof)));
            }

            let (h, mi, s, ms) = time.unwrap_or((0, 0, 0, 0));

            Ok(Duration::YMDHMS {
                year: y.unwrap_or(0),
                month: mo.unwrap_or(0),
                day: d.unwrap_or(0),
                hour: h,
                minute: mi,
                second: s,
                millisecond: ms,
            })
        },
    )(i)
}

fn duration_weeks(i: &[u8]) -> IResult<&[u8], Duration> {
    map(
        preceded(tag(b"P"), terminated(duration_week, tag(b"W"))),
        Duration::Weeks,
    )(i)
}

// YYYY, no sign
fn duration_datetime_year(i: &[u8]) -> IResult<&[u8], u32> {
    take_n_digits(i, 4)
}

fn duration_datetime(i: &[u8]) -> IResult<&[u8], Duration> {
    map(
        preceded(
            tuple((tag(b"P"), not(sign))),
            tuple((
                duration_datetime_year,
                opt(tag(b"-")),
                date_month,
                opt(tag(b"-")),
                date_day,
                tag(b"T"),
                parse_time,
            )),
        ),
        |(year, _, month, _, day, _, t)| Duration::YMDHMS {
            year,
            month,
            day,
            hour: t.hour,
            minute: t.minute,
            second: t.second,
            millisecond: t.millisecond,
        },
    )(i)
}

pub fn parse_duration(i: &[u8]) -> IResult<&[u8], Duration> {
    alt((duration_ymdhms, duration_weeks, duration_datetime))(i)
}

//! The low-level parsers for date, datetime, duration and time.
//!
//! The top-level functions [`date()`][`crate::date()`], [`datetime()`][`crate::datetime()`],
//! [`duration()`][`crate::duration()`] and [`time()`][`crate::time()`]
//! provide convenient wrappers around the low-level parsers,
//! but throw away leftover input on success.
//!
//! Using the low-level functions provided here allows to recover leftover input
//! or to combine these parsers with other parser combinators.

use core::str;
use std::ops::RangeBounds;
use winnow::ascii::digit1;
use winnow::combinator::{alt, trace};
use winnow::combinator::{not, opt};
use winnow::combinator::{preceded, separated_pair, terminated};
use winnow::error::{ContextError, ErrMode};
use winnow::token::one_of;
use winnow::token::{literal, take_while};
use winnow::{seq, PResult, Parser, Partial};

use crate::{Date, DateTime, Duration, Time, Timezone};

#[cfg(test)]
mod tests;

/// Type for holding partial data for parsers
pub type Stream<'i> = Partial<&'i [u8]>;

// UTILITY

fn take_digits(i: &mut Stream) -> PResult<u32> {
    trace("take_digits", move |input: &mut Stream<'_>| {
        let digits = take_while(1.., |c: u8| char::from(c).is_digit(10)).parse_next(input)?;

        if digits.is_empty() {
            return Err(ErrMode::Backtrack(ContextError::new()));
        }

        let s = str::from_utf8(digits).expect("Invalid data, expected UTF-8 string");
        let res = s
            .parse()
            .expect("Invalid string, expected ASCII representation of a number");

        Ok(res)
    })
    .parse_next(i)
}

fn take_digits_in_range(
    i: &mut Stream,
    places: usize,
    range: impl RangeBounds<u32>,
) -> PResult<u32> {
    let n = take_while(places, |c: u8| char::from(c).is_digit(10)).parse_next(i)?;

    let s = str::from_utf8(n).expect("Invalid data, expected UTF-8 string");

    let number: u32 = s
        .parse()
        .expect("Invalid string, expected ASCII representation of a number");

    if range.contains(&number) {
        Ok(number)
    } else {
        return Err(ErrMode::Backtrack(ContextError::new()));
    }
}

fn sign(i: &mut Stream) -> PResult<i32> {
    alt((literal(b"-"), literal(b"+")))
        .map(|s: &[u8]| match s {
            b"-" => -1,
            _ => 1,
        })
        .parse_next(i)
}

// DATE

// [+/-]YYYY
fn date_year(i: &mut Stream) -> PResult<i32> {
    trace("date_year", move |input: &mut Stream<'_>| {
        // The sign is optional, but defaults to `+`
        let sign = opt(sign).parse_next(input)?.unwrap_or(1);
        let year = take_digits_in_range(input, 4, 100..)?;
        Ok(sign * year as i32)
    })
    .parse_next(i)
}

// MM
fn date_month(i: &mut Stream) -> PResult<u32> {
    trace("date_month", move |input: &mut Stream<'_>| {
        take_digits_in_range(input, 2, 1..=12)
    })
    .parse_next(i)
}

// DD
fn date_day(i: &mut Stream) -> PResult<u32> {
    trace("date_day", move |input: &mut Stream<'_>| {
        take_digits_in_range(input, 2, 1..=31)
    })
    .parse_next(i)
}

// WW
fn date_week(i: &mut Stream) -> PResult<u32> {
    trace("date_week", move |input: &mut Stream<'_>| {
        take_digits_in_range(input, 2, 1..=52)
    })
    .parse_next(i)
}

fn date_week_day(i: &mut Stream) -> PResult<u32> {
    trace("date_week_day", move |input: &mut Stream<'_>| {
        take_digits_in_range(input, 1, 1..=7)
    })
    .parse_next(i)
}

// ordinal DDD
fn date_ord_day(i: &mut Stream) -> PResult<u32> {
    trace("date_ord_day", move |input: &mut Stream<'_>| {
        take_digits_in_range(input, 3, 1..=366)
    })
    .parse_next(i)
}

// YYYY-MM-DD
fn date_ymd(i: &mut Stream) -> PResult<Date> {
    trace("date_ymd", move |input: &mut Stream<'_>| {
        seq!((
        date_year,      // YYYY
        _: opt(literal(b"-")), // -
        date_month,     // MM
        _: opt(literal(b"-")), // -
        date_day,       //DD
        ))
        .map(|(year, month, day)| Date::YMD { year, month, day })
        .parse_next(input)
    })
    .parse_next(i)
}

// YYYY-DDD
fn date_ordinal(i: &mut Stream) -> PResult<Date> {
    trace("date_ordinal", move |input: &mut Stream<'_>| {
        separated_pair(date_year, opt(literal(b"-")), date_ord_day)
            .map(|(year, ddd)| Date::Ordinal { year, ddd })
            .parse_next(input)
    })
    .parse_next(i)
}

// YYYY-"W"WW-D
fn date_iso_week(i: &mut Stream) -> PResult<Date> {
    trace("", move |input: &mut Stream<'_>| {
        seq!((
            date_year,                                 // y
            seq!((opt(literal(b"-")), literal(b"W"))), // [-]W
            date_week,                                 // w
            opt(literal(b"-")),                        // [-]
            date_week_day,                             // d
        ))
        .map(|(year, _, ww, _, d)| Date::Week { year, ww, d })
        .parse_next(input)
    })
    .parse_next(i)
}

/// Parses a date string.
///
/// See [`date()`][`crate::date()`] for the supported formats.
pub fn parse_date(i: &mut Stream) -> PResult<Date> {
    trace("parse_date", move |input: &mut Stream<'_>| {
        alt((date_ymd, date_iso_week, date_ordinal)).parse_next(input)
    })
    .parse_next(i)
}

// TIME

// HH
fn time_hour(i: &mut Stream) -> PResult<u32> {
    trace("time_hour", move |input: &mut Stream<'_>| {
        take_digits_in_range(input, 2, 0..=24)
    })
    .parse_next(i)
}

// MM
fn time_minute(i: &mut Stream) -> PResult<u32> {
    trace("time_minute", move |input: &mut Stream<'_>| {
        take_digits_in_range(input, 2, 0..=59)
    })
    .parse_next(i)
}

// SS
fn time_second(i: &mut Stream) -> PResult<u32> {
    trace("time_second", move |input: &mut Stream<'_>| {
        take_digits_in_range(input, 2, 0..=60)
    })
    .parse_next(i)
}

// Converts the fractional part if-any of a number of seconds to milliseconds
// truncating towards zero if there are more than three digits.
// e.g. "" -> 0, "1" -> 100, "12" -> 120, "123" -> 123, "1234" -> 123
fn fraction_millisecond(i: &mut Stream) -> PResult<u32> {
    trace("fraction_millisecond", move |input: &mut Stream<'_>| {
        let mut digits = digit1(input)?;
        let mut l = digits.len();
        if l > 3 {
            digits = digits.get(0..3).unwrap();
        }
        let mut result = 0;
        if l > 0 {
            let digits = str::from_utf8(digits).unwrap(); // This can't panic, `digits` will only include digits.
            result = digits.parse().unwrap();
        }
        while l < 3 {
            result *= 10;
            l += 1;
        }
        Ok(result)
    })
    .parse_next(i)
}

/// Parses a time string.
///
/// See [`time()`][`crate::time()`] for the supported formats.
// HH:MM:[SS][.(m*)][(Z|+...|-...)]
pub fn parse_time(i: &mut Stream) -> PResult<Time> {
    trace("parse_time", move |input: &mut Stream<'_>| {
        let t = seq! {Time {
            hour: time_hour,                                         // HH
            _: opt(literal(b":")),                                    // :
            minute: time_minute,                                       // MM
            second: opt(preceded(opt(literal(b":")), time_second)).map(|d| d.unwrap_or(0)),        // [SS]
            millisecond: opt(preceded(one_of(b",."), fraction_millisecond)).map(|d| d.unwrap_or(0)), // [.(m*)]
            timezone: opt(parse_timezone).map(|tz| tz.unwrap_or(Default::default())),           // [(Z|+...|-...)]
        }}
        .parse_next(input)?;
        Ok(t)
    })
    .parse_next(i)
}

/// Parses a timezone offset string.
///
/// See [`timezone()`][`crate::timezone()`] for the supported formats.
// (Z|+...|-...)
pub fn parse_timezone(i: &mut Stream) -> PResult<Timezone> {
    trace("timezone_hour", move |input: &mut Stream<'_>| {
        alt((timezone_hour, timezone_utc)).parse_next(input)
    })
    .parse_next(i)
}

// (+...|-...)
fn timezone_hour(i: &mut Stream) -> PResult<Timezone> {
    trace("timezone_hour", move |input: &mut Stream<'_>| {
        seq!((
            sign,
            time_hour,
            opt(preceded(opt(literal(b":")), time_minute))
        ))
        .map(|(s, h, m)| Timezone {
            offset_hours: s * (h as i32),
            offset_minutes: s * (m.unwrap_or(0) as i32),
        })
        .parse_next(input)
    })
    .parse_next(i)
}

// Z
fn timezone_utc(i: &mut Stream) -> PResult<Timezone> {
    trace("timezone_utc", move |input: &mut Stream<'_>| {
        literal(b"Z").map(|_| Timezone::default()).parse_next(input)
    })
    .parse_next(i)
}

/// Parses a datetime string.
///
/// See [`datetime()`][`crate::datetime()`] for supported formats.
// Full ISO8601 datetime
pub fn parse_datetime(i: &mut Stream) -> PResult<DateTime> {
    trace("parse_datetime", move |input: &mut Stream<'_>| {
        separated_pair(parse_date, literal(b"T"), parse_time)
            .map(|(d, t)| DateTime { date: d, time: t })
            .parse_next(input)
    })
    .parse_next(i)
}

// DURATION

///    dur-year          = 1*DIGIT "Y" [dur-month]
fn duration_year(i: &mut Stream) -> PResult<u32> {
    trace("duration_year", move |input: &mut Stream<'_>| {
        (terminated(take_digits, literal(b"Y"))).parse_next(input)
    })
    .parse_next(i)
}

///    dur-month         = 1*DIGIT "M" [dur-day]
fn duration_month(i: &mut Stream) -> PResult<u32> {
    trace("duration_month", move |input: &mut Stream<'_>| {
        terminated(take_digits, literal(b"M")).parse_next(input)
    })
    .parse_next(i)
}

///    dur-week          = 1*DIGIT "W"
fn duration_week(i: &mut Stream) -> PResult<u32> {
    trace("duration_week", move |input: &mut Stream<'_>| {
        terminated(take_digits, literal(b"W")).parse_next(input)
    })
    .parse_next(i)
}

//    dur-day           = 1*DIGIT "D"
fn duration_day(i: &mut Stream) -> PResult<u32> {
    trace("duration_day", move |input: &mut Stream<'_>| {
        terminated(take_digits, literal(b"D")).parse_next(input)
    })
    .parse_next(i)
}

///    dur-hour          = 1*DIGIT "H" [dur-minute]
///    dur-time          = "T" (dur-hour / dur-minute / dur-second)
fn duration_hour(i: &mut Stream) -> PResult<u32> {
    trace("duration_hour", move |input: &mut Stream<'_>| {
        terminated(take_digits, literal(b"H")).parse_next(input)
    })
    .parse_next(i)
}

///    dur-minute        = 1*DIGIT "M" [dur-second]
fn duration_minute(i: &mut Stream) -> PResult<u32> {
    trace("", move |input: &mut Stream<'_>| {
        terminated(take_digits, literal(b"M")).parse_next(input)
    })
    .parse_next(i)
}

///    dur-second        = 1*DIGIT "S"
fn duration_second(i: &mut Stream) -> PResult<u32> {
    trace("duration_second", move |input: &mut Stream<'_>| {
        terminated(take_digits, literal(b"S")).parse_next(input)
    })
    .parse_next(i)
}

///    dur-second-ext    = 1*DIGIT (,|.) 1*DIGIT "S"
fn duration_second_and_millisecond(i: &mut Stream) -> PResult<(u32, u32)> {
    trace(
        "duration_second_and_millisecond",
        move |input: &mut Stream<'_>| {
            alt((
                // no milliseconds
                duration_second.map(|m| (m, 0)),
                terminated(
                    // with milliseconds
                    separated_pair(take_digits, one_of(b",."), fraction_millisecond),
                    literal(b"S"),
                ),
            ))
            .parse_next(input)
        },
    )
    .parse_next(i)
}

fn duration_time(i: &mut Stream) -> PResult<(u32, u32, u32, u32)> {
    trace("duration_time", move |input: &mut Stream<'_>| {
        seq!((
            opt(duration_hour),
            opt(duration_minute),
            opt(duration_second_and_millisecond),
        ))
        .map(|(h, m, s)| {
            let (s, ms) = s.unwrap_or((0, 0));

            (h.unwrap_or(0), m.unwrap_or(0), s, ms)
        })
        .parse_next(input)
    })
    .parse_next(i)
}

fn duration_ymdhms(i: &mut Stream) -> PResult<Duration> {
    trace("", move |input: &mut Stream<'_>| {
        seq!((
            _: literal(b"P"),
            opt(duration_year),
            opt(duration_month),
            opt(duration_day),
            opt(preceded(literal(b"T"), duration_time)),
        ))
        .verify(|(y, mo, d, time)| {
            if y.is_none() && mo.is_none() && d.is_none() && time.is_none() {
                false
            } else {
                true
            }
        })
        .map(|(y, mo, d, time)| {
            // at least one element must be present for a valid duration representation

            let (h, mi, s, ms) = time.unwrap_or((0, 0, 0, 0));

            Duration::YMDHMS {
                year: y.unwrap_or(0),
                month: mo.unwrap_or(0),
                day: d.unwrap_or(0),
                hour: h,
                minute: mi,
                second: s,
                millisecond: ms,
            }
        })
        .parse_next(input)
    })
    .parse_next(i)
}

fn duration_weeks(i: &mut Stream) -> PResult<Duration> {
    trace("", move |input: &mut Stream<'_>| {
        preceded(literal(b"P"), duration_week)
            .map(Duration::Weeks)
            .parse_next(input)
    })
    .parse_next(i)
}

// YYYY, no sign
fn duration_datetime_year(i: &mut Stream) -> PResult<u32> {
    trace("duration_datetime_year", move |input: &mut Stream<'_>| {
        take_digits_in_range(input, 4, 1..)
    })
    .parse_next(i)
}

fn duration_datetime(i: &mut Stream) -> PResult<Duration> {
    trace("duration_datetime", move |input: &mut Stream<'_>| {
        preceded(
            seq!((literal(b"P"), not(sign))),
            seq!((
                duration_datetime_year,
                opt(literal(b"-")),
                date_month,
                opt(literal(b"-")),
                date_day,
                literal(b"T"),
                parse_time,
            )),
        )
        .map(|(year, _, month, _, day, _, t)| Duration::YMDHMS {
            year,
            month,
            day,
            hour: t.hour,
            minute: t.minute,
            second: t.second,
            millisecond: t.millisecond,
        })
        .parse_next(input)
    })
    .parse_next(i)
}

/// Parses a duration string.
///
/// See [`duration()`][`crate::duration()`] for supported formats.
pub fn parse_duration(i: &mut Stream) -> PResult<Duration> {
    trace("parse_duration", move |input: &mut Stream<'_>| {
        alt((duration_ymdhms, duration_weeks, duration_datetime)).parse_next(input)
    })
    .parse_next(i)
}

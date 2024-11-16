use core::fmt::Debug;

use super::*;
use winnow::stream::{AsBytes, StreamIsPartial};

fn test_date_part<O: Debug + PartialEq + Eq>(
    parser: &dyn Fn(&mut Stream) -> PResult<O>,
    i: &str,
    rem: &str,
    exepected: O,
) -> PResult<O> {
    let mut input = Stream::new(i.as_bytes());
    let _ = input.complete();
    let part = parser(&mut input)?;

    assert_eq!(rem.as_bytes(), input.as_bytes());
    assert_eq!(exepected, part);

    Ok(part)
}

#[test]
fn test_date_year() {
    test_date_part(&date_year, "2015\n", "\n", 2015).unwrap();
    test_date_part(&date_year, "2015\n", "\n", 2015).unwrap();
    test_date_part(&date_year, "-0333\n", "\n", -333).unwrap();
    test_date_part(&date_year, "2015-\n", "-\n", 2015).unwrap();
    assert!(date_year(&mut Stream::new(b"abcd")).is_err());
    assert!(date_year(&mut Stream::new(b"2a03")).is_err());
}

#[test]
fn test_date_month() {
    test_date_part(&date_month, "01", "", 1).unwrap();
    test_date_part(&date_month, "06", "", 6).unwrap();
    test_date_part(&date_month, "12", "", 12).unwrap();
    test_date_part(&date_month, "12-", "-", 12).unwrap();

    assert!(date_month(&mut Stream::new(b"13\n")).is_err());
    assert!(date_month(&mut Stream::new(b"00\n")).is_err());
}

#[test]
fn test_date_day() {
    test_date_part(&date_day, "01", "", 1).unwrap();
    test_date_part(&date_day, "12", "", 12).unwrap();
    test_date_part(&date_day, "20", "", 20).unwrap();
    test_date_part(&date_day, "28", "", 28).unwrap();
    test_date_part(&date_day, "30", "", 30).unwrap();
    test_date_part(&date_day, "31", "", 31).unwrap();
    test_date_part(&date_day, "31-", "-", 31).unwrap();

    assert!(date_day(&mut Stream::new(b"00")).is_err());
    assert!(date_day(&mut Stream::new(b"32")).is_err());
}

#[test]
fn test_time_hour() {
    test_date_part(&time_hour, "00", "", 0).unwrap();
    test_date_part(&time_hour, "01", "", 1).unwrap();
    test_date_part(&time_hour, "06", "", 6).unwrap();
    test_date_part(&time_hour, "12", "", 12).unwrap();
    test_date_part(&time_hour, "13", "", 13).unwrap();
    test_date_part(&time_hour, "20", "", 20).unwrap();
    test_date_part(&time_hour, "24", "", 24).unwrap();

    assert!(time_hour(&mut Stream::new(b"25")).is_err());
    assert!(time_hour(&mut Stream::new(b"30")).is_err());
    assert!(time_hour(&mut Stream::new(b"ab")).is_err());
}

#[test]
fn test_time_minute() {
    test_date_part(&time_minute, "00", "", 0).unwrap();
    test_date_part(&time_minute, "01", "", 1).unwrap();
    test_date_part(&time_minute, "30", "", 30).unwrap();
    test_date_part(&time_minute, "59", "", 59).unwrap();

    assert!(time_minute(&mut Stream::new(b"60")).is_err());
    assert!(time_minute(&mut Stream::new(b"61")).is_err());
    assert!(time_minute(&mut Stream::new(b"ab")).is_err());
}

#[test]
fn test_time_second() {
    test_date_part(&time_second, "00", "", 0).unwrap();
    test_date_part(&time_second, "01", "", 1).unwrap();
    test_date_part(&time_second, "30", "", 30).unwrap();
    test_date_part(&time_second, "59", "", 59).unwrap();
    test_date_part(&time_second, "60", "", 60).unwrap();

    assert!(time_second(&mut Stream::new(b"61")).is_err());
    assert!(time_second(&mut Stream::new(b"ab")).is_err());
}

#[test]
fn test_date() {
    assert!(parse_date(&mut Stream::new(b"201")).is_err());
    assert!(parse_date(&mut Stream::new(b"2015p00p00")).is_err());
    assert!(parse_date(&mut Stream::new(b"pppp")).is_err());
}

#[test]
fn test_time() {
    assert!(parse_time(&mut Stream::new(b"20:")).is_err());
    assert!(parse_time(&mut Stream::new(b"20p42p16")).is_err());
    assert!(parse_time(&mut Stream::new(b"pppp")).is_err());
}

#[test]
fn test_time_with_timezone() {
    assert!(parse_time(&mut Stream::new(b"20:")).is_err());
    assert!(parse_time(&mut Stream::new(b"20p42p16")).is_err());
    assert!(parse_time(&mut Stream::new(b"pppp")).is_err());
}

#[test]
fn test_date_iso_week_date() {
    assert!(date_iso_week(&mut Stream::new(b"2015-W06-8")).is_err());
    assert!(date_iso_week(&mut Stream::new(b"2015-W068")).is_err());
    assert!(date_iso_week(&mut Stream::new(b"2015-W06-0")).is_err());
    assert!(date_iso_week(&mut Stream::new(b"2015-W00-2")).is_err());
    assert!(date_iso_week(&mut Stream::new(b"2015-W54-2")).is_err());
    assert!(date_iso_week(&mut Stream::new(b"2015-W542")).is_err());
}

#[test]
fn test_date_ordinal_date() {
    // not valid here either
    assert!(date_ordinal(&mut Stream::new(b"2015-400")).is_err());
}

#[test]
fn format_equivalence() {
    assert_eq!(
        parse_datetime(&mut Stream::new(b"2001-02-03T04:05:06+07:00")),
        parse_datetime(&mut Stream::new(b"20010203T040506+0700"))
    );
    assert_eq!(
        parse_datetime(&mut Stream::new(b"2001-02-03T04:05:06+07:00")),
        parse_datetime(&mut Stream::new(b"20010203T04:05:06+0700"))
    );
    assert_eq!(
        parse_datetime(&mut Stream::new(b"2001-02-03T04:05:00+07:00")),
        parse_datetime(&mut Stream::new(b"20010203T0405+0700"))
    );
    assert_eq!(
        parse_datetime(&mut Stream::new(b"20010203T0405+0700")),
        parse_datetime(&mut Stream::new(b"2001-02-03T0405+0700"))
    );
    assert_eq!(
        parse_datetime(&mut Stream::new(b"20010203T040506+0700")),
        parse_datetime(&mut Stream::new(b"2001-02-03T040506+0700"))
    );
    assert_eq!(
        parse_datetime(&mut Stream::new(b"20010203T040506+0000")),
        parse_datetime(&mut Stream::new(b"20010203T040506Z"))
    );
    assert_eq!(
        parse_datetime(&mut Stream::new(b"2015W056T04:05:06+07:00")),
        parse_datetime(&mut Stream::new(b"2015-W05-6T04:05:06+07:00"))
    );
}

#[test]
fn test_datetime_error() {
    let test_datetimes = vec!["ppp", "dumd-di-duTmd:iu:m"];

    for iso_string in test_datetimes {
        let res = parse_datetime(&mut Stream::new(iso_string.as_bytes()));
        assert!(res.is_err());
    }
}

#[test]
fn disallows_notallowed() {
    assert!(parse_time(&mut Stream::new(b"30:90:90")).is_err());
    assert!(parse_date(&mut Stream::new(b"0000-20-40")).is_err());
    assert!(parse_datetime(&mut Stream::new(b"2001-w05-6t04:05:06.123z")).is_err());
}

#[test]
fn test_duration_year() {
    test_date_part(&duration_year, "2019Y", "", 2019).unwrap();
    test_date_part(&duration_year, "0Y", "", 0).unwrap();
    test_date_part(&duration_year, "10000Y", "", 10000).unwrap();
    assert!(duration_year(&mut Stream::new(b"abcd")).is_err());
    assert!(duration_year(&mut Stream::new(b"-1")).is_err());
}

#[test]
fn test_duration_month() {
    test_date_part(&duration_month, "6M", "", 6).unwrap();
    test_date_part(&duration_month, "0M", "", 0).unwrap();
    test_date_part(&duration_month, "12M", "", 12).unwrap();

    assert!(duration_month(&mut Stream::new(b"ab")).is_err());
    assert!(duration_month(&mut Stream::new(b"-1")).is_err());
    assert!(duration_month(&mut Stream::new(b"13")).is_err());
}

#[test]
fn test_duration_week() {
    test_date_part(&duration_week, "26W", "", 26).unwrap();
    test_date_part(&duration_week, "0W", "", 0).unwrap();
    test_date_part(&duration_week, "52W", "", 52).unwrap();
    assert!(duration_week(&mut Stream::new(b"ab")).is_err());
    assert!(duration_week(&mut Stream::new(b"-1")).is_err());
    assert!(duration_week(&mut Stream::new(b"53")).is_err());
}

#[test]
fn test_duration_day() {
    test_date_part(&duration_day, "16D", "", 16).unwrap();
    test_date_part(&duration_day, "0D", "", 0).unwrap();
    test_date_part(&duration_day, "31D", "", 31).unwrap();
    assert!(duration_day(&mut Stream::new(b"ab")).is_err());
    assert!(duration_day(&mut Stream::new(b"-1")).is_err());
    assert!(duration_day(&mut Stream::new(b"32")).is_err());
}

#[test]
fn test_duration_hour() {
    test_date_part(&duration_hour, "12H", "", 12).unwrap();
    test_date_part(&duration_hour, "0H", "", 0).unwrap();
    test_date_part(&duration_hour, "24H", "", 24).unwrap();
    assert!(duration_hour(&mut Stream::new(b"ab")).is_err());
    assert!(duration_hour(&mut Stream::new(b"-1")).is_err());
    assert!(duration_hour(&mut Stream::new(b"25")).is_err());
}

#[test]
fn test_duration_minute() {
    test_date_part(&duration_minute, "30M", "", 30).unwrap();
    test_date_part(&duration_minute, "0M", "", 0).unwrap();
    test_date_part(&duration_minute, "60M", "", 60).unwrap();
    assert!(duration_minute(&mut Stream::new(b"ab")).is_err());
    assert!(duration_minute(&mut Stream::new(b"-1")).is_err());
    assert!(duration_minute(&mut Stream::new(b"61")).is_err());
}

#[test]
fn test_duration_second_and_millisecond1() {
    test_date_part(&duration_second_and_millisecond, "30S", "", (30, 0)).unwrap();
    test_date_part(&duration_second_and_millisecond, "0S", "", (0, 0)).unwrap();
    test_date_part(&duration_second_and_millisecond, "60S", "", (60, 0)).unwrap();
    test_date_part(&duration_second_and_millisecond, "1,23S", "", (1, 230)).unwrap();
    test_date_part(&duration_second_and_millisecond, "2.34S", "", (2, 340)).unwrap();
    assert!(duration_second_and_millisecond(&mut Stream::new(b"abS")).is_err());
    assert!(duration_second_and_millisecond(&mut Stream::new(b"-1S")).is_err());
}

#[test]
fn test_duration_time() {
    test_date_part(&duration_time, "1H2M3S", "", (1, 2, 3, 0)).unwrap();
    test_date_part(&duration_time, "10H12M30S", "", (10, 12, 30, 0)).unwrap();
    test_date_part(&duration_time, "1H3S", "", (1, 0, 3, 0)).unwrap();
    test_date_part(&duration_time, "2M", "", (0, 2, 0, 0)).unwrap();
    test_date_part(&duration_time, "1H2M3,4S", "", (1, 2, 3, 400)).unwrap();
    test_date_part(&duration_time, "1H2M3.4S", "", (1, 2, 3, 400)).unwrap();
    test_date_part(&duration_time, "0,123S", "", (0, 0, 0, 123)).unwrap();
    test_date_part(&duration_time, "0.123S", "", (0, 0, 0, 123)).unwrap();
}

#[test]
fn test_duration_ymdhms_error() {
    assert!(duration_ymdhms(&mut Stream::new(b"")).is_err());
    assert!(duration_ymdhms(&mut Stream::new(b"P")).is_err()); // empty duration is not 0 seconds
    assert!(duration_ymdhms(&mut Stream::new(b"1Y2M3DT4H5M6S")).is_err()); // missing P at start
    assert!(duration_ymdhms(&mut Stream::new(b"T4H5M6S")).is_err()); // missing P, required even if no YMD part
}

#[test]
fn test_duration_weeks_error() {
    assert!(duration_weeks(&mut Stream::new(b"")).is_err());
    assert!(duration_weeks(&mut Stream::new(b"P")).is_err()); // empty duration is not 0 seconds
    assert!(duration_weeks(&mut Stream::new(b"P1")).is_err()); // missing W after number
    assert!(duration_weeks(&mut Stream::new(b"PW")).is_err()); // missing number
}

#[test]
fn test_duration_datetime_error() {
    assert!(duration_datetime(&mut Stream::new(b"")).is_err());
    assert!(duration_datetime(&mut Stream::new(b"P")).is_err()); // empty duration is not 0 seconds
    assert!(duration_datetime(&mut Stream::new(b"0001-02-03T04:05:06")).is_err());
    // missing P at start
}

#[rustfmt::skip]
#[test]
fn test_duration_second_and_millisecond2() {
    test_date_part(&parse_duration, "PT30S", "", Duration::YMDHMS { year: 0, month: 0, day: 0, hour: 0, minute: 0, second: 30, millisecond: 0 }).unwrap();
    test_date_part(&parse_duration, "PT30.123S", "", Duration::YMDHMS { year: 0, month: 0, day: 0, hour: 0, minute: 0, second: 30, millisecond: 123 }).unwrap();
    test_date_part(&parse_duration, "P2021Y11M16DT23H26M59.123S", "", Duration::YMDHMS { year: 2021, month: 11, day: 16, hour: 23, minute: 26, second: 59, millisecond: 123 }).unwrap();
}

#[rustfmt::skip]
#[test]
fn duration_roundtrip() {
    test_date_part(&parse_duration, "P0W", "", Duration::Weeks(0)).unwrap();
    test_date_part(&parse_duration, "P2021Y11M16DT23H26M59.123S", "", Duration::YMDHMS { year: 2021, month: 11, day: 16, hour: 23, minute: 26, second: 59, millisecond: 123 }).unwrap();
    test_date_part(&parse_duration, "P2021Y11M16DT23H26M59S", "", Duration::YMDHMS { year: 2021, month: 11, day: 16, hour: 23, minute: 26, second: 59, millisecond: 0 }).unwrap();
    test_date_part(&parse_duration, "P2021Y11M16DT23H26M", "", Duration::YMDHMS { year: 2021, month: 11, day: 16, hour: 23, minute: 26, second: 0, millisecond: 0 }).unwrap();
    test_date_part(&parse_duration, "P2021Y11M16DT23H", "", Duration::YMDHMS { year: 2021, month: 11, day: 16, hour: 23, minute: 0, second: 0, millisecond: 0 }).unwrap();
    test_date_part(&parse_duration, "P2021Y11M16D", "", Duration::YMDHMS { year: 2021, month: 11, day: 16, hour: 0, minute: 0, second: 0, millisecond: 0 }).unwrap();
    test_date_part(&parse_duration, "P2021Y11M16DT1S", "", Duration::YMDHMS { year: 2021, month: 11, day: 16, hour: 0, minute: 0, second: 1, millisecond: 0 }).unwrap();
    test_date_part(&parse_duration, "P2021Y11M16DT0.471S", "", Duration::YMDHMS { year: 2021, month: 11, day: 16, hour: 0, minute: 0, second: 0, millisecond: 471 }).unwrap();
    test_date_part(&parse_duration, "P2021Y11M", "", Duration::YMDHMS { year: 2021, month: 11, day: 0, hour: 0, minute: 0, second: 0, millisecond: 0 }).unwrap();
    test_date_part(&parse_duration, "P11M", "", Duration::YMDHMS { year: 0, month: 11, day: 0, hour: 0, minute: 0, second: 0, millisecond: 0 }).unwrap();
    test_date_part(&parse_duration, "P16D", "", Duration::YMDHMS { year: 0, month: 0, day: 16, hour: 0, minute: 0, second: 0, millisecond: 0 }).unwrap();
    test_date_part(&parse_duration, "P0D", "", Duration::YMDHMS { year: 0, month: 0, day: 0, hour: 0, minute: 0, second: 0, millisecond: 0 }).unwrap();
}

#[rustfmt::skip]
#[test]
fn duration_multi_digit_hour() {
    test_date_part(&parse_duration, "PT12H", "", Duration::YMDHMS { year: 0, month: 0, day: 0, hour: 12, minute: 0, second: 0, millisecond: 0 }).unwrap();
    test_date_part(&parse_duration, "PT8760H", "", Duration::YMDHMS { year: 0, month: 0, day: 0, hour: 365*24, minute: 0, second: 0, millisecond: 0 }).unwrap();
}

#[rustfmt::skip]
#[test]
fn duration_multi_digit_minute() {
    test_date_part(&parse_duration, "PT15M", "", Duration::YMDHMS { year: 0, month: 0, day: 0, hour: 0, minute: 15, second: 0, millisecond: 0 }).unwrap();
    test_date_part(&parse_duration, "PT600M", "", Duration::YMDHMS { year: 0, month: 0, day: 0, hour: 0, minute: 600, second: 0, millisecond: 0 }).unwrap();
}

#[rustfmt::skip]
#[test]
fn duration_multi_digit_second() {
    test_date_part(&parse_duration, "PT16S", "", Duration::YMDHMS { year: 0, month: 0, day: 0, hour: 0, minute: 0, second: 16, millisecond: 0 }).unwrap();
    test_date_part(&parse_duration, "PT900S", "", Duration::YMDHMS { year: 0, month: 0, day: 0, hour: 0, minute: 0, second: 900, millisecond: 0 }).unwrap();
}

#[rustfmt::skip]
#[test]
fn duration_multi_digit_day() {
    test_date_part(&parse_duration, "P365D", "", Duration::YMDHMS { year: 0, month: 0, day: 365, hour: 0, minute: 0, second: 0, millisecond: 0 }).unwrap();
    test_date_part(&parse_duration, "P36500D", "", Duration::YMDHMS { year: 0, month: 0, day: 36500, hour: 0, minute: 0, second: 0, millisecond: 0 }).unwrap();
}

// #[test]
// fn corner_cases() {
//    // how to deal with left overs?
//    assert!(parse_datetime((b"2015-06-26T22:57:09Z00:00").is_done());
//    assert!(date("2015-06-26T22:57:09Z00:00").is_err());
//
//    assert!(parse_datetime((b"2015-06-26T22:57:09Z+00:00").is_done());
//    assert!(datetime("2015-06-26T22:57:09Z+00:00").is_err());
//    assert!(parse_datetime((b"2001-W05-6T04:05:06.123455Z").is_err());
//    assert!(parse_datetime((b"2015-06-26TZ").is_err());
// }

#[test]
#[ignore]
/// a few things we don't parse correctly yet
/// see <https://ijmacd.github.io/rfc3339-iso8601/>
fn iso8601_vs_rfc3339() {
    // "+002023-02-18".parse::<Date>().unwrap();  // six digit years
    // "+002023-02".parse::<Date>().unwrap(); // six digit years
    // "+002023".parse::<Date>().unwrap(); // six digit years
    // "+2023".parse::<Date>().unwrap(); // missing months etc
    // "2023-02-18 18:29:24+01:00".parse::<DateTime>().unwrap();
    // "2023-02-18_17:29:49.278Z".parse::<DateTime>().unwrap();
    // "2021-208T22:20:32.332320+08".parse::<DateTime>().unwrap();
}

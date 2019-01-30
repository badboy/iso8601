use super::{
    day, hour, iso_week_date, minute, month, ordinal_date, parse_date, parse_datetime, parse_time,
    second, year,
};

use nom::types::CompleteByteSlice;

#[test]
fn test_year() {
    assert_eq!(Ok((CompleteByteSlice(&[][..]),   2015)), year(CompleteByteSlice(b"2015")));
    assert_eq!(Ok((CompleteByteSlice(&[][..]),  -0333)), year(CompleteByteSlice(b"-0333")));
    assert_eq!(Ok((CompleteByteSlice(&b"-"[..]), 2015)), year(CompleteByteSlice(b"2015-")));
    assert!(year(CompleteByteSlice(b"abcd")).is_err());
    assert!(year(CompleteByteSlice(b"2a03")).is_err());
}

#[test]
fn test_month() {
    assert_eq!(Ok((CompleteByteSlice(&[][..]), 1)),    month(CompleteByteSlice(b"01")));
    assert_eq!(Ok((CompleteByteSlice(&[][..]), 6)),    month(CompleteByteSlice(b"06")));
    assert_eq!(Ok((CompleteByteSlice(&[][..]), 12)),   month(CompleteByteSlice(b"12")));
    assert_eq!(Ok((CompleteByteSlice(&b"-"[..]), 12)), month(CompleteByteSlice(b"12-")));

    assert!(month(CompleteByteSlice(b"13")).is_err());
    assert!(month(CompleteByteSlice(b"00")).is_err());
}

#[test]
fn test_day() {
    assert_eq!(Ok((CompleteByteSlice(&[][..]), 1)),    day(CompleteByteSlice(b"01")));
    assert_eq!(Ok((CompleteByteSlice(&[][..]), 12)),   day(CompleteByteSlice(b"12")));
    assert_eq!(Ok((CompleteByteSlice(&[][..]), 20)),   day(CompleteByteSlice(b"20")));
    assert_eq!(Ok((CompleteByteSlice(&[][..]), 28)),   day(CompleteByteSlice(b"28")));
    assert_eq!(Ok((CompleteByteSlice(&[][..]), 30)),   day(CompleteByteSlice(b"30")));
    assert_eq!(Ok((CompleteByteSlice(&[][..]), 31)),   day(CompleteByteSlice(b"31")));
    assert_eq!(Ok((CompleteByteSlice(&b"-"[..]), 31)), day(CompleteByteSlice(b"31-")));

    assert!(day(CompleteByteSlice(b"00")).is_err());
    assert!(day(CompleteByteSlice(b"32")).is_err());
}

#[test]
fn test_hour() {
    assert_eq!(Ok((CompleteByteSlice(&[][..]), 0)),  hour(CompleteByteSlice(b"00")));
    assert_eq!(Ok((CompleteByteSlice(&[][..]), 1)),  hour(CompleteByteSlice(b"01")));
    assert_eq!(Ok((CompleteByteSlice(&[][..]), 6)),  hour(CompleteByteSlice(b"06")));
    assert_eq!(Ok((CompleteByteSlice(&[][..]), 12)), hour(CompleteByteSlice(b"12")));
    assert_eq!(Ok((CompleteByteSlice(&[][..]), 13)), hour(CompleteByteSlice(b"13")));
    assert_eq!(Ok((CompleteByteSlice(&[][..]), 20)), hour(CompleteByteSlice(b"20")));
    assert_eq!(Ok((CompleteByteSlice(&[][..]), 24)), hour(CompleteByteSlice(b"24")));

    assert!(hour(CompleteByteSlice(b"25")).is_err());
    assert!(hour(CompleteByteSlice(b"30")).is_err());
    assert!(hour(CompleteByteSlice(b"ab")).is_err());
}

#[test]
fn test_minute() {
    assert_eq!(Ok((CompleteByteSlice(&[][..]), 0)),  minute(CompleteByteSlice(b"00")));
    assert_eq!(Ok((CompleteByteSlice(&[][..]), 1)),  minute(CompleteByteSlice(b"01")));
    assert_eq!(Ok((CompleteByteSlice(&[][..]), 30)), minute(CompleteByteSlice(b"30")));
    assert_eq!(Ok((CompleteByteSlice(&[][..]), 59)), minute(CompleteByteSlice(b"59")));

    assert!(minute(CompleteByteSlice(b"60")).is_err());
    assert!(minute(CompleteByteSlice(b"61")).is_err());
    assert!(minute(CompleteByteSlice(b"ab")).is_err());
}

#[test]
fn test_second() {
    assert_eq!(Ok((CompleteByteSlice(&[][..]), 0)),  second(CompleteByteSlice(b"00")));
    assert_eq!(Ok((CompleteByteSlice(&[][..]), 1)),  second(CompleteByteSlice(b"01")));
    assert_eq!(Ok((CompleteByteSlice(&[][..]), 30)), second(CompleteByteSlice(b"30")));
    assert_eq!(Ok((CompleteByteSlice(&[][..]), 59)), second(CompleteByteSlice(b"59")));
    assert_eq!(Ok((CompleteByteSlice(&[][..]), 60)), second(CompleteByteSlice(b"60")));

    assert!(second(CompleteByteSlice(b"61")).is_err());
    assert!(second(CompleteByteSlice(b"ab")).is_err());
}

#[test]
fn test_date() {
    assert!(parse_date(CompleteByteSlice(b"201")).is_err());
    assert!(parse_date(CompleteByteSlice(b"2015p00p00")).is_err());
    assert!(parse_date(CompleteByteSlice(b"pppp")).is_err());
}

#[test]
fn test_time() {
    assert!(parse_time(CompleteByteSlice(b"20:")).is_err());
    assert!(parse_time(CompleteByteSlice(b"20p42p16")).is_err());
    assert!(parse_time(CompleteByteSlice(b"pppp")).is_err());
}

#[test]
fn test_time_with_timezone() {
    assert!(parse_time(CompleteByteSlice(b"20:")).is_err());
    assert!(parse_time(CompleteByteSlice(b"20p42p16")).is_err());
    assert!(parse_time(CompleteByteSlice(b"pppp")).is_err());
}

#[test]
fn test_iso_week_date() {
    assert!(iso_week_date(CompleteByteSlice(b"2015-W06-8")).is_err());
    assert!(iso_week_date(CompleteByteSlice(b"2015-W068")).is_err());
    assert!(iso_week_date(CompleteByteSlice(b"2015-W06-0")).is_err());
    assert!(iso_week_date(CompleteByteSlice(b"2015-W00-2")).is_err());
    assert!(iso_week_date(CompleteByteSlice(b"2015-W54-2")).is_err());
    assert!(iso_week_date(CompleteByteSlice(b"2015-W542")).is_err());
}

#[test]
fn test_ordinal_date() {
    // not valid here either
    assert!(ordinal_date(CompleteByteSlice(b"2015-400")).is_err());
}

#[test]
fn format_equivalence() {
    assert_eq!(parse_datetime(CompleteByteSlice(b"2001-02-03T04:05:06+07:00")),  parse_datetime(CompleteByteSlice(b"20010203T040506+0700")));
    assert_eq!(parse_datetime(CompleteByteSlice(b"2001-02-03T04:05:06+07:00")),  parse_datetime(CompleteByteSlice(b"20010203T04:05:06+0700")));
    assert_eq!(parse_datetime(CompleteByteSlice(b"2001-02-03T04:05:00+07:00")),  parse_datetime(CompleteByteSlice(b"20010203T0405+0700")));
    assert_eq!(parse_datetime(CompleteByteSlice(b"20010203T0405+0700")),         parse_datetime(CompleteByteSlice(b"2001-02-03T0405+0700")));
    assert_eq!(parse_datetime(CompleteByteSlice(b"20010203T040506+0700")),       parse_datetime(CompleteByteSlice(b"2001-02-03T040506+0700")));
    assert_eq!(parse_datetime(CompleteByteSlice(b"20010203T040506+0000")),       parse_datetime(CompleteByteSlice(b"20010203T040506Z")));
    assert_eq!(parse_datetime(CompleteByteSlice(b"2015W056T04:05:06+07:00")),    parse_datetime(CompleteByteSlice(b"2015-W05-6T04:05:06+07:00")));
}

#[test]
fn test_datetime_error() {
    let test_datetimes = vec!["ppp", "dumd-di-duTmd:iu:m"];

    for iso_string in test_datetimes {
        let res = parse_datetime(CompleteByteSlice(iso_string.as_bytes()));
        assert!(res.is_err());
    }
}

#[test]
fn disallows_notallowed() {
    assert!(parse_time(CompleteByteSlice(b"30:90:90")).is_err());
    assert!(parse_date(CompleteByteSlice(b"0000-20-40")).is_err());
    assert!(parse_datetime(CompleteByteSlice(b"2001-w05-6t04:05:06.123z")).is_err());
}

// #[test]
// fn corner_cases() {
//    // how to deal with left overs?
//    assert!(parse_datetime(CompleteByteSlice(b"2015-06-26T22:57:09Z00:00").is_done());
//    assert!(date("2015-06-26T22:57:09Z00:00").is_err());
//
//    assert!(parse_datetime(CompleteByteSlice(b"2015-06-26T22:57:09Z+00:00").is_done());
//    assert!(datetime("2015-06-26T22:57:09Z+00:00").is_err());
//    assert!(parse_datetime(CompleteByteSlice(b"2001-W05-6T04:05:06.123455Z").is_err());
//    assert!(parse_datetime(CompleteByteSlice(b"2015-06-26TZ").is_err());
// }

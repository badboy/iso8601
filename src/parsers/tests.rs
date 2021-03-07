use super::*;

#[test]
fn test_date_year() {
    assert_eq!(Ok((&[][..], 2015)), date_year(b"2015"));
    assert_eq!(Ok((&[][..], -0333)), date_year(b"-0333"));
    assert_eq!(Ok((&b"-"[..], 2015)), date_year(b"2015-"));
    assert!(date_year(b"abcd").is_err());
    assert!(date_year(b"2a03").is_err());
}

#[test]
fn test_date_month() {
    assert_eq!(Ok((&[][..], 1)), date_month(b"01"));
    assert_eq!(Ok((&[][..], 6)), date_month(b"06"));
    assert_eq!(Ok((&[][..], 12)), date_month(b"12"));
    assert_eq!(Ok((&b"-"[..], 12)), date_month(b"12-"));

    assert!(date_month(b"13").is_err());
    assert!(date_month(b"00").is_err());
}

#[test]
fn test_date_day() {
    assert_eq!(Ok((&[][..], 1)), date_day(b"01"));
    assert_eq!(Ok((&[][..], 12)), date_day(b"12"));
    assert_eq!(Ok((&[][..], 20)), date_day(b"20"));
    assert_eq!(Ok((&[][..], 28)), date_day(b"28"));
    assert_eq!(Ok((&[][..], 30)), date_day(b"30"));
    assert_eq!(Ok((&[][..], 31)), date_day(b"31"));
    assert_eq!(Ok((&b"-"[..], 31)), date_day(b"31-"));

    assert!(date_day(b"00").is_err());
    assert!(date_day(b"32").is_err());
}

#[test]
fn test_time_hour() {
    assert_eq!(Ok((&[][..], 0)), time_hour(b"00"));
    assert_eq!(Ok((&[][..], 1)), time_hour(b"01"));
    assert_eq!(Ok((&[][..], 6)), time_hour(b"06"));
    assert_eq!(Ok((&[][..], 12)), time_hour(b"12"));
    assert_eq!(Ok((&[][..], 13)), time_hour(b"13"));
    assert_eq!(Ok((&[][..], 20)), time_hour(b"20"));
    assert_eq!(Ok((&[][..], 24)), time_hour(b"24"));

    assert!(time_hour(b"25").is_err());
    assert!(time_hour(b"30").is_err());
    assert!(time_hour(b"ab").is_err());
}

#[test]
fn test_time_minute() {
    assert_eq!(Ok((&[][..], 0)), time_minute(b"00"));
    assert_eq!(Ok((&[][..], 1)), time_minute(b"01"));
    assert_eq!(Ok((&[][..], 30)), time_minute(b"30"));
    assert_eq!(Ok((&[][..], 59)), time_minute(b"59"));

    assert!(time_minute(b"60").is_err());
    assert!(time_minute(b"61").is_err());
    assert!(time_minute(b"ab").is_err());
}

#[test]
fn test_time_second() {
    assert_eq!(Ok((&[][..], 0)), time_second(b"00"));
    assert_eq!(Ok((&[][..], 1)), time_second(b"01"));
    assert_eq!(Ok((&[][..], 30)), time_second(b"30"));
    assert_eq!(Ok((&[][..], 59)), time_second(b"59"));
    assert_eq!(Ok((&[][..], 60)), time_second(b"60"));

    assert!(time_second(b"61").is_err());
    assert!(time_second(b"ab").is_err());
}

#[test]
fn test_date() {
    assert!(parse_date(b"201").is_err());
    assert!(parse_date(b"2015p00p00").is_err());
    assert!(parse_date(b"pppp").is_err());
}

#[test]
fn test_time() {
    assert!(parse_time(b"20:").is_err());
    assert!(parse_time(b"20p42p16").is_err());
    assert!(parse_time(b"pppp").is_err());
}

#[test]
fn test_time_with_timezone() {
    assert!(parse_time(b"20:").is_err());
    assert!(parse_time(b"20p42p16").is_err());
    assert!(parse_time(b"pppp").is_err());
}

#[test]
fn test_date_iso_week_date() {
    assert!(date_iso_week(b"2015-W06-8").is_err());
    assert!(date_iso_week(b"2015-W068").is_err());
    assert!(date_iso_week(b"2015-W06-0").is_err());
    assert!(date_iso_week(b"2015-W00-2").is_err());
    assert!(date_iso_week(b"2015-W54-2").is_err());
    assert!(date_iso_week(b"2015-W542").is_err());
}

#[test]
fn test_date_ordinal_date() {
    // not valid here either
    assert!(date_ordinal(b"2015-400").is_err());
}

#[test]
fn format_equivalence() {
    assert_eq!(
        parse_datetime(b"2001-02-03T04:05:06+07:00"),
        parse_datetime(b"20010203T040506+0700")
    );
    assert_eq!(
        parse_datetime(b"2001-02-03T04:05:06+07:00"),
        parse_datetime(b"20010203T04:05:06+0700")
    );
    assert_eq!(
        parse_datetime(b"2001-02-03T04:05:00+07:00"),
        parse_datetime(b"20010203T0405+0700")
    );
    assert_eq!(
        parse_datetime(b"20010203T0405+0700"),
        parse_datetime(b"2001-02-03T0405+0700")
    );
    assert_eq!(
        parse_datetime(b"20010203T040506+0700"),
        parse_datetime(b"2001-02-03T040506+0700")
    );
    assert_eq!(
        parse_datetime(b"20010203T040506+0000"),
        parse_datetime(b"20010203T040506Z")
    );
    assert_eq!(
        parse_datetime(b"2015W056T04:05:06+07:00"),
        parse_datetime(b"2015-W05-6T04:05:06+07:00")
    );
}

#[test]
fn test_datetime_error() {
    let test_datetimes = vec!["ppp", "dumd-di-duTmd:iu:m"];

    for iso_string in test_datetimes {
        let res = parse_datetime(iso_string.as_bytes());
        assert!(res.is_err());
    }
}

#[test]
fn disallows_notallowed() {
    assert!(parse_time(b"30:90:90").is_err());
    assert!(parse_date(b"0000-20-40").is_err());
    assert!(parse_datetime(b"2001-w05-6t04:05:06.123z").is_err());
}

#[test]
fn test_duration_year() {
    assert_eq!(Ok((&[][..], 2019)), duration_year(b"2019"));
    assert_eq!(Ok((&[][..], 0)), duration_year(b"0"));
    assert_eq!(Ok((&[][..], 10000)), duration_year(b"10000"));
    assert!(duration_year(b"abcd").is_err());
    assert!(duration_year(b"-1").is_err());
}

#[test]
fn test_duration_month() {
    assert_eq!(Ok((&[][..], 6)), duration_month(b"6"));
    assert_eq!(Ok((&[][..], 0)), duration_month(b"0"));
    assert_eq!(Ok((&[][..], 12)), duration_month(b"12"));
    assert!(duration_month(b"ab").is_err());
    assert!(duration_month(b"-1").is_err());
    assert!(duration_month(b"13").is_err());
}

#[test]
fn test_duration_week() {
    assert_eq!(Ok((&[][..], 26)), duration_week(b"26"));
    assert_eq!(Ok((&[][..], 0)), duration_week(b"0"));
    assert_eq!(Ok((&[][..], 52)), duration_week(b"52"));
    assert!(duration_week(b"ab").is_err());
    assert!(duration_week(b"-1").is_err());
    assert!(duration_week(b"53").is_err());
}

#[test]
fn test_duration_day() {
    assert_eq!(Ok((&[][..], 16)), duration_day(b"16"));
    assert_eq!(Ok((&[][..], 0)), duration_day(b"0"));
    assert_eq!(Ok((&[][..], 31)), duration_day(b"31"));
    assert!(duration_day(b"ab").is_err());
    assert!(duration_day(b"-1").is_err());
    assert!(duration_day(b"32").is_err());
}

#[test]
fn test_duration_hour() {
    assert_eq!(Ok((&[][..], 12)), duration_hour(b"12"));
    assert_eq!(Ok((&[][..], 0)), duration_hour(b"0"));
    assert_eq!(Ok((&[][..], 24)), duration_hour(b"24"));
    assert!(duration_hour(b"ab").is_err());
    assert!(duration_hour(b"-1").is_err());
    assert!(duration_hour(b"25").is_err());
}

#[test]
fn test_duration_minute() {
    assert_eq!(Ok((&[][..], 30)), duration_minute(b"30"));
    assert_eq!(Ok((&[][..], 0)), duration_minute(b"0"));
    assert_eq!(Ok((&[][..], 60)), duration_minute(b"60"));
    assert!(duration_minute(b"ab").is_err());
    assert!(duration_minute(b"-1").is_err());
    assert!(duration_minute(b"61").is_err());
}

#[test]
fn test_duration_second_and_millisecond() {
    assert_eq!(
        Ok((&[][..], (30, 0))),
        duration_second_and_millisecond(b"30")
    );
    assert_eq!(Ok((&[][..], (0, 0))), duration_second_and_millisecond(b"0"));
    assert_eq!(
        Ok((&[][..], (60, 0))),
        duration_second_and_millisecond(b"60")
    );
    assert_eq!(
        Ok((&[][..], (1, 230))),
        duration_second_and_millisecond(b"1,23")
    );
    assert_eq!(
        Ok((&[][..], (1, 230))),
        duration_second_and_millisecond(b"1.23")
    );
    assert!(duration_second_and_millisecond(b"ab").is_err());
    assert!(duration_second_and_millisecond(b"-1").is_err());
    assert!(duration_second_and_millisecond(b"61").is_err());
}

#[test]
fn test_duration_time() {
    assert_eq!(Ok((&[][..], (1, 2, 3, 0))), duration_time(b"1H2M3S"));
    assert_eq!(Ok((&[][..], (1, 0, 3, 0))), duration_time(b"1H3S"));
    assert_eq!(Ok((&[][..], (0, 2, 0, 0))), duration_time(b"2M"));
    assert_eq!(Ok((&[][..], (1, 2, 3, 400))), duration_time(b"1H2M3,4S"));
    assert_eq!(Ok((&[][..], (1, 2, 3, 400))), duration_time(b"1H2M3.4S"));
    assert_eq!(Ok((&[][..], (0, 0, 0, 123))), duration_time(b"0,123S"));
    assert_eq!(Ok((&[][..], (0, 0, 0, 123))), duration_time(b"0.123S"));
}

#[test]
fn test_duration_ymdhms_error() {
    assert!(duration_ymdhms(b"").is_err());
    assert!(duration_ymdhms(b"P").is_err()); // empty duration is not 0 seconds
    assert!(duration_ymdhms(b"1Y2M3DT4H5M6S").is_err()); // missing P at start
    assert!(duration_ymdhms(b"T4H5M6S").is_err()); // missing P, required even if no YMD part
}

#[test]
fn test_duration_weeks_error() {
    assert!(duration_weeks(b"").is_err());
    assert!(duration_weeks(b"P").is_err()); // empty duration is not 0 seconds
    assert!(duration_weeks(b"P1").is_err()); // missing W after number
    assert!(duration_weeks(b"PW").is_err()); // missing number
}

#[test]
fn test_duration_datetime_error() {
    assert!(duration_datetime(b"").is_err());
    assert!(duration_datetime(b"P").is_err()); // empty duration is not 0 seconds
    assert!(duration_datetime(b"0001-02-03T04:05:06").is_err()); // missing P at start
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

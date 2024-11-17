use super::*;

#[test]
fn test_date_year() {
    assert_eq!(date_year(&mut "2015".as_bstr()).unwrap(), 2015);
    assert_eq!(date_year(&mut "+2015".as_bstr()).unwrap(), 2015);
    assert_eq!(date_year(&mut "-333".as_bstr()).unwrap(), -333);
    assert_eq!(date_year(&mut "2015-".as_bstr()).unwrap(), 2015);
    assert!(date_year(&mut Stream::new(b"abcd")).is_err());
    assert!(date_year(&mut Stream::new(b"2a03")).is_err());
}

#[test]
fn test_date_month() {
    assert_eq!(date_month(&mut "01".as_bstr()).unwrap(), 1);
    assert_eq!(date_month(&mut "06".as_bstr()).unwrap(), 6);
    assert_eq!(date_month(&mut "12".as_bstr()).unwrap(), 12);
    assert_eq!(date_month(&mut "12-".as_bstr()).unwrap(), 12);

    assert!(date_month(&mut Stream::new(b"13\n")).is_err());
    assert!(date_month(&mut Stream::new(b"00\n")).is_err());
}

#[test]
fn test_date_day() {
    assert_eq!(date_day(&mut "01".as_bstr()).unwrap(), 1);
    assert_eq!(date_day(&mut "12".as_bstr()).unwrap(), 12);
    assert_eq!(date_day(&mut "20".as_bstr()).unwrap(), 20);
    assert_eq!(date_day(&mut "28".as_bstr()).unwrap(), 28);
    assert_eq!(date_day(&mut "30".as_bstr()).unwrap(), 30);
    assert_eq!(date_day(&mut "31".as_bstr()).unwrap(), 31);
    assert_eq!(date_day(&mut "31-".as_bstr()).unwrap(), 31);

    assert!(date_day(&mut Stream::new(b"00")).is_err());
    assert!(date_day(&mut Stream::new(b"32")).is_err());
}

#[test]
fn test_time_hour() {
    assert_eq!(time_hour(&mut "00".as_bstr()).unwrap(), 0);
    assert_eq!(time_hour(&mut "01".as_bstr()).unwrap(), 1);
    assert_eq!(time_hour(&mut "06".as_bstr()).unwrap(), 6);
    assert_eq!(time_hour(&mut "12".as_bstr()).unwrap(), 12);
    assert_eq!(time_hour(&mut "13".as_bstr()).unwrap(), 13);
    assert_eq!(time_hour(&mut "20".as_bstr()).unwrap(), 20);
    assert_eq!(time_hour(&mut "24".as_bstr()).unwrap(), 24);

    assert!(time_hour(&mut Stream::new(b"25")).is_err());
    assert!(time_hour(&mut Stream::new(b"30")).is_err());
    assert!(time_hour(&mut Stream::new(b"ab")).is_err());
}

#[test]
fn test_time_minute() {
    assert_eq!(time_minute(&mut "00".as_bstr()).unwrap(), 0);
    assert_eq!(time_minute(&mut "01".as_bstr()).unwrap(), 1);
    assert_eq!(time_minute(&mut "30".as_bstr()).unwrap(), 30);
    assert_eq!(time_minute(&mut "59".as_bstr()).unwrap(), 59);

    assert!(time_minute(&mut Stream::new(b"60")).is_err());
    assert!(time_minute(&mut Stream::new(b"61")).is_err());
    assert!(time_minute(&mut Stream::new(b"ab")).is_err());
}

#[test]
fn test_time_second() {
    assert_eq!(time_second(&mut "00".as_bstr()).unwrap(), 0);
    assert_eq!(time_second(&mut "01".as_bstr()).unwrap(), 1);
    assert_eq!(time_second(&mut "30".as_bstr()).unwrap(), 30);
    assert_eq!(time_second(&mut "59".as_bstr()).unwrap(), 59);
    assert_eq!(time_second(&mut "60".as_bstr()).unwrap(), 60);

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
    assert_eq!(duration_year(&mut "2019Y".as_bstr()).unwrap(), 2019);
    assert_eq!(duration_year(&mut "0Y".as_bstr()).unwrap(), 0);
    assert_eq!(duration_year(&mut "10000Y".as_bstr()).unwrap(), 10000);
    assert!(duration_year(&mut Stream::new(b"abcd")).is_err());
    assert!(duration_year(&mut Stream::new(b"-1")).is_err());
}

#[test]
fn test_duration_month() {
    assert_eq!(duration_month(&mut "6M".as_bstr()).unwrap(), 6);
    assert_eq!(duration_month(&mut "0M".as_bstr()).unwrap(), 0);
    assert_eq!(duration_month(&mut "12M".as_bstr()).unwrap(), 12);

    assert!(duration_month(&mut Stream::new(b"ab")).is_err());
    assert!(duration_month(&mut Stream::new(b"-1")).is_err());
    assert!(duration_month(&mut Stream::new(b"13")).is_err());
}

#[test]
fn test_duration_week() {
    assert_eq!(duration_week(&mut "26W".as_bstr()).unwrap(), 26);
    assert_eq!(duration_week(&mut "0W".as_bstr()).unwrap(), 0);
    assert_eq!(duration_week(&mut "52W".as_bstr()).unwrap(), 52);
    assert!(duration_week(&mut Stream::new(b"ab")).is_err());
    assert!(duration_week(&mut Stream::new(b"-1")).is_err());
    assert!(duration_week(&mut Stream::new(b"53")).is_err());
}

#[test]
fn test_duration_day() {
    assert_eq!(duration_day(&mut "16D".as_bstr()).unwrap(), 16);
    assert_eq!(duration_day(&mut "0D".as_bstr()).unwrap(), 0);
    assert_eq!(duration_day(&mut "31D".as_bstr()).unwrap(), 31);
    assert!(duration_day(&mut Stream::new(b"ab")).is_err());
    assert!(duration_day(&mut Stream::new(b"-1")).is_err());
    assert!(duration_day(&mut Stream::new(b"32")).is_err());
}

#[test]
fn test_duration_hour() {
    assert_eq!(duration_hour(&mut "12H".as_bstr()).unwrap(), 12);
    assert_eq!(duration_hour(&mut "0H".as_bstr()).unwrap(), 0);
    assert_eq!(duration_hour(&mut "24H".as_bstr()).unwrap(), 24);
    assert!(duration_hour(&mut Stream::new(b"ab")).is_err());
    assert!(duration_hour(&mut Stream::new(b"-1")).is_err());
    assert!(duration_hour(&mut Stream::new(b"25")).is_err());
}

#[test]
fn test_duration_minute() {
    assert_eq!(duration_minute(&mut "30M".as_bstr()).unwrap(), 30);
    assert_eq!(duration_minute(&mut "0M".as_bstr()).unwrap(), 0);
    assert_eq!(duration_minute(&mut "60M".as_bstr()).unwrap(), 60);
    assert!(duration_minute(&mut Stream::new(b"ab")).is_err());
    assert!(duration_minute(&mut Stream::new(b"-1")).is_err());
    assert!(duration_minute(&mut Stream::new(b"61")).is_err());
}

#[test]
fn test_duration_second_and_millisecond1() {
    assert_eq!(
        duration_second_and_millisecond(&mut "30S".as_bstr()).unwrap(),
        (30, 0)
    );
    assert_eq!(
        duration_second_and_millisecond(&mut "0S".as_bstr()).unwrap(),
        (0, 0)
    );
    assert_eq!(
        duration_second_and_millisecond(&mut "60S".as_bstr()).unwrap(),
        (60, 0)
    );
    assert_eq!(
        duration_second_and_millisecond(&mut "1,23S".as_bstr()).unwrap(),
        (1, 230)
    );
    assert_eq!(
        duration_second_and_millisecond(&mut "2.34S".as_bstr()).unwrap(),
        (2, 340)
    );
    assert!(duration_second_and_millisecond(&mut Stream::new(b"abS")).is_err());
    assert!(duration_second_and_millisecond(&mut Stream::new(b"-1S")).is_err());
}

#[test]
fn test_duration_time() {
    assert_eq!(
        duration_time(&mut "1H2M3S".as_bstr()).unwrap(),
        (1, 2, 3, 0)
    );
    assert_eq!(
        duration_time(&mut "10H12M30S".as_bstr()).unwrap(),
        (10, 12, 30, 0)
    );
    assert_eq!(duration_time(&mut "1H3S".as_bstr()).unwrap(), (1, 0, 3, 0));
    assert_eq!(duration_time(&mut "2M".as_bstr()).unwrap(), (0, 2, 0, 0));
    assert_eq!(
        duration_time(&mut "1H2M3,4S".as_bstr()).unwrap(),
        (1, 2, 3, 400)
    );
    assert_eq!(
        duration_time(&mut "1H2M3.4S".as_bstr()).unwrap(),
        (1, 2, 3, 400)
    );
    assert_eq!(
        duration_time(&mut "0,123S".as_bstr()).unwrap(),
        (0, 0, 0, 123)
    );
    assert_eq!(
        duration_time(&mut "0.123S".as_bstr()).unwrap(),
        (0, 0, 0, 123)
    );
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
    assert_eq!(parse_duration(&mut "PT30S".as_bstr()).unwrap(), Duration::YMDHMS { year: 0, month: 0, day: 0, hour: 0, minute: 0, second: 30, millisecond: 0 });
    assert_eq!(parse_duration(&mut "PT30.123S".as_bstr()).unwrap(), Duration::YMDHMS { year: 0, month: 0, day: 0, hour: 0, minute: 0, second: 30, millisecond: 123 });
    assert_eq!(parse_duration(&mut "P2021Y11M16DT23H26M59.123S".as_bstr()).unwrap(), Duration::YMDHMS { year: 2021, month: 11, day: 16, hour: 23, minute: 26, second: 59, millisecond: 123 });
}

#[rustfmt::skip]
#[test]
fn duration_roundtrip() {
    assert_eq!(parse_duration(&mut "P0W".as_bstr()).unwrap(), Duration::Weeks(0));
    assert_eq!(parse_duration(&mut "P2021Y11M16DT23H26M59.123S".as_bstr()).unwrap(), Duration::YMDHMS { year: 2021, month: 11, day: 16, hour: 23, minute: 26, second: 59, millisecond: 123 });
    assert_eq!(parse_duration(&mut "P2021Y11M16DT23H26M59S".as_bstr()).unwrap(), Duration::YMDHMS { year: 2021, month: 11, day: 16, hour: 23, minute: 26, second: 59, millisecond: 0 });
    assert_eq!(parse_duration(&mut "P2021Y11M16DT23H26M".as_bstr()).unwrap(), Duration::YMDHMS { year: 2021, month: 11, day: 16, hour: 23, minute: 26, second: 0, millisecond: 0 });
    assert_eq!(parse_duration(&mut "P2021Y11M16DT23H".as_bstr()).unwrap(), Duration::YMDHMS { year: 2021, month: 11, day: 16, hour: 23, minute: 0, second: 0, millisecond: 0 });
    assert_eq!(parse_duration(&mut "P2021Y11M16D".as_bstr()).unwrap(), Duration::YMDHMS { year: 2021, month: 11, day: 16, hour: 0, minute: 0, second: 0, millisecond: 0 });
    assert_eq!(parse_duration(&mut "P2021Y11M16DT1S".as_bstr()).unwrap(), Duration::YMDHMS { year: 2021, month: 11, day: 16, hour: 0, minute: 0, second: 1, millisecond: 0 });
    assert_eq!(parse_duration(&mut "P2021Y11M16DT0.471S".as_bstr()).unwrap(), Duration::YMDHMS { year: 2021, month: 11, day: 16, hour: 0, minute: 0, second: 0, millisecond: 471 });
    assert_eq!(parse_duration(&mut "P2021Y11M".as_bstr()).unwrap(), Duration::YMDHMS { year: 2021, month: 11, day: 0, hour: 0, minute: 0, second: 0, millisecond: 0 });
    assert_eq!(parse_duration(&mut "P11M".as_bstr()).unwrap(), Duration::YMDHMS { year: 0, month: 11, day: 0, hour: 0, minute: 0, second: 0, millisecond: 0 });
    assert_eq!(parse_duration(&mut "P16D".as_bstr()).unwrap(), Duration::YMDHMS { year: 0, month: 0, day: 16, hour: 0, minute: 0, second: 0, millisecond: 0 });
    assert_eq!(parse_duration(&mut "P0D".as_bstr()).unwrap(), Duration::YMDHMS { year: 0, month: 0, day: 0, hour: 0, minute: 0, second: 0, millisecond: 0 });
}

#[rustfmt::skip]
#[test]
fn duration_multi_digit_hour() {
    assert_eq!(parse_duration(&mut "PT12H".as_bstr()).unwrap(), Duration::YMDHMS { year: 0, month: 0, day: 0, hour: 12, minute: 0, second: 0, millisecond: 0 });
    assert_eq!(parse_duration(&mut "PT8760H".as_bstr()).unwrap(), Duration::YMDHMS { year: 0, month: 0, day: 0, hour: 365*24, minute: 0, second: 0, millisecond: 0 });
}

#[rustfmt::skip]
#[test]
fn duration_multi_digit_minute() {
    assert_eq!(parse_duration(&mut "PT15M".as_bstr()).unwrap(), Duration::YMDHMS { year: 0, month: 0, day: 0, hour: 0, minute: 15, second: 0, millisecond: 0 });
    assert_eq!(parse_duration(&mut "PT600M".as_bstr()).unwrap(), Duration::YMDHMS { year: 0, month: 0, day: 0, hour: 0, minute: 600, second: 0, millisecond: 0 });
}

#[rustfmt::skip]
#[test]
fn duration_multi_digit_second() {
    assert_eq!(parse_duration(&mut "PT16S".as_bstr()).unwrap(), Duration::YMDHMS { year: 0, month: 0, day: 0, hour: 0, minute: 0, second: 16, millisecond: 0 });
    assert_eq!(parse_duration(&mut "PT900S".as_bstr()).unwrap(), Duration::YMDHMS { year: 0, month: 0, day: 0, hour: 0, minute: 0, second: 900, millisecond: 0 });
}

#[rustfmt::skip]
#[test]
fn duration_multi_digit_day() {
    assert_eq!(parse_duration(&mut "P365D".as_bstr()).unwrap(), Duration::YMDHMS { year: 0, month: 0, day: 365, hour: 0, minute: 0, second: 0, millisecond: 0 });
    assert_eq!(parse_duration(&mut "P36500D".as_bstr()).unwrap(), Duration::YMDHMS { year: 0, month: 0, day: 36500, hour: 0, minute: 0, second: 0, millisecond: 0 }); 
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

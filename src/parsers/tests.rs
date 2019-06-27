use super::{
    day,
    month,
    year,

    hour,
    minute,
    second,

    iso_week_date,
    ordinal_date,

    parse_date,
    parse_time,
    parse_datetime,
};

#[test]
fn test_year() {
    assert_eq!(
        Ok(((&[][..]), 2015)),
        year(b"2015")
    );
    assert_eq!(
        Ok(((&[][..]), -0333)),
        year(b"-0333")
    );
    assert_eq!(
        Ok(((&b"-"[..]), 2015)),
        year(b"2015-")
    );
    assert!(year(b"abcd").is_err());
    assert!(year(b"2a03").is_err());
}

#[test]
fn test_month() {
    assert_eq!(
        Ok(((&[][..]), 1)),
        month(b"01")
    );
    assert_eq!(
        Ok(((&[][..]), 6)),
        month(b"06")
    );
    assert_eq!(
        Ok(((&[][..]), 12)),
        month(b"12")
    );
    assert_eq!(
        Ok(((&b"-"[..]), 12)),
        month(b"12-")
    );

    assert!(month(b"13").is_err());
    assert!(month(b"00").is_err());
}

#[test]
fn test_day() {
    assert_eq!(
        Ok(((&[][..]), 1)),
        day(b"01")
    );
    assert_eq!(
        Ok(((&[][..]), 12)),
        day(b"12")
    );
    assert_eq!(
        Ok(((&[][..]), 20)),
        day(b"20")
    );
    assert_eq!(
        Ok(((&[][..]), 28)),
        day(b"28")
    );
    assert_eq!(
        Ok(((&[][..]), 30)),
        day(b"30")
    );
    assert_eq!(
        Ok(((&[][..]), 31)),
        day(b"31")
    );
    assert_eq!(
        Ok(((&b"-"[..]), 31)),
        day(b"31-")
    );

    assert!(day(b"00").is_err());
    assert!(day(b"32").is_err());
}

#[test]
fn test_hour() {
    assert_eq!(
        Ok((&[][..], 0)),
        hour(b"00")
    );
    assert_eq!(
        Ok((&[][..], 1)),
        hour(b"01")
    );
    assert_eq!(
        Ok((&[][..], 6)),
        hour(b"06")
    );
    assert_eq!(
        Ok((&[][..], 12)),
        hour(b"12")
    );
    assert_eq!(
        Ok((&[][..], 13)),
        hour(b"13")
    );
    assert_eq!(
        Ok((&[][..], 20)),
        hour(b"20")
    );
    assert_eq!(
        Ok((&[][..], 24)),
        hour(b"24")
    );

    assert!(hour(b"25").is_err());
    assert!(hour(b"30").is_err());
    assert!(hour(b"ab").is_err());
}

#[test]
fn test_minute() {
    assert_eq!(
        Ok(((&[][..]), 0)),
        minute(b"00")
    );
    assert_eq!(
        Ok(((&[][..]), 1)),
        minute(b"01")
    );
    assert_eq!(
        Ok(((&[][..]), 30)),
        minute(b"30")
    );
    assert_eq!(
        Ok(((&[][..]), 59)),
        minute(b"59")
    );

    assert!(minute(b"60").is_err());
    assert!(minute(b"61").is_err());
    assert!(minute(b"ab").is_err());
}

#[test]
fn test_second() {
    assert_eq!(
        Ok((&[][..], 0)),
        second(b"00")
    );
    assert_eq!(
        Ok(((&[][..]), 1)),
        second(b"01")
    );
    assert_eq!(
        Ok(((&[][..]), 30)),
        second(b"30")
    );
    assert_eq!(
        Ok(((&[][..]), 59)),
        second(b"59")
    );
    assert_eq!(
        Ok(((&[][..]), 60)),
        second(b"60")
    );

    assert!(second(b"61").is_err());
    assert!(second(b"ab").is_err());
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
fn test_iso_week_date() {
    assert!(iso_week_date(b"2015-W06-8").is_err());
    assert!(iso_week_date(b"2015-W068").is_err());
    assert!(iso_week_date(b"2015-W06-0").is_err());
    assert!(iso_week_date(b"2015-W00-2").is_err());
    assert!(iso_week_date(b"2015-W54-2").is_err());
    assert!(iso_week_date(b"2015-W542").is_err());
}

#[test]
fn test_ordinal_date() {
    // not valid here either
    assert!(ordinal_date(b"2015-400").is_err());
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

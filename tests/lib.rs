use iso8601::*;

#[test]
fn test_date() {
    assert_eq!(
        Ok(Date::YMD {
            year: 2015,
            month: 6,
            day: 26,
        }),
        date("2015-06-26")
    );
    assert_eq!(
        Ok(Date::YMD {
            year: -333,
            month: 7,
            day: 11,
        }),
        date("-0333-07-11")
    );
}

#[test]
fn test_millisecond() {
    assert_eq!(
        Ok(Time {
            hour: 16,
            minute: 43,
            second: 0,
            millisecond: 100,
            tz_offset_hours: 0,
            tz_offset_minutes: 0
        }),
        time("16:43:00.1")
    );
    assert_eq!(
        Ok(Time {
            hour: 16,
            minute: 43,
            second: 0,
            millisecond: 120,
            tz_offset_hours: 0,
            tz_offset_minutes: 0
        }),
        time("16:43:00.12")
    );
    assert_eq!(
        Ok(Time {
            hour: 16,
            minute: 43,
            second: 0,
            millisecond: 123,
            tz_offset_hours: 0,
            tz_offset_minutes: 0
        }),
        time("16:43:00.123")
    );
    assert_eq!(
        Ok(Time {
            hour: 16,
            minute: 43,
            second: 0,
            millisecond: 432,
            tz_offset_hours: 0,
            tz_offset_minutes: 0
        }),
        time("16:43:00.4321")
    );
    assert_eq!(
        Ok(Time {
            hour: 16,
            minute: 43,
            second: 0,
            millisecond: 432,
            tz_offset_hours: 0,
            tz_offset_minutes: 0
        }),
        time("16:43.4321")
    );
    assert_eq!(
        Ok(Time {
            hour: 16,
            minute: 43,
            second: 11,
            millisecond: 432,
            tz_offset_hours: 0,
            tz_offset_minutes: 0
        }),
        time("16:43:11.4321")
    );

    assert_eq!(
        Ok(Time {
            hour: 16,
            minute: 43,
            second: 0,
            millisecond: 100,
            tz_offset_hours: 0,
            tz_offset_minutes: 0
        }),
        time("16:43:00,1")
    );

    assert_eq!(
        Ok(Time {
            hour: 04,
            minute: 05,
            second: 06,
            millisecond: 123,
            tz_offset_hours: 0,
            tz_offset_minutes: 0
        }),
        time("04:05:06.12345")
    );

    assert_eq!(
        Ok(DateTime {
            date: Date::Week {
                year: 2001,
                ww: 05,
                d: 6
            },
            time: Time {
                hour: 04,
                minute: 05,
                second: 06,
                millisecond: 123,
                tz_offset_hours: 0,
                tz_offset_minutes: 0
            }
        }),
        datetime("2001-W05-6T04:05:06.12345Z")
    );

    assert_eq!(
        Ok(Time {
            hour: 16,
            minute: 43,
            second: 16,
            millisecond: 123,
            tz_offset_hours: 0,
            tz_offset_minutes: 0
        }),
        time("16:43:16.123")
    );
    assert_eq!(
        Ok(Time {
            hour: 16,
            minute: 43,
            second: 16,
            millisecond: 123,
            tz_offset_hours: 0,
            tz_offset_minutes: 0
        }),
        time("16:43:16.123+00:00")
    );
    assert_eq!(
        Ok(Time {
            hour: 16,
            minute: 43,
            second: 16,
            millisecond: 123,
            tz_offset_hours: 0,
            tz_offset_minutes: 0
        }),
        time("16:43:16.123-00:00")
    );
    assert_eq!(
        Ok(Time {
            hour: 16,
            minute: 43,
            second: 16,
            millisecond: 123,
            tz_offset_hours: 5,
            tz_offset_minutes: 0
        }),
        time("16:43:16.123+05:00")
    );
}

#[test]
fn test_time() {
    assert_eq!(
        time("16:43:16"),
        Ok(Time {
            hour: 16,
            minute: 43,
            second: 16,
            millisecond: 0,
            tz_offset_hours: 0,
            tz_offset_minutes: 0,
        })
    );
    assert_eq!(
        time("16:43"),
        Ok(Time {
            hour: 16,
            minute: 43,
            second: 0,
            millisecond: 0,
            tz_offset_hours: 0,
            tz_offset_minutes: 0,
        })
    );

    assert!(time("20:").is_err());
    assert!(time("20p42p16").is_err());
    assert!(time("pppp").is_err());
}

#[test]
fn test_time_set_tz() {
    let original = Time {
        hour: 0,
        minute: 0,
        second: 0,
        millisecond: 0,
        tz_offset_hours: 0,
        tz_offset_minutes: 0,
    };
    let expected = Time {
        hour: 0,
        minute: 0,
        second: 0,
        millisecond: 0,
        tz_offset_hours: 2,
        tz_offset_minutes: 30,
    };

    assert_eq!(expected, original.set_tz((2, 30)));
}

#[test]
fn short_time1() {
    assert_eq!(
        time("1648"),
        Ok(Time {
            hour: 16,
            minute: 48,
            second: 0,
            millisecond: 0,
            tz_offset_hours: 0,
            tz_offset_minutes: 0,
        })
    );
}
#[test]
fn short_time2() {
    assert_eq!(
        time("16:48"),
        Ok(Time {
            hour: 16,
            minute: 48,
            second: 0,
            millisecond: 0,
            tz_offset_hours: 0,
            tz_offset_minutes: 0,
        })
    );
}
#[test]
fn short_time3() {
    assert_eq!(
        time("16:48Z"),
        Ok(Time {
            hour: 16,
            minute: 48,
            second: 0,
            millisecond: 0,
            tz_offset_hours: 0,
            tz_offset_minutes: 0,
        })
    );
}
#[test]
fn short_time4() {
    assert_eq!(
        time("164800"),
        Ok(Time {
            hour: 16,
            minute: 48,
            second: 0,
            millisecond: 0,
            tz_offset_hours: 0,
            tz_offset_minutes: 0,
        })
    );
}
#[test]
fn short_time5() {
    assert_eq!(
        time("164800.1"),
        Ok(Time {
            hour: 16,
            minute: 48,
            second: 0,
            millisecond: 100,
            tz_offset_hours: 0,
            tz_offset_minutes: 0,
        })
    );
}
#[test]
fn short_time6() {
    assert_eq!(
        time("164800.1Z"),
        Ok(Time {
            hour: 16,
            minute: 48,
            second: 0,
            millisecond: 100,
            tz_offset_hours: 0,
            tz_offset_minutes: 0,
        })
    );
}
#[test]
fn short_time7() {
    assert_eq!(
        time("16:48:00"),
        Ok(Time {
            hour: 16,
            minute: 48,
            second: 0,
            millisecond: 0,
            tz_offset_hours: 0,
            tz_offset_minutes: 0,
        })
    );
}

#[test]
fn short_twtz1() {
    assert_eq!(
        time("1648Z"),
        Ok(Time {
            hour: 16,
            minute: 48,
            second: 0,
            millisecond: 0,
            tz_offset_hours: 0,
            tz_offset_minutes: 0,
        })
    );
}
#[test]
fn short_twtz2() {
    assert_eq!(
        time("16:48Z"),
        Ok(Time {
            hour: 16,
            minute: 48,
            second: 0,
            millisecond: 0,
            tz_offset_hours: 0,
            tz_offset_minutes: 0,
        })
    );
}

#[test]
fn short_dtim1() {
    assert_eq!(
        datetime("20070831T1648"),
        Ok(DateTime {
            date: Date::YMD {
                year: 2007,
                month: 08,
                day: 31,
            },
            time: Time {
                hour: 16,
                minute: 48,
                second: 0,
                millisecond: 0,
                tz_offset_hours: 0,
                tz_offset_minutes: 0,
            }
        })
    );
}
#[test]
fn short_dtim2() {
    assert_eq!(
        datetime("20070831T1648Z"),
        Ok(DateTime {
            date: Date::YMD {
                year: 2007,
                month: 08,
                day: 31,
            },
            time: Time {
                hour: 16,
                minute: 48,
                second: 0,
                millisecond: 0,
                tz_offset_hours: 0,
                tz_offset_minutes: 0,
            },
        })
    );
}
#[test]
fn short_dtim3() {
    assert_eq!(
        datetime("2008-12-24T18:21Z"),
        Ok(DateTime {
            date: Date::YMD {
                year: 2008,
                month: 12,
                day: 24,
            },
            time: Time {
                hour: 18,
                minute: 21,
                second: 0,
                millisecond: 0,
                tz_offset_hours: 0,
                tz_offset_minutes: 0,
            },
        })
    );
}

#[test]
fn test_time_with_timezone() {
    assert_eq!(
        Ok(Time {
            hour: 16,
            minute: 43,
            second: 16,
            millisecond: 0,
            tz_offset_hours: 0,
            tz_offset_minutes: 0,
        }),
        time("16:43:16")
    );
    assert_eq!(
        Ok(Time {
            hour: 16,
            minute: 43,
            second: 16,
            millisecond: 0,
            tz_offset_hours: 0,
            tz_offset_minutes: 0,
        }),
        time("16:43:16Z")
    );
    assert_eq!(
        Ok(Time {
            hour: 16,
            minute: 43,
            second: 16,
            millisecond: 0,
            tz_offset_hours: 0,
            tz_offset_minutes: 0,
        }),
        time("16:43:16+00:00")
    );
    assert_eq!(
        Ok(Time {
            hour: 16,
            minute: 43,
            second: 16,
            millisecond: 0,
            tz_offset_hours: 0,
            tz_offset_minutes: 0,
        }),
        time("16:43:16-00:00")
    );
    assert_eq!(
        Ok(Time {
            hour: 16,
            minute: 43,
            second: 16,
            millisecond: 0,
            tz_offset_hours: 5,
            tz_offset_minutes: 0,
        }),
        time("16:43:16+05:00")
    );

    assert!(time("20:").is_err());
    assert!(time("20p42p16").is_err());
    assert!(time("pppp").is_err());
}

#[test]
fn test_iso_week_date() {
    assert_eq!(
        Ok(Date::Week {
            year: 2015,
            ww: 5,
            d: 7,
        }),
        date("2015-W05-7")
    );
    assert_eq!(
        Ok(Date::Week {
            year: 2015,
            ww: 6,
            d: 6,
        }),
        date("2015-W06-6")
    );
    assert_eq!(
        Ok(Date::Week {
            year: 2015,
            ww: 6,
            d: 6,
        }),
        date("2015-W066")
    );
    assert_eq!(
        Ok(Date::Week {
            year: 2015,
            ww: 6,
            d: 6,
        }),
        date("2015W066")
    );
    assert_eq!(
        Ok(Date::Week {
            year: 2015,
            ww: 43,
            d: 6,
        }),
        date("2015-W43-6")
    );

    assert!(date("2015-W06-8").is_err());
    assert!(date("2015-W068").is_err());
    assert!(date("2015-W06-0").is_err());
    assert!(date("2015-W00-2").is_err());
    assert!(date("2015-W54-2").is_err());
    assert!(date("2015-W542").is_err());
}

#[test]
fn test_ordinal_date() {
    assert_eq!(
        Ok(Date::Ordinal {
            year: 2015,
            ddd: 57,
        }),
        date("2015-057")
    );

    assert_eq!(
        Ok(Date::Ordinal {
            year: 2015,
            ddd: 358,
        }),
        date("2015-358")
    );
    assert_eq!(
        Ok(Date::Ordinal {
            year: 2015,
            ddd: 366,
        }),
        date("2015-366")
    );
    assert_eq!(Ok(Date::Ordinal { year: 2015, ddd: 1 }), date("2015-001"));

    // not valid here either
    assert!(date("2015-400").is_err());
}

#[test]
fn format_equivalence() {
    assert_eq!(
        datetime("2001-02-03T04:05:06+07:00"),
        datetime("20010203T040506+0700")
    );
    assert_eq!(
        datetime("2001-02-03T04:05:06+07:00"),
        datetime("20010203T04:05:06+0700")
    );
    assert_eq!(
        datetime("2001-02-03T04:05:00+07:00"),
        datetime("20010203T0405+0700")
    );
    assert_eq!(
        datetime("20010203T0405+0700"),
        datetime("2001-02-03T0405+0700")
    );
    assert_eq!(
        datetime("20010203T040506+0700"),
        datetime("2001-02-03T040506+0700")
    );
    assert_eq!(
        datetime("20010203T040506+0000"),
        datetime("20010203T040506Z")
    );
    assert_eq!(
        datetime("2015W056T04:05:06+07:00"),
        datetime("2015-W05-6T04:05:06+07:00")
    );
}

#[test]
fn test_datetime_correct() {
    assert_eq!(
        datetime("20060831T16:44+00:00"),
        Ok(DateTime {
            date: Date::YMD {
                year: 2006,
                month: 08,
                day: 31
            },
            time: Time {
                hour: 16,
                minute: 44,
                second: 0,
                millisecond: 0,
                tz_offset_hours: 0,
                tz_offset_minutes: 0
            }
        })
    );
    assert_eq!(
        datetime("2007-08-31T16:45+00:00"),
        Ok(DateTime {
            date: Date::YMD {
                year: 2007,
                month: 08,
                day: 31
            },
            time: Time {
                hour: 16,
                minute: 45,
                second: 0,
                millisecond: 0,
                tz_offset_hours: 0,
                tz_offset_minutes: 0
            }
        })
    );
    assert_eq!(
        datetime("20070831T1646+00:00"),
        Ok(DateTime {
            date: Date::YMD {
                year: 2007,
                month: 08,
                day: 31
            },
            time: Time {
                hour: 16,
                minute: 46,
                second: 0,
                millisecond: 0,
                tz_offset_hours: 0,
                tz_offset_minutes: 0
            }
        })
    );
    assert_eq!(
        datetime("20070831T1647+0000"),
        Ok(DateTime {
            date: Date::YMD {
                year: 2007,
                month: 08,
                day: 31
            },
            time: Time {
                hour: 16,
                minute: 47,
                second: 0,
                millisecond: 0,
                tz_offset_hours: 0,
                tz_offset_minutes: 0
            }
        })
    );
    assert_eq!(
        datetime("2009-02-01T09:00:22+05"),
        Ok(DateTime {
            date: Date::YMD {
                year: 2009,
                month: 02,
                day: 01
            },
            time: Time {
                hour: 9,
                minute: 0,
                second: 22,
                millisecond: 0,
                tz_offset_hours: 5,
                tz_offset_minutes: 0
            }
        })
    );
    assert_eq!(
        datetime("2010-01-01T12:00:00+01:00"),
        Ok(DateTime {
            date: Date::YMD {
                year: 2010,
                month: 1,
                day: 1
            },
            time: Time {
                hour: 12,
                minute: 0,
                second: 0,
                millisecond: 0,
                tz_offset_hours: 1,
                tz_offset_minutes: 0
            }
        })
    );
    assert_eq!(
        datetime("2011-06-30T18:30:00+02:00"),
        Ok(DateTime {
            date: Date::YMD {
                year: 2011,
                month: 06,
                day: 30
            },
            time: Time {
                hour: 18,
                minute: 30,
                second: 0,
                millisecond: 0,
                tz_offset_hours: 2,
                tz_offset_minutes: 0
            }
        })
    );
    assert_eq!(
        datetime("2015-06-29T23:07+02:00"),
        Ok(DateTime {
            date: Date::YMD {
                year: 2015,
                month: 06,
                day: 29
            },
            time: Time {
                hour: 23,
                minute: 07,
                second: 0,
                millisecond: 0,
                tz_offset_hours: 2,
                tz_offset_minutes: 0
            }
        })
    );
    assert_eq!(
        datetime("2015-06-26T16:43:16"),
        Ok(DateTime {
            date: Date::YMD {
                year: 2015,
                month: 06,
                day: 26
            },
            time: Time {
                hour: 16,
                minute: 43,
                second: 16,
                millisecond: 0,
                tz_offset_hours: 0,
                tz_offset_minutes: 0
            }
        })
    );
    assert_eq!(
        datetime("2015-06-26T16:43:16"),
        Ok(DateTime {
            date: Date::YMD {
                year: 2015,
                month: 06,
                day: 26
            },
            time: Time {
                hour: 16,
                minute: 43,
                second: 16,
                millisecond: 0,
                tz_offset_hours: 0,
                tz_offset_minutes: 0
            }
        })
    );
    assert_eq!(
        datetime("2015-W05-6T04:05:06+07:00"),
        Ok(DateTime {
            date: Date::Week {
                year: 2015,
                ww: 05,
                d: 6
            },
            time: Time {
                hour: 04,
                minute: 5,
                second: 6,
                millisecond: 0,
                tz_offset_hours: 7,
                tz_offset_minutes: 0
            }
        })
    );
    assert_eq!(
        datetime("2015W056T04:05:06+07:00"),
        Ok(DateTime {
            date: Date::Week {
                year: 2015,
                ww: 05,
                d: 6
            },
            time: Time {
                hour: 04,
                minute: 5,
                second: 6,
                millisecond: 0,
                tz_offset_hours: 7,
                tz_offset_minutes: 0
            }
        })
    );
    assert_eq!(
        datetime("2015-056T04:05:06+07:00"),
        Ok(DateTime {
            date: Date::Ordinal {
                year: 2015,
                ddd: 56
            },
            time: Time {
                hour: 04,
                minute: 5,
                second: 6,
                millisecond: 0,
                tz_offset_hours: 7,
                tz_offset_minutes: 0
            }
        })
    );
    assert_eq!(
        datetime("2015056T04:05:06+07:00"),
        Ok(DateTime {
            date: Date::Ordinal {
                year: 2015,
                ddd: 56
            },
            time: Time {
                hour: 04,
                minute: 5,
                second: 6,
                millisecond: 0,
                tz_offset_hours: 7,
                tz_offset_minutes: 0
            }
        })
    );
    assert_eq!(
        datetime("2015-297T16:30:48Z"),
        Ok(DateTime {
            date: Date::Ordinal {
                year: 2015,
                ddd: 297
            },
            time: Time {
                hour: 16,
                minute: 30,
                second: 48,
                millisecond: 0,
                tz_offset_hours: 0,
                tz_offset_minutes: 0
            }
        })
    );
    assert_eq!(
        datetime("2015-W43-6T16:30:48Z"),
        Ok(DateTime {
            date: Date::Week {
                year: 2015,
                ww: 43,
                d: 6
            },
            time: Time {
                hour: 16,
                minute: 30,
                second: 48,
                millisecond: 0,
                tz_offset_hours: 0,
                tz_offset_minutes: 0
            }
        })
    );
    assert_eq!(
        datetime("2001-W05-6T04:05:06.1234Z"),
        Ok(DateTime {
            date: Date::Week {
                year: 2001,
                ww: 05,
                d: 6
            },
            time: Time {
                hour: 04,
                minute: 05,
                second: 06,
                millisecond: 123,
                tz_offset_hours: 0,
                tz_offset_minutes: 0
            }
        })
    );
    assert_eq!(
        datetime("2001-W05-6T04:05:06.12345Z"),
        Ok(DateTime {
            date: Date::Week {
                year: 2001,
                ww: 05,
                d: 6
            },
            time: Time {
                hour: 04,
                minute: 05,
                second: 06,
                millisecond: 123,
                tz_offset_hours: 0,
                tz_offset_minutes: 0
            }
        })
    );
}

#[test]
fn issue12_regression_1() {
    let input = "164801.";

    assert_eq!(
        Ok(Time {
            hour: 16,
            minute: 48,
            second: 1,
            millisecond: 0,
            tz_offset_hours: 0,
            tz_offset_minutes: 0
        }),
        time(input)
    );
}

#[test]
fn issue12_regression_2() {
    let input = "04:05:06.1226001015632)*450";

    assert_eq!(
        Ok(Time {
            hour: 4,
            minute: 5,
            second: 6,
            millisecond: 122,
            tz_offset_hours: 0,
            tz_offset_minutes: 0
        }),
        time(input)
    );
}

#[test]
fn test_duration_ymdhms() {
    // full YMDHMS
    assert_eq!(
        Ok(Duration::YMDHMS {
            year: 1,
            month: 2,
            day: 3,
            hour: 4,
            minute: 5,
            second: 6,
            millisecond: 0,
        }),
        duration("P1Y2M3DT4H5M6S")
    );

    // full YMDHMS with milliseconds dot delimiter
    assert_eq!(
        Ok(Duration::YMDHMS {
            year: 1,
            month: 2,
            day: 3,
            hour: 4,
            minute: 5,
            second: 6,
            millisecond: 700,
        }),
        duration("P1Y2M3DT4H5M6.7S")
    );

    // full YMDHMS with milliseconds comma delimiter
    assert_eq!(
        Ok(Duration::YMDHMS {
            year: 1,
            month: 2,
            day: 3,
            hour: 4,
            minute: 5,
            second: 6,
            millisecond: 700,
        }),
        duration("P1Y2M3DT4H5M6,7S")
    );

    // subset YM-HM-
    assert_eq!(
        Ok(Duration::YMDHMS {
            year: 1,
            month: 2,
            day: 0,
            hour: 4,
            minute: 5,
            second: 0,
            millisecond: 0,
        }),
        duration("P1Y2MT4H5M")
    );

    // subset Y-----
    assert_eq!(
        Ok(Duration::YMDHMS {
            year: 1,
            month: 0,
            day: 0,
            hour: 0,
            minute: 0,
            second: 0,
            millisecond: 0,
        }),
        duration("P1Y")
    );

    // subset ---H--
    assert_eq!(
        Ok(Duration::YMDHMS {
            year: 0,
            month: 0,
            day: 0,
            hour: 4,
            minute: 0,
            second: 0,
            millisecond: 0,
        }),
        duration("PT4H")
    );

    // subset -----S with milliseconds dot delimiter
    assert_eq!(
        Ok(Duration::YMDHMS {
            year: 0,
            month: 0,
            day: 0,
            hour: 0,
            minute: 0,
            second: 6,
            millisecond: 700,
        }),
        duration("PT6.7S")
    );

    // subset -----S with milliseconds comma delimiter
    assert_eq!(
        Ok(Duration::YMDHMS {
            year: 0,
            month: 0,
            day: 0,
            hour: 0,
            minute: 0,
            second: 6,
            millisecond: 700,
        }),
        duration("PT6,700S")
    );

    // empty duration, using Y
    assert_eq!(
        Ok(Duration::YMDHMS {
            year: 0,
            month: 0,
            day: 0,
            hour: 0,
            minute: 0,
            second: 0,
            millisecond: 0,
        }),
        duration("P0Y")
    );

    // empty duration, using S
    assert_eq!(
        Ok(Duration::YMDHMS {
            year: 0,
            month: 0,
            day: 0,
            hour: 0,
            minute: 0,
            second: 0,
            millisecond: 0,
        }),
        duration("PT0S")
    );

    assert_eq!(
        Ok(Duration::YMDHMS {
            year: 0,
            month: 0,
            day: 0,
            hour: 0,
            minute: 42,
            second: 30,
            millisecond: 0,
        }),
        duration("PT42M30S")
    );
}

#[test]
fn test_duration_weeks() {
    assert_eq!(Ok(Duration::Weeks(0)), duration("P0W"));
    assert_eq!(Ok(Duration::Weeks(26)), duration("P26W"));
    assert_eq!(Ok(Duration::Weeks(52)), duration("P52W"));
}

#[test]
fn test_duration_datetime() {
    assert_eq!(
        Ok(Duration::DateTime(DateTime {
            date: Date::YMD {
                year: 1,
                month: 2,
                day: 3,
            },
            time: Time {
                hour: 4,
                minute: 5,
                second: 6,
                millisecond: 0,
                tz_offset_hours: 0,
                tz_offset_minutes: 0,
            }
        })),
        duration("P0001-02-03T04:05:06")
    );

    assert_eq!(
        Ok(Duration::DateTime(DateTime {
            date: Date::YMD {
                year: 2018,
                month: 4,
                day: 27,
            },
            time: Time {
                hour: 0,
                minute: 0,
                second: 0,
                millisecond: 0,
                tz_offset_hours: 0,
                tz_offset_minutes: 0,
            }
        })),
        duration("P2018-04-27T00:00:00")
    );
}

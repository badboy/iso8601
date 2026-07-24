use core::fmt::{self, Display};

use super::{Date, DateTime, Duration, Time};

impl Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // like `2015-11-02`
            Date::YMD { year, month, day } => write!(f, "{:04}-{:02}-{:02}", year, month, day),
            // like `2015-W45-01`
            Date::Week { year, ww, d } => write!(f, "{:04}-W{:02}-{:01}", year, ww, d),
            // like `2015-306`
            Date::Ordinal { year, ddd } => write!(f, "{:04}-{:03}", year, ddd),
        }
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // like `16:43:16.123+00:00`
        write!(
            f,
            "{:02}:{:02}:{:02}.{:03}+{:02}:{:02}",
            self.hour,
            self.minute,
            self.second,
            self.millisecond,
            self.tz_offset_hours,
            self.tz_offset_minutes
        )
    }
}

impl Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // like `16:43:16.123+00:00`
        write!(f, "{}T{}", self.date, self.time)
    }
}

impl Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Duration::YMDHMS {
                year,
                month,
                day,
                hour,
                minute,
                second,
                millisecond,
            } => {
                if self.is_zero() {
                    write!(f, "P0D")?;
                    return Ok(());
                }

                write!(f, "P")?;

                if *year > 0 {
                    write!(f, "{}Y", year)?
                }

                if *month > 0 {
                    write!(f, "{}M", month)?
                }

                if *day > 0 {
                    write!(f, "{}D", day)?
                }

                if *hour > 0 || *minute > 0 || *second > 0 || *millisecond > 0 {
                    write!(f, "T")?
                }
                if *hour > 0 {
                    write!(f, "{}H", hour)?
                }
                if *minute > 0 {
                    write!(f, "{}M", minute)?
                }

                if *millisecond > 0 {
                    write!(f, "{}.{:03}S", second, millisecond)?
                } else if *second > 0 {
                    write!(f, "{}S", second)?
                }
                Ok(())
            }
            Duration::Weeks(w) => write!(f, "P{}W", w),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::parse_duration;

    use super::*;

    fn test_duration_reparse(duration: Duration) {
        let serialized = format!("{}", duration);
        let reparsed = parse_duration(serialized.as_bytes()).unwrap().1;
        assert_eq!(duration, reparsed);
    }

    #[test]
    fn display_duration_0() {
        let duration = Duration::YMDHMS {
            year: 2021,
            month: 11,
            day: 16,
            hour: 23,
            minute: 26,
            second: 59,
            millisecond: 0,
        };
        test_duration_reparse(duration);
    }

    #[test]
    fn display_duration_1() {
        let duration = Duration::YMDHMS {
            year: 2021,
            month: 11,
            day: 16,
            hour: 23,
            minute: 26,
            second: 59,
            millisecond: 123,
        };
        test_duration_reparse(duration);
    }

    #[test]
    fn display_duration_2() {
        let duration = Duration::Weeks(50);
        test_duration_reparse(duration);
    }

    #[test]
    fn display_duration_small_milliseconds() {
        for millisecond in [1, 10, 100] {
            let duration = Duration::YMDHMS {
                year: 0,
                month: 0,
                day: 0,
                hour: 0,
                minute: 0,
                second: 0,
                millisecond,
            };
            test_duration_reparse(duration);
        }
    }

    #[test]
    fn display_time_small_milliseconds() {
        let time = crate::time("16:43:16.001").unwrap();
        assert_eq!(format!("{}", time), "16:43:16.001+00:00");

        let time = crate::time("16:43:16.010").unwrap();
        assert_eq!(format!("{}", time), "16:43:16.010+00:00");
    }

    fn test_date_reparse(date: Date) {
        let serialized = format!("{}", date);
        let reparsed = crate::parsers::parse_date(serialized.as_bytes()).unwrap().1;
        assert_eq!(date, reparsed);
    }

    #[test]
    fn display_date_ymd() {
        test_date_reparse(Date::YMD {
            year: 2015,
            month: 6,
            day: 26,
        });
    }

    #[test]
    fn display_date_week() {
        test_date_reparse(Date::Week {
            year: 2015,
            ww: 45,
            d: 1,
        });
    }

    #[test]
    fn display_date_ordinal() {
        test_date_reparse(Date::Ordinal {
            year: 2015,
            ddd: 306,
        });
    }

    #[test]
    fn display_date_negative_year() {
        test_date_reparse(Date::YMD {
            year: -333,
            month: 7,
            day: 11,
        });
    }

    fn test_datetime_reparse(datetime: DateTime) {
        let serialized = format!("{}", datetime);
        let reparsed = crate::parsers::parse_datetime(serialized.as_bytes())
            .unwrap()
            .1;
        assert_eq!(datetime, reparsed);
    }

    #[test]
    fn display_datetime_with_positive_offset() {
        test_datetime_reparse(DateTime {
            date: Date::YMD {
                year: 2015,
                month: 6,
                day: 26,
            },
            time: Time {
                hour: 16,
                minute: 43,
                second: 16,
                millisecond: 123,
                tz_offset_hours: 5,
                tz_offset_minutes: 30,
            },
        });
    }

    // The `Display` impl for `Time` hardcodes a literal `+` before the offset,
    // so a negative offset serializes to something like "+-05:00", which does
    // not round-trip back to the original value.
    #[test]
    fn display_time_with_negative_offset_does_not_round_trip() {
        let time = Time {
            hour: 16,
            minute: 43,
            second: 16,
            millisecond: 0,
            tz_offset_hours: -5,
            tz_offset_minutes: 0,
        };
        let serialized = format!("{}", time);
        let reparsed = crate::parsers::parse_time(serialized.as_bytes()).unwrap().1;
        assert_eq!(
            time, reparsed,
            "formatted as {:?}, which does not reparse back to the original value",
            serialized
        );
    }
}

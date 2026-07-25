use core::fmt::{self, Display};

use super::{Date, DateTime, Duration, Time};

fn write_year(f: &mut fmt::Formatter, year: i32) -> fmt::Result {
    if year < 0 {
        let year = -year;
        write!(f, "-{year:04}")
    } else {
        write!(f, "{year:04}")
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Date::YMD { year, month, day } => {
                write_year(f, year)?;
                write!(f, "-{month:02}-{day:02}")
            }
            Date::Week { year, ww, d } => {
                write_year(f, year)?;
                write!(f, "-W{ww:02}-{d:01}")
            }
            Date::Ordinal { year, ddd } => {
                write_year(f, year)?;
                write!(f, "-{ddd:03}")
            }
        }
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // like `16:43:16.123+00:00`
        let (sign, tz_offset_hours, tz_offset_minutes) =
            if self.tz_offset_hours < 0 || self.tz_offset_minutes < 0 {
                ('-', -self.tz_offset_hours, -self.tz_offset_minutes)
            } else {
                ('+', self.tz_offset_hours, self.tz_offset_minutes)
            };

        write!(
            f,
            "{:02}:{:02}:{:02}.{:03}{}{:02}:{:02}",
            self.hour,
            self.minute,
            self.second,
            self.millisecond,
            sign,
            tz_offset_hours,
            tz_offset_minutes
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

    fn assert_duration_reparse(duration: Duration) {
        let serialized = format!("{}", duration);
        let reparsed = parse_duration(serialized.as_bytes()).unwrap().1;
        assert_eq!(duration, reparsed);
    }

    #[test]
    fn duration_0() {
        let duration = Duration::YMDHMS {
            year: 2021,
            month: 11,
            day: 16,
            hour: 23,
            minute: 26,
            second: 59,
            millisecond: 0,
        };
        assert_duration_reparse(duration);
    }

    #[test]
    fn duration_1() {
        let duration = Duration::YMDHMS {
            year: 2021,
            month: 11,
            day: 16,
            hour: 23,
            minute: 26,
            second: 59,
            millisecond: 123,
        };
        assert_duration_reparse(duration);
    }

    #[test]
    fn duration_2() {
        let duration = Duration::Weeks(50);
        assert_duration_reparse(duration);
    }

    #[test]
    fn duration_small_milliseconds() {
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
            assert_duration_reparse(duration);
        }
    }

    #[test]
    fn time_small_milliseconds() {
        let time = crate::time("16:43:16.001").unwrap();
        assert_eq!(format!("{}", time), "16:43:16.001+00:00");

        let time = crate::time("16:43:16.010").unwrap();
        assert_eq!(format!("{}", time), "16:43:16.010+00:00");
    }

    fn assert_date_reparse(date: Date) {
        let serialized = format!("{}", date);
        let reparsed = crate::parsers::parse_date(serialized.as_bytes()).unwrap().1;
        assert_eq!(date, reparsed);
    }

    #[test]
    fn date_ymd() {
        assert_date_reparse(Date::YMD {
            year: 2015,
            month: 6,
            day: 26,
        });
    }

    #[test]
    fn date_week() {
        assert_date_reparse(Date::Week {
            year: 2015,
            ww: 45,
            d: 1,
        });
    }

    #[test]
    fn date_ordinal() {
        assert_date_reparse(Date::Ordinal {
            year: 2015,
            ddd: 306,
        });
    }

    #[test]
    fn date_negative_year() {
        assert_date_reparse(Date::YMD {
            year: -333,
            month: 7,
            day: 11,
        });
    }

    fn assert_datetime_reparse(datetime: DateTime) {
        let serialized = format!("{}", datetime);
        let reparsed = crate::parsers::parse_datetime(serialized.as_bytes())
            .unwrap()
            .1;
        assert_eq!(datetime, reparsed);
    }

    #[test]
    fn datetime_with_positive_offset() {
        assert_datetime_reparse(DateTime {
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

    #[test]
    fn time_with_negative_offset() {
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
        assert_eq!(time, reparsed, "serialized as {:?}", serialized);
    }
}

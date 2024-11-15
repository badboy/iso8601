# Parsing [ISO8601][iso] dates using [winnow][]

[![crates.io](https://img.shields.io/crates/v/iso8601?style=flat-square)](https://crates.io/crates/iso8601)
[![docs.rs docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/iso8601)
[![License: MIT](https://img.shields.io/github/license/badboy/iso8601?style=flat-square)](LICENSE)
[![Build status](https://img.shields.io/github/actions/workflow/status/badboy/iso8601/ci.yaml?branch=main)](https://github.com/badboy/iso8601/actions/workflows/ci.yaml)


[iso]: https://en.wikipedia.org/wiki/ISO_8601
[nom]: https://github.com/Geal/nom

```rust,ignore
let datetime = iso8601::datetime("2015-06-26T16:43:23+0200").unwrap();

// the above will give you:
DateTime {
    date: Date::YMD {
        year: 2015,
        month: 6,
        day: 26,
    },
    time: Time {
        hour: 16,
        minute: 43,
        second: 23,
        tz_offset_hours: 2,
        tz_offset_minutes: 0,
    },
};
```

This parses most cases of iso8601 dates.

# [Documentation][docs]

[Documentation][docs] is online.

# License

MIT Licensed. See [LICENSE]()

[docs]: https://docs.rs/iso8601/

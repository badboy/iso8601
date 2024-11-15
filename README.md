# winnow-iso8601, making parsing [ISO8601][iso] dates a breeze

[iso]: https://en.wikipedia.org/wiki/ISO_8601
[winnow]: https://github.com/winnow-rs/winnow
[iso-crate]: https://crates.io/crates/iso8601

## About

Provides a set of complete parsers to deal with simple cases where you are parsing a stand-alone date string.

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

Each of these complete methods simply build a `Partial<&[u8]>` which is flagged as complete. run the partial parsers
available. So, for most cases you would probably want to use: 

```rust,ignore

```

# Contributors

winnow-iso8601 is the fruit of the work of many contributors over the years, many
thanks for your help! In particular, thanks to [badboy](https://github.com/badboy)
and [hoodie](https://github.com/hoodie) for the original [`iso8601` crate][iso-crate] and actually reading the standard.

# [Documentation][docs]

[Documentation][docs] is online.

# License

MIT Licensed. See [LICENSE]()

[docs]: https://docs.rs/iso8601/

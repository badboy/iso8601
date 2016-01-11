# omnomnom - ~~Eating~~ Parsing [ISO8601][iso] dates using [nom][]

[![Build Status](https://travis-ci.org/badboy/iso8601.svg?branch=master)](https://travis-ci.org/badboy/iso8601)
[![crates.io](http://meritbadge.herokuapp.com/iso8601)](https://crates.io/crates/iso8601)

[iso]: https://en.wikipedia.org/wiki/ISO_8601
[nom]: https://github.com/Geal/nom

![omnomnom](http://24.media.tumblr.com/tumblr_lttcbyLaoP1r44hlho1_400.gif)

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
        tz_offset_hours: 0,
        tz_offset_minutes: 0,
    },
};
```

Still rough around the edges, though it won't fail with timezone offsets of half an hour anymore.
It's also safe for kittens now.

# [Documentation][docs]

[Documentation][docs] is online.

# License

MIT Licensed. See [LICENSE]()

[docs]: http://badboy.github.io/iso8601/iso8601/

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

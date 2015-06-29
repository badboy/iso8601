# omnomnom - ~~Eating~~ Parsing [ISO8601][iso] dates using [nom][]

[iso]: https://en.wikipedia.org/wiki/ISO_8601
[nom]: https://github.com/Geal/nom

![omnomnom](http://24.media.tumblr.com/tumblr_lttcbyLaoP1r44hlho1_400.gif)

```rust
datetime(b"2015-06-26T16:43:23+0200");

// the above will give you:
DateTime {
    date: Date { year: 2015, month: 6, day: 26 },
    time: Time { hour: 16, minute: 43, second: 23, tz_offset: 2 },
}
```

Rough around the edges, will fail with timezone offsets of half an hour and will likely eat some kittens on the way. Sorry.

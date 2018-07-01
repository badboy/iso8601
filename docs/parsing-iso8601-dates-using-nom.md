# omnomnom - Parsing ISO8601 dates using nom

There are thousands of ways to note down a date and time.
The international date format is standardized as [ISO8601][iso], though it still allows a widespread of different formats.

The basic format looks like this:

> 2015-07-02T19:45:00+0100

And that's what we will parse today using [nom][nom],
a parser combinator library created by [Geoffroy Couprie][gcouprie].

The idea is that you write small self-contained parsers, which all do only one simple thing, like parsing the year in our string, and then combine these small parsers to a bigger one to parse the full format.
`nom` comes with a wide variety of small parsers: handling different integers, reading simple byte arrays, optional fields, mapping parsed data over a function, ...
Most of them are provided as combinable macros.
It's very easy to implement your own small parsers, either by providing a method that handles a short byte buffer or by combining existing parsers.

So let's dive right in and see how to use nom in real code.

### Analysis

This is what we want to parse:

> 2015-07-02T19:45:00+0100

It has several parts we need to parse:

> YYYY-MM-DDTHH:MM:SS+OOOO

with the following meaning:

| Characters | Meaning                                                            |
| ---------- | -------                                                            |
| YYYY       | The year, can be negative or null and can be extended if necessary |
| MM         | Month from 1 to 12 (0-prefixed)                                    |
| DD         | Day from 1 to 31 (0-prefixed)                                      |
| T          | Separator between date and time                                    |
| HH         | Hour, 0-23 (0-prefixed)                                            |
| MM         | Minutes, 0-59 (0-prefixed)                                         |
| SS         | Seconds, 0-59 (0-prefixed)                                         |
| OOOO       | Timezone offset, separated by a `+` or `-` sign or `Z` for UTC     |

Parts like the seconds and the timezone offset are optional. Datetime strings without them will default to a zero value for that field.
The date parts are separated by a dash (`-`) and the time parts by a colon (`:`).

We will built a small parser for each of these parts and at the end combine them to parse a full date time string.

### Parsing the date: 2015-07-16

Let's start with the sign. As we need it several times, we create its own parser for that.
Parsers are created by giving them a name, stating the return value (or defaulting to a byte slice) and the parser combinators to handle the input.

~~~rust
named!(sign <&[u8], i32>, alt!(
        tag!("-") => { |_| -1 } |
        tag!("+") => { |_| 1 }
        )
    );
~~~

First, we parse either a plus or a minus sign.
This combines two already existing parsers: `tag!`, which will match the given byte array (in our case a single character) and `alt!`, which will try a list of parsers, returning on the first successful one.
We can directly map the result of the sub-parsers to either `-1` or `1`, so we don't need to deal with the byte slice later.

Next we parse the year, which consists of an optional sign and 4 digits (I know, I know, it is possible to extend this to more digits, but let's keep it simple for now).

~~~rust
named!(positive_year  <&[u8], i32>, map!(call!(take_4_digits), buf_to_i32));
named!(pub year <&[u8], i32>, chain!(
        pref: opt!(sign) ~
        y:    positive_year
        ,
        || {
            pref.unwrap_or(1) * y
        }));
~~~

A lot of additional stuff here. So let's separate it.

~~~rust
named!(positive_year  <&[u8], i32>, map!(call!(take_4_digits), buf_to_i32));
~~~

This creates a new named parser, that again returns the remaining input and an 32-bit integer.
To work, it first calls `take_4_digits` and then maps that result to the corresponding integer (using a [small helper function][buftoi32]).

`take_4_digits` is another small helper parser. We also got one for 2 digits:

~~~rust
named!(pub take_4_digits, flat_map!(take!(4), check!(is_digit)));
named!(pub take_2_digits, flat_map!(take!(2), check!(is_digit)));
~~~

This takes 4 (or 2) characters from the input and checks that each character is a digit.
`flat_map!` and `check!` are quite generic, so they are useful for a lot of cases.

~~~rust
named!(pub year <&[u8], i32>, chain!(
~~~

The year is also returned as a 32-bit integer (there's a pattern!).
Using the `chain!` macro, we can chain together multiple parsers and work with the sub-results.

~~~rust
        pref: opt!(sign) ~
        y:    positive_year
~~~

Our sign is directly followed by 4 digits. It's optional though, that's why we use `opt!`.
`~` is the concatenation operator in the `chain!` macro.
We save the sub-results to variables (`pref` and `y`).


~~~rust
        ,
        || {
            pref.unwrap_or(1) * y
        }));
~~~

To get the final result, we multiply the prefix (which comes back as either `1` or `-1`) with the year.
Don't forget the `,` (comma) right before the closure.
This is a small syntactic hint for the `chain!` macro that the mapping function will follow and no more parsers.

We can now successfully parse a year:

~~~rust
assert_eq!(Done(&[][..], 2015), year(b"2015"));
assert_eq!(Done(&[][..], -0333), year(b"-0333"));
~~~

Our nom parser will return an `IResult`. If all went well, we get `Done(I,O)` with `I` and `O` being the appropriate types.
For our case `I` is the same as the input, a buffer slice (`&[u8]`), and `O` is the output of the parser itself, an integer (`i32`).
The return value could also be an `Error(Err)`, if something went completely wrong, or `Incomplete(u32)`, requesting more data to be able to satisfy the parser (you can't parse a 4-digit year with only 3 characters input).

Parsing the month and day is a bit easier now: we simply take the digits and map them to an integer:

~~~rust
named!(pub month <&[u8], u32>, map!(call!(take_2_digits), buf_to_u32));
named!(pub day   <&[u8], u32>, map!(call!(take_2_digits), buf_to_u32));
~~~

All that's left is combining these 3 parts to parse a full date.
Again we can chain the different parsers and map it to some useful value:

~~~rust
named!(pub date <&[u8], Date>, chain!(
        y: year      ~
           tag!("-") ~
        m: month     ~
           tag!("-") ~
        d: day
        ,
        || { Date{ year: y, month: m, day: d }
        }
        ));
~~~

`Date` is a [small struct][datestruct], that can hold the necessary information, just as you would expect.

And it already works:

~~~rust
assert_eq!(Done(&[][..], Date{ year: 2015, month: 7, day: 16  }), date(b"2015-07-16"));
assert_eq!(Done(&[][..], Date{ year: -333, month: 6, day: 11  }), date(b"-0333-06-11"));
~~~

### Parsing the time: 16:43:52

Next, we parse the time. The individual parts are really simple, just some digits:

~~~rust
named!(pub hour   <&[u8], u32>, map!(call!(take_2_digits), buf_to_u32));
named!(pub minute <&[u8], u32>, map!(call!(take_2_digits), buf_to_u32));
named!(pub second <&[u8], u32>, map!(call!(take_2_digits), buf_to_u32));
~~~

Putting them together becomes a bit more complex, as the `second` part is optional:

~~~rust
named!(pub time <&[u8], Time>, chain!(
        h: hour      ~
           tag!(":") ~
        m: minute    ~
        s: empty_or!(chain!(tag!(":") ~ s:second , || { s }))
        ,
        || { Time{ hour: h,
                   minute: m,
                   second: s.unwrap_or(0),
                   tz_offset: 0 }
           }
        ));
~~~

As you can see, even `chain!` parsers can be nested.
The sub-parts then must be mapped once for the inner parser and once into the final value of the outer parser.
`empty_or!` returns an `Option`. Either `None` if there is no input left or it applies the nested parser. If this parser doesn't fail, `Some(value)` is returned.

Our parser now works for simple time information:

~~~rust
assert_eq!(Done(&[][..], Time{ hour: 16, minute: 43, second: 52, tz_offset: 0}), time(b"16:43:52"));
assert_eq!(Done(&[][..], Time{ hour: 16, minute: 43, second:  0, tz_offset: 0}), time(b"16:43"));
~~~

But it leaves out one important bit: the timezone.

### Parsing the timezone: +0100

~~~
2015-07-02T19:45:00-0500
2015-07-02T19:45:00Z
2015-07-02T19:45:00+01
~~~

Above are three variants of valid dates with timezones.
The timezone in an ISO8601 string is either an appended `Z`, indicating UTC,
or it's separated using a sign (`+` or `-`) and appends the offset from UTC in hours and minutes (with the minutes being optional).

Let's cover the UTC special case first:

~~~rust
named!(timezone_utc <&[u8], i32>, map!(tag!("Z"), |_| 0));
~~~

This should look familiar by now.
It's a simple `Z` character, which we map to `0`.

The other case is the sign-separated hour and minute offset.

~~~rust
named!(timezone_hour <&[u8], i32>, chain!(
        s: sign ~
        h: hour ~
        m: empty_or!(chain!(tag!(":")? ~ m: minute , || { m }))
        ,
        || { (s * (h as i32) * 3600) + (m.unwrap_or(0) * 60) as i32 }
        ));
~~~

We can re-use our already existing parsers and once again chain them to get what we want.
The minutes are optional (and might be separated using a colon).

Instead of keeping this as is, we're mapping it to the offset in seconds.
We will see why later.
We could also just map it to a tuple like <br>`(s, h, m.unwrap_or(0))` and handle conversion at a later point.

Combined we get

~~~rust
named!(timezone <&[u8], i32>, alt!(timezone_utc | timezone_hour));
~~~

### Putting it all together

We now got individual parsers for the date, the time and the timezone offset.

Putting it all together, our final datetime parser looks quite small and easy to understand:

~~~rust
named!(pub datetime <&[u8], DateTime>, chain!(
        d:   date      ~
             tag!("T") ~
        t:   time      ~
        tzo: empty_or!(call!(timezone))
        ,
        || {
            DateTime{
                date: d,
                time: t.set_tz(tzo.unwrap_or(0)),
            }
        }
        ));
~~~

Nothing special anymore. We can now parse all kinds of date strings:

~~~rust
datetime("2007-08-31T16:47+00:00");
datetime("2007-12-24T18:21Z");
datetime("2008-02-01T09:00:22+05");
~~~

But it will also parse invalid dates and times:

~~~rust
datetime("2234-13-42T25:70Z");
~~~

But this is fine for now. We can handle the actual validation in a later step.
For example, we could use [chrono][], a time library, [to handle this for us][chrono-convert].
Using chrono it's obvious why we already multiplied our timezone offset to be in seconds: this time we can just hand it off to chrono as is.

The full code for this ISO8601 parser is available in [easy.rs][easy.rs]. The repository also includes [a more complex parser][lib.rs], that does some validation while parsing
(it checks that the time and date are reasonable values, but it does not check that it is a valid date for example)

### What's left?

These simple parsers or even some more complex ones are already usable.
At least if you already got all the data at hand and if a simple return value satisfies your needs.
But especially for larger and more complex formats like media files reading everything into memory and spitting out a single large value isn't sufficient at all.

nom is prepared for that.
Soon it will become as easy as using an object from which nom can [`Read`][read].
For most things you shouldn't worry about that, as a simple `BufReader` will work.

For the other end of the chain, nom has [Consumers][consumer].
A Consumer handles the complex part of actually requesting data, calling the right sub-parsers and holding the necessary state.
This is what you need to build yourself.
Internally it's best abstracted using some kind of state machine, so you always know which part of the format to expect next, how to parse it, what to return to the user and so on.
Take a look at [the MP4 parser][mp4], which has an `MP4Consumer` handling the different parts of the format.
Soon my own library, [rdb-rs][rdb-rs], will have this as well.

Small thing aside: Geoffroy created [machine][] to define a state machine and I got [microstate][] for this.

### Why am I doing this?

I'm currently developing [rdb-rs][rdb-rs], a library to parse and analyze Redis dump files.
It's currently limited to parsing and reformatting into several formats and can be mainly used as a CLI utility.
But [there are projects][rsedis] that could benefit from a nicer API to integrate it into another tool.
The current parser is hand-made. It's fast, it's working, but it provides a limited, not very extensible API.
I hope to get a proper parser done with nom, that I can build on to provide all necessary methods, while still being super-fast and memory-safe.
Work [already started][rdb-rs-nom], but I'm far from done for now

---

Thanks to [Geoffroy][gcouprie] for the discussions, the help and for reading a draft of this post.

[iso]: https://en.wikipedia.org/wiki/ISO_8601
[repo]: https://github.com/badboy/iso8601
[nom]: https://github.com/Geal/nom
[gcouprie]: https://twitter.com/gcouprie
[taken]: https://github.com/badboy/iso8601/blob/master/src/macros.rs#L20-L39
[datestruct]: https://github.com/badboy/iso8601/blob/master/src/lib.rs#L19-23
[rdb-rs]: http://rdb.fnordig.de/
[rsedis]: https://github.com/seppo0010/rsedis
[rdb-rs-nom]: https://github.com/badboy/rdb-rs/tree/nom-parser
[mp4]: https://github.com/Geal/nom/blob/master/tests/mp4.rs
[chrono]: https://crates.io/crates/chrono
[chrono-convert]: https://github.com/badboy/iso8601/blob/master/src/lib.rs#L65-L71
[easy.rs]: https://github.com/badboy/iso8601/blob/master/src/easy.rs
[lib.rs]: https://github.com/badboy/iso8601/blob/master/src/lib.rs
[consumer]: https://github.com/Geal/nom#consumers
[machine]: https://github.com/Geal/machine
[microstate]: https://github.com/badboy/microstate
[buftoi32]: https://github.com/badboy/iso8601/blob/master/src/helper.rs#L8
[read]: http://doc.rust-lang.org/nightly/std/io/trait.Read.html

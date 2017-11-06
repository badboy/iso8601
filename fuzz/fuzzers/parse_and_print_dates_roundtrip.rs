#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate iso8601;

macro_rules! roundtrip {
    ($data:ident, $method:ident) => {
        if let Ok(x) = iso8601::$method($data) {
            let x_printed = format!("{}", x);
            let _parse_again = iso8601::$method(&x_printed);
        }
    };
}

fuzz_target!(|data| {
    if let Ok(data) = std::str::from_utf8(data) {
        roundtrip!(data, date);
        roundtrip!(data, time);
        roundtrip!(data, datetime);
    }
});

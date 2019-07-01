use std::str::{self, FromStr};

pub fn buf_to_u32(s: &[u8]) -> u32 {
    let s = str::from_utf8(s).expect("Invalid data, expected UTF-8 string");
    FromStr::from_str(s).expect("Invalid string, expected ASCII reprensation of a number")
}

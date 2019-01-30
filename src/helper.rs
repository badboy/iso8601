use std::str::{from_utf8_unchecked, FromStr};

pub fn to_string(s: &[u8]) -> &str {
    unsafe { from_utf8_unchecked(s) }
}
pub fn to_i32(s: &str) -> i32 {
    FromStr::from_str(s).unwrap()
}
pub fn to_u32(s: &str) -> u32 {
    FromStr::from_str(s).unwrap()
}

pub fn buf_to_u32(s: &[u8]) -> u32 {
    to_u32(to_string(s))
}
pub fn buf_to_i32(s: &[u8]) -> i32 {
    to_i32(to_string(s))
}

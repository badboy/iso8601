extern crate nom;
extern crate chrono;
extern crate iso8601;

use iso8601::datetime;
use std::env;
use nom::IResult::*;
use chrono::LocalResult;

fn main() {
    let mut args = env::args();
    let _program = args.next().unwrap();

    for arg in args {
        let t = datetime(arg.as_bytes());
        match t {
            Done(_, dt) =>  {
                match dt.to_chrono() {
                    LocalResult::Single(s) => println!("Date: {:?}", s),
                    LocalResult::Ambiguous(a,b) => println!("Date ambiguous: {:?} - {:?}", a, b),
                    LocalResult::None      => println!("Invalid datetime string: {:?}", arg),
                }
            }
            _ => { println!("Can't parse {:?}", arg); }
        }
    }
}

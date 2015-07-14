use nom::{self,is_digit};

#[macro_export]
macro_rules! empty_or(
    ($i:expr, $submac:ident!( $($args:tt)* )) => (
        if $i.len() == 0 {
            nom::IResult::Done($i, None)
        } else {
            match $submac!($i, $($args)*) {
                nom::IResult::Done(i,o)     => nom::IResult::Done(i, Some(o)),
                nom::IResult::Error(_)      => nom::IResult::Done($i, None),
                nom::IResult::Incomplete(i) => nom::IResult::Incomplete(i)

            }
        }
    );
);

#[macro_export]
macro_rules! check(
  ($input:expr, $submac:ident!( $($args:tt)* )) => (

    {
      let mut failed = false;
      for idx in 0..$input.len() {
        if !$submac!($input[idx], $($args)*) {
            failed = true;
            break;
        }
      }
      if failed {
        nom::IResult::Error(nom::Err::Position(nom::ErrorCode::Filter as u32,$input))
      } else {
        nom::IResult::Done(&b""[..], $input)
      }
    }
  );
  ($input:expr, $f:expr) => (
    check!($input, call!($f));
  );
);

#[macro_export]
macro_rules! char_between(
    ($input:expr, $min:expr, $max:expr) => (
        {
        fn f(c: u8) -> bool { c >= ($min as u8) && c <= ($max as u8)}
        flat_map!($input, take!(1), check!(f))
        }
    );
);


named!(pub take_4_digits, flat_map!(take!(4), check!(is_digit)));
named!(pub take_2_digits, flat_map!(take!(2), check!(is_digit)));

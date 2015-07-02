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
macro_rules! take_n_filter(
  ($input:expr, $count:expr, $submac:ident!( $($args:tt)* )) => (
      {
          if $input.len() < $count {
              return nom::IResult::Incomplete(nom::Needed::Size($count))
          }

          for idx in 0..$count {
              if !$submac!($input[idx], $($args)*) {
                  return nom::IResult::Error(nom::Err::Position(42, $input))
              }
          }

          nom::IResult::Done(&$input[$count..], &$input[0..$count])
      }
  );
  ($input:expr, $count: expr, $f:expr) => (
      take_n_filter!($input, $count, call!($f));
  );
);

#[macro_export]
macro_rules! char_between(
    ($input:expr, $min:expr, $max:expr) => (
        {
        fn f(c: u8) -> bool { c >= ($min as u8) && c <= ($max as u8)}
        take_n_filter!($input, 1, f)
        }
    );
);

named!(pub take_4_digits, take_n_filter!(4, is_digit));
named!(pub take_2_digits, take_n_filter!(2, is_digit));

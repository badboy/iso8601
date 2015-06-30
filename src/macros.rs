/// Take n bytes and ensure that they are only in the provided range of bytes
#[macro_export]
macro_rules! take_n_between(
    ($input:expr, $count:expr, $min:expr, $max:expr) => (
        {
            let new_min = $min as u8;
            let new_max = $max as u8;
            let cnt = $count as usize;
            if $input.len() < cnt {
                nom::IResult::Incomplete(nom::Needed::Size(cnt))
            } else {
                for idx in 0..$count {
                    if $input[idx] < new_min || $input[idx] > new_max {
                        return nom::IResult::Error(nom::Err::Position(42 as u32,$input));
                    }
                }

                nom::IResult::Done(&$input[$count..], &$input[0..$count])
            }
        }
        );
    );

#[macro_export]
macro_rules! char_between(
    ($input:expr, $min:expr, $max:expr) => (
        take_n_between!($input, 1, $min, $max)
    );
);

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
              return nom::IResult::Incomplete(Needed::Size($count))
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

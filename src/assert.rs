use std::fmt;

pub fn print_result<T: fmt::Debug>(input: &str, rest: &[u8], result: &T) {
    println!(
        "INPUT: {:?}\nLEFT:  {:?}\nRESULT: {:#?}",
        input, rest, result
    );
}

#[macro_export]
macro_rules! assert_parser {
    ($parser:ident, $line:expr, $expectation:expr) => {{
        use std::string::ToString;

        let (rest, parsed) = $parser($line.as_bytes()).unwrap();
        crate::assert::print_result($line, &rest, &parsed);

        assert_eq!(
            parsed, $expectation,
            "{:?} not parsed as expected (leftover: {:?})",
            $line, rest
        );
        assert!(rest.is_empty(), "not parsed completely");

        let serialized = parsed.to_string();
        assert_eq!($line, serialized, "does not reserialize correctly");
        println!("✅");
    }};
}

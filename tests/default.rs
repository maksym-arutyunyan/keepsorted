use keepsorted::process_lines;
use std::io::{self};

// Helper function to hide text-lines conversion.
fn process_default(text: &str) -> io::Result<String> {
    let lines: Vec<&str> = text.lines().collect();
    let processed_lines = process_lines(lines)?;
    Ok(processed_lines.join("\n"))
}

// Macro for defining the core test logic.
macro_rules! test_inner {
    ($input:expr, $expected:expr) => {{
        let input = $input;
        let expected = $expected;
        let result = process_default(input).unwrap();
        assert_eq!(
            result, expected,
            "Expected: {}\nActual: {}",
            expected, result
        );
    }};
}

#[test]
fn empty() {
    test_inner!("", "");
}

#[test]
fn single_letter() {
    test_inner!(
        "
            a
        ",
        "
            a
        "
    );
}

#[test]
fn no_comment() {
    test_inner!(
        "
            b
            a
        ",
        "
            b
            a
        "
    );
}

#[test]
fn simple_block() {
    test_inner!(
        "
            # Keep sorted.
            b
            a
        ",
        "
            # Keep sorted.
            a
            b
        "
    );
}

#[test]
fn blocks_divided_by_newline() {
    test_inner!(
        "
            # Keep sorted.
            d
            c

            b
            a
        ",
        "
            # Keep sorted.
            c
            d

            b
            a
        "
    );
}

use keepsorted::process_lines;
use std::io::{self};

// Helper function to hide text-lines conversion.
fn process_default(text: &str) -> io::Result<String> {
    let lines: Vec<&str> = text.lines().collect();
    let processed_lines = process_lines(lines)?;
    Ok(processed_lines.join("\n"))
}

#[test]
fn empty() {
    let input = "";
    let expected = "";
    let result = process_default(input).unwrap();
    assert!(result == expected, "Expected: {expected}\nActual: {result}");
}

#[test]
fn single_letter() {
    let input = "
        a
    ";
    let expected = "
        a
    ";
    let result = process_default(input).unwrap();
    assert!(result == expected, "Expected: {expected}\nActual: {result}");
}

#[test]
fn no_comment() {
    let input = "
        b
        a
    ";
    let expected = "
        b
        a
    ";
    let result = process_default(input).unwrap();
    assert!(result == expected, "Expected: {expected}\nActual: {result}");
}

#[test]
fn simple_block() {
    let input = "
        # Keep sorted.
        b
        a
    ";
    let expected = "
        # Keep sorted.
        a
        b
    ";
    let result = process_default(input).unwrap();
    assert!(result == expected, "Expected: {expected}\nActual: {result}");
}

#[test]
fn blocks_divided_by_newline() {
    let input = "
        # Keep sorted.
        d
        c

        b
        a
    ";
    let expected = "
        # Keep sorted.
        c
        d

        b
        a
    ";
    let result = process_default(input).unwrap();
    assert!(result == expected, "Expected: {expected}\nActual: {result}");
}

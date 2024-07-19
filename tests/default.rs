use keepsorted::{process_lines, SortStrategy};
use std::io::{self};

// Helper function to hide text-lines conversion.
fn process_default(text: &str) -> io::Result<String> {
    let lines: Vec<&str> = text.lines().collect();
    let processed_lines = process_lines(SortStrategy::Default, lines)?;
    Ok(processed_lines.join("\n"))
}

// Macro for defining the core test logic.
macro_rules! test_inner {
    ($input:expr, $expected:expr) => {{
        let input = $input;
        let expected = $expected;
        let result = process_default(input).unwrap();
        assert!(
            result == expected,
            "Expected: {}\nActual: {}",
            expected,
            result
        );
    }};
}

#[test]
fn default_empty() {
    test_inner!("", "");
}

#[test]
fn default_single_item() {
    test_inner!(
        r#"
            a
        "#,
        r#"
            a
        "#
    );
}

#[test]
fn default_no_sorting_comment() {
    test_inner!(
        r#"
            b
            a
        "#,
        r#"
            b
            a
        "#
    );
}

#[test]
fn default_simple_block() {
    test_inner!(
        r#"
            # Keep sorted.
            b
            a
        "#,
        r#"
            # Keep sorted.
            a
            b
        "#
    );
}

#[test]
fn default_blocks_divided_by_newline() {
    test_inner!(
        r#"
            # Keep sorted.
            d
            c

            b
            a
        "#,
        r#"
            # Keep sorted.
            c
            d

            b
            a
        "#
    );
}

// TODO: move to the appropriate place.
#[test]
#[ignore]
fn with_multi_line_comment_rust() {
    test_inner!(
        r#"
            // Keep sorted.
            y,
            /* Some multi-line comment,
               for the line below.  */,
            x,
            b,
            a,
        "#,
        r#"
            // Keep sorted.
            a,
            b,
            /* Some multi-line comment,
               for the line below.  */,
            x,
            y,
        "#
    );
}

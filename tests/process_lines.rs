use keepsorted::{process_lines, process_lines_bazel};
use std::io::{self};

// Helper function to hide text-lines conversion.
fn process_text(text: &str) -> io::Result<String> {
    let lines: Vec<&str> = text.lines().collect();
    let processed_lines = process_lines(lines)?;
    Ok(processed_lines.join("\n"))
}

fn process_text_bazel(text: &str) -> io::Result<String> {
    let lines: Vec<&str> = text.lines().collect();
    let processed_lines = process_lines_bazel(lines)?;
    Ok(processed_lines.join("\n"))
}

#[test]
fn empty() {
    let input = "";
    let expected = "";
    let result = process_text(input).unwrap();
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
    let result = process_text(input).unwrap();
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
    let result = process_text(input).unwrap();
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
    let result = process_text(input).unwrap();
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
    let result = process_text(input).unwrap();
    assert!(result == expected, "Expected: {expected}\nActual: {result}");
}

#[test]
fn bazel_block() {
    let input = r#"
        block = [
            # Keep sorted.
            "b",
            "a",
        ]
    "#;
    let expected = r#"
        block = [
            # Keep sorted.
            "a",
            "b",
        ]
    "#;
    let result = process_text_bazel(input).unwrap();
    assert!(result == expected, "Expected: {expected}\nActual: {result}");
}

#[test]
fn bazel_block_with_comment() {
    let input = r#"
        block = [
            # Keep sorted.
            "d",
            # Some comment about the line below.
            "c",
            "b",  # TODO[bbb]
            "a",
            # Trailing comment.
        ]
    "#;
    let expected = r#"
        block = [
            # Keep sorted.
            "a",
            "b",  # TODO[bbb]
            # Some comment about the line below.
            "c",
            "d",
            # Trailing comment.
        ]
    "#;
    let result = process_text_bazel(input).unwrap();
    assert!(result == expected, "Expected: {expected}\nActual: {result}");
}

#[test]
fn bazel_blocks() {
    let input = r#"
        block_1 = [
            # Keep sorted.
            "b",
            "a",
        ],
        block_2 = [
            "y",
            "x",
        ],
    "#;
    let expected = r#"
        block_1 = [
            # Keep sorted.
            "a",
            "b",
        ],
        block_2 = [
            "y",
            "x",
        ],
    "#;
    let result = process_text_bazel(input).unwrap();
    assert!(result == expected, "Expected: {expected}\nActual: {result}");
}

#[test]
fn bazel_blocks_select() {
    let input = r#"
        deps = [
            # Keep sorted.
            "b",
            "a",
        ] + select({
            "@platforms//os:osx": [
                # Keep sorted.
                "y",
                "x",
            ],
            "//conditions:default": [
                # Keep sorted.
                "m",
                "k",
            ],
        })
    "#;
    let expected = r#"
        deps = [
            # Keep sorted.
            "a",
            "b",
        ] + select({
            "@platforms//os:osx": [
                # Keep sorted.
                "x",
                "y",
            ],
            "//conditions:default": [
                # Keep sorted.
                "k",
                "m",
            ],
        })
    "#;
    let result = process_text_bazel(input).unwrap();
    assert!(result == expected, "Expected: {expected}\nActual: {result}");
}

#[test]
fn bazel_order() {
    let input = r#"
        block = [
            # Keep sorted.
            ":b",
            ":a",
            "//path/b",
            "//path/a",
            "@crate_index//:b",
            "@crate_index//:a",
        ]
    "#;
    let expected = r#"
        block = [
            # Keep sorted.
            ":a",
            ":b",
            "//path/a",
            "//path/b",
            "@crate_index//:a",
            "@crate_index//:b",
        ]
    "#;
    let result = process_text_bazel(input).unwrap();
    assert!(result == expected, "Expected: {expected}\nActual: {result}");
}

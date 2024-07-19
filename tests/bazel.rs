use keepsorted::process_lines_bazel;
use std::io::{self};

// Helper function to hide text-lines conversion.
fn process_bazel(text: &str) -> io::Result<String> {
    let lines: Vec<&str> = text.lines().collect();
    let processed_lines = process_lines_bazel(lines)?;
    Ok(processed_lines.join("\n"))
}

// Macro for defining the core test logic.
macro_rules! test_inner {
    ($input:expr, $expected:expr) => {{
        let input = $input;
        let expected = $expected;
        let result = process_bazel(input).unwrap();
        assert_eq!(
            result, expected,
            "Expected: {}\nActual: {}",
            expected, result
        );
    }};
}

#[test]
fn bazel_block() {
    test_inner!(
        r#"
            block = [
                # Keep sorted.
                "b",
                "a",
            ]
        "#,
        r#"
            block = [
                # Keep sorted.
                "a",
                "b",
            ]
        "#
    );
}

#[test]
fn bazel_block_with_comment() {
    test_inner!(
        r#"
            block = [
                # Keep sorted.
                "d",
                # Some comment about the line below.
                "c",
                "b",  # TODO[bbb]
                "a",
                # Trailing comment.
            ]
        "#,
        r#"
            block = [
                # Keep sorted.
                "a",
                "b",  # TODO[bbb]
                # Some comment about the line below.
                "c",
                "d",
                # Trailing comment.
            ]
        "#
    );
}

#[test]
fn bazel_blocks() {
    test_inner!(
        r#"
            block_1 = [
                # Keep sorted.
                "b",
                "a",
            ],
            block_2 = [
                "y",
                "x",
            ],
        "#,
        r#"
            block_1 = [
                # Keep sorted.
                "a",
                "b",
            ],
            block_2 = [
                "y",
                "x",
            ],
        "#
    );
}

#[test]
fn bazel_blocks_select() {
    test_inner!(
        r#"
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
        "#,
        r#"
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
        "#
    );
}

#[test]
fn bazel_order() {
    test_inner!(
        r#"
            block = [
                # Keep sorted.
                ":b",
                ":a",
                "//path/b",
                "//path/a",
                "@crate_index//:b",
                "@crate_index//:a",
            ]
        "#,
        r#"
            block = [
                # Keep sorted.
                ":a",
                ":b",
                "//path/a",
                "//path/b",
                "@crate_index//:a",
                "@crate_index//:b",
            ]
        "#
    );
}

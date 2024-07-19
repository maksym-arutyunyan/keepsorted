use keepsorted::{process_lines, SortStrategy};
use std::io::{self};

// Helper function to hide text-lines conversion.
fn process_bazel(text: &str) -> io::Result<String> {
    let lines: Vec<&str> = text.lines().collect();
    let processed_lines = process_lines(SortStrategy::Bazel, lines)?;
    Ok(processed_lines.join("\n"))
}

// Macro for defining the core test logic.
macro_rules! test_inner {
    ($input:expr, $expected:expr) => {{
        let input = $input;
        let expected = $expected;
        let result = process_bazel(input).unwrap();
        assert!(
            result == expected,
            "Expected: {}\nActual: {}",
            expected,
            result
        );
    }};
}

#[test]
fn bazel_single_block() {
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
fn bazel_inline_comment() {
    test_inner!(
        r#"
            block = [
                # Keep sorted.
                "y",
                "x",  # Some in-line comment.
                "b",
                "a",
            ]
        "#,
        r#"
            block = [
                # Keep sorted.
                "a",
                "b",
                "x",  # Some in-line comment.
                "y",
            ]
        "#
    );
}

#[test]
fn bazel_inline_comment_with_braces() {
    test_inner!(
        r#"
            block = [
                # Keep sorted.
                "y",
                "x",  # TODO[xxx].
                "b",
                "a",
            ]
        "#,
        r#"
            block = [
                # Keep sorted.
                "a",
                "b",
                "x",  # TODO[xxx].
                "y",
            ]
        "#
    );
}

#[test]
fn bazel_multi_line_comment() {
    test_inner!(
        r#"
            block = [
                # Keep sorted.
                "y",
                # Some multi-line comment,
                # for the line below.,
                "x",
                "b",
                "a",
            ]
        "#,
        r#"
            block = [
                # Keep sorted.
                "a",
                "b",
                # Some multi-line comment,
                # for the line below.,
                "x",
                "y",
            ]
        "#
    );
}

#[test]
fn bazel_multi_line_trailing_comment() {
    test_inner!(
        r#"
            block = [
                # Keep sorted.
                "b",
                "a",
                # Some multi-line comment
                # trailing comment.
            ]
        "#,
        r#"
            block = [
                # Keep sorted.
                "a",
                "b",
                # Some multi-line comment
                # trailing comment.
            ]
        "#
    );
}

#[test]
fn bazel_several_multi_line_comments() {
    test_inner!(
        r#"
            block = [
                # Keep sorted.
                "y",
                # Some multi-line comment
                # for the line below.
                "x",
                "b",
                "a",
                # Some multi-line comment
                # trailing comment.
            ]
        "#,
        r#"
            block = [
                # Keep sorted.
                "a",
                "b",
                # Some multi-line comment
                # for the line below.
                "x",
                "y",
                # Some multi-line comment
                # trailing comment.
            ]
        "#
    );
}

#[test]
fn bazel_single_block_with_comment() {
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
fn bazel_blocks_with_select() {
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
                ":bbb",
                ":aaa",
                "nested",
                "//dir/subdir/folder:yyy",  # TODO[yyy]
                "//dir/subdir/folder:xxx",
                "//dir/subdir/folder",  # Some in-line comment.
                "//dir/subdir:bbb",
                "//dir/subdir:aaa",
                "@crate_index//project",
                "@crate_index//:base64-bytestring",
                "@crate_index//:base32",
                "@crate_index//:base",
                "@crate_index//:bbb",
                "@crate_index//:aaa",
                requirement("gitpython"),
                requirement("python-gitlab"),
                requirement("pyyaml"),
            ]
        "#,
        r#"
            block = [
                # Keep sorted.
                "nested",
                ":aaa",
                ":bbb",
                "//dir/subdir:aaa",
                "//dir/subdir:bbb",
                "//dir/subdir/folder",  # Some in-line comment.
                "//dir/subdir/folder:xxx",
                "//dir/subdir/folder:yyy",  # TODO[yyy]
                "@crate_index//:aaa",
                "@crate_index//:base",
                "@crate_index//:base32",
                "@crate_index//:base64-bytestring",
                "@crate_index//:bbb",
                "@crate_index//project",
                requirement("gitpython"),
                requirement("python-gitlab"),
                requirement("pyyaml"),
            ]
        "#
    );
}

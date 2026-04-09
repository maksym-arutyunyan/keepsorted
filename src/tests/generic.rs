use super::common::check;
use crate::Strategy::Generic;

#[test]
fn generic_simple_block() {
    check(
        Generic,
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
fn generic_simple_block_2() {
    check(
        Generic,
        r#"
// Keep sorted.
b
a
        "#,
        r#"
// Keep sorted.
a
b
        "#
    );
}

#[test]
fn generic_simple_block_3() {
    check(
        Generic,
        r#"
# keepsorted: keep sorted
b
a
        "#,
        r#"
# keepsorted: keep sorted
a
b
        "#
    );
}

#[test]
fn generic_blocks_with_newline() {
    check(
        Generic,
        r#"
# Keep sorted.
y
x

b
a
        "#,
        r#"
# Keep sorted.
x
y

b
a
        "#
    );
}

#[test]
fn generic_several_blocks() {
    check(
        Generic,
        r#"
# Keep sorted.
y
x

# Keep sorted.
b
a
        "#,
        r#"
# Keep sorted.
x
y

# Keep sorted.
a
b
        "#
    );
}

#[test]
fn generic_block_with_multi_line_comment() {
    check(
        Generic,
        r#"
# Keep sorted.
y
# Some multi-line comment
# for the line below.
x
b
a
        "#,
        r#"
# Keep sorted.
a
b
# Some multi-line comment
# for the line below.
x
y
        "#
    );
}

#[test]
fn generic_block_with_trailing_comment() {
    check(
        Generic,
        r#"
# Keep sorted.
b
a
# Some multi-line comment
# trailing comment.
        "#,
        r#"
# Keep sorted.
a
b
# Some multi-line comment
# trailing comment.
        "#
    );
}

#[test]
fn generic_block_with_inline_comment() {
    check(
        Generic,
        r#"
# Keep sorted.
y
x  # Some in-line comment.
b
a
        "#,
        r#"
# Keep sorted.
a
b
x  # Some in-line comment.
y
        "#
    );
}

#[test]
fn generic_ignore_file() {
    check(
        Generic,
        r#"
  # keepsorted: ignore file
# Keep sorted.
1b
1a

# Keep sorted.
2b
2a

# Keep sorted.
3b
3a
        "#,
        r#"
  # keepsorted: ignore file
# Keep sorted.
1b
1a

# Keep sorted.
2b
2a

# Keep sorted.
3b
3a
        "#
    );
}

#[test]
fn generic_ignore_block_inside() {
    check(
        Generic,
        r#"
# Keep sorted.
1b
1a

# Keep sorted.
#    keepsorted: ignore block
2b
2a

# Keep sorted.
3b
3a
        "#,
        r#"
# Keep sorted.
1a
1b

# Keep sorted.
#    keepsorted: ignore block
2b
2a

# Keep sorted.
3a
3b
        "#
    );
}

#[test]
fn generic_ignore_block_before() {
    check(
        Generic,
        r#"
# Keep sorted.
1b
1a

#    keepsorted: ignore block
# Keep sorted.
2b
2a

# Keep sorted.
3b
3a
        "#,
        r#"
# Keep sorted.
1a
1b

#    keepsorted: ignore block
# Keep sorted.
2b
2a

# Keep sorted.
3a
3b
        "#
    );
}

// Tracked in: https://github.com/maksym-arutyunyan/keepsorted/issues/133
#[test]
#[ignore = "multi-line block comments (/* ... */) not yet supported"]
fn with_multi_line_comment_rust() {
    check(
        Generic,
        r#"
// Keep sorted.
y,
/*
 * Some multi-line comment
 * for the line below.
 */
x,
b,
a,
        "#,
        r#"
// Keep sorted.
a,
b,
/*
 * Some multi-line comment
 * for the line below.
 */
x,
y,
        "#
    );
}

#[test]
fn generic_simple_lua_table() {
    check(
        Generic,
        r#"
local config = {
    -- Keep sorted.
    setting = true,
    name = "some name",
}
        "#,
        r#"
local config = {
    -- Keep sorted.
    name = "some name",
    setting = true,
}
        "#
    );
}

#[test]
fn generic_nested_lua_tables() {
    check(
        Generic,
        r#"
local config = {
    b = {
        "ghijkl",
        "abcdef",
    },
    a = {
        -- Keep sorted.
        "ghijkl",
        "abcdef",
    }
}
        "#,
        r#"
local config = {
    b = {
        "ghijkl",
        "abcdef",
    },
    a = {
        -- Keep sorted.
        "abcdef",
        "ghijkl",
    }
}
        "#
    );
}

#[test]
fn generic_nested_lua_tables_specific_example() {
    check(
        Generic,
        r#"
local candidates = {
  light = {
    -- keep sorted
    "catppuccin-latte",
    "base16-catppuccin-latte",
  },
  dark = {},
}"#,
        r#"
local candidates = {
  light = {
    -- keep sorted
    "base16-catppuccin-latte",
    "catppuccin-latte",
  },
  dark = {},
}"#
    );
}

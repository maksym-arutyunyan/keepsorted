#[macro_use]
mod common;

use keepsorted::Strategy::Generic;

#[test]
fn generic_simple_block() {
    test_inner!(
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
fn generic_blocks_with_newline() {
    test_inner!(
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
    test_inner!(
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
    test_inner!(
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
    test_inner!(
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
    test_inner!(
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
    test_inner!(
        Generic,
        r#"
    keepsorted:ignore-file
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
    keepsorted:ignore-file
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
    test_inner!(
        Generic,
        r#"
# Keep sorted.
1b
1a

# Keep sorted.
    keepsorted:ignore-block
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
    keepsorted:ignore-block
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
    test_inner!(
        Generic,
        r#"
# Keep sorted.
1b
1a

    keepsorted:ignore-block
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

    keepsorted:ignore-block
# Keep sorted.
2b
2a

# Keep sorted.
3a
3b
        "#
    );
}

// TODO: move to the appropriate place.
#[test]
#[ignore]
fn with_multi_line_comment_rust() {
    test_inner!(
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

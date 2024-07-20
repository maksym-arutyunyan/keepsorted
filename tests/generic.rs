#[macro_use]
mod common;

use keepsorted::SortStrategy::Generic;

#[test]
fn generic_no_sorting_comment() {
    test_inner!(
        Generic,
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
fn generic_blocks_divided_by_newline() {
    test_inner!(
        Generic,
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
        Generic,
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

#[macro_use]
mod common;

use keepsorted::SortStrategy::Default;

#[test]
fn default_empty() {
    test_inner!(Default, "", "");
}

#[test]
fn default_single_item() {
    test_inner!(
        Default,
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
        Default,
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
        Default,
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
        Default,
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
        Default,
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

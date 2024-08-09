#[macro_use]
mod common;

use keepsorted::Strategy::Gitignore;

#[test]
fn gitignore_1() {
    test_inner!(
        Gitignore,
        r#"

/b
/a

# [Bazel]
/b
/a

# [Rust]
/b
/a
        "#,
        r#"

/a
/b

# [Bazel]
/a
/b

# [Rust]
/a
/b
        "#
    );
}

#[test]
fn gitignore_ignore_file() {
    test_inner!(
        Gitignore,
        r#"
# keepsorted:ignore-file

/b
/a

# [Bazel]
/b
/a

# [Rust]
/b
/a
        "#,
        r#"
# keepsorted:ignore-file

/b
/a

# [Bazel]
/b
/a

# [Rust]
/b
/a
        "#
    );
}

#[test]
fn gitignore_ignore_block_after_header_comment() {
    test_inner!(
        Gitignore,
        r#"

/b
/a

# [Bazel]
# keepsorted:ignore-block
/b
/a

# [Rust]
/b
/a
        "#,
        r#"

/a
/b

# [Bazel]
# keepsorted:ignore-block
/b
/a

# [Rust]
/a
/b
        "#
    );
}

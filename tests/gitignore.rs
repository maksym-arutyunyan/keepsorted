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

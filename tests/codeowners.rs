#[macro_use]
mod common;

use keepsorted::Strategy::CodeOwners;

#[test]
fn codeowners_simple_block() {
    test_inner!(
        CodeOwners,
        r#"
/.d/                 @company/teams/a
/.c/                 @company/teams/b
/.b/workflows        @company/teams/c @company/teams/d
/.a/CODEOWNERS       @company/teams/e
        "#,
        r#"
/.a/CODEOWNERS       @company/teams/e
/.b/workflows        @company/teams/c @company/teams/d
/.c/                 @company/teams/b
/.d/                 @company/teams/a
        "#
    );
}

#[test]
fn codeowners_two_blocks() {
    test_inner!(
        CodeOwners,
        r#"
/.d/                 @company/teams/a
/.c/                 @company/teams/b

/.b/workflows        @company/teams/c @company/teams/d
/.a/CODEOWNERS       @company/teams/e
        "#,
        r#"
/.c/                 @company/teams/b
/.d/                 @company/teams/a

/.a/CODEOWNERS       @company/teams/e
/.b/workflows        @company/teams/c @company/teams/d
        "#
    );
}

#[test]
fn codeowners_1() {
    test_inner!(
        CodeOwners,
        r#"

# [Misc]
/b
/a

# [Bazel]
/b
/a

# [Rust Lang]
/b
/a
        "#,
        r#"

# [Misc]
/a
/b

# [Bazel]
/a
/b

# [Rust Lang]
/a
/b
        "#
    );
}

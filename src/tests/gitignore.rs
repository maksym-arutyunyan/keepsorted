use crate::Strategy::Gitignore;

// --- .gitignore ---

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
# keepsorted: ignore file

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
# keepsorted: ignore file

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
# keepsorted: ignore block
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
# keepsorted: ignore block
/b
/a

# [Rust]
/a
/b
        "#
    );
}

// --- CODEOWNERS ---

#[test]
fn codeowners_simple_block() {
    test_inner!(
        Gitignore,
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
        Gitignore,
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
fn codeowners_ignore_file() {
    test_inner!(
        Gitignore,
        r#"
# keepsorted: ignore file
/.d/                 @company/teams/a
/.c/                 @company/teams/b

/.b/workflows        @company/teams/c @company/teams/d
/.a/CODEOWNERS       @company/teams/e
        "#,
        r#"
# keepsorted: ignore file
/.d/                 @company/teams/a
/.c/                 @company/teams/b

/.b/workflows        @company/teams/c @company/teams/d
/.a/CODEOWNERS       @company/teams/e
        "#
    );
}

#[test]
fn codeowners_ignore_block() {
    test_inner!(
        Gitignore,
        r#"
# keepsorted: ignore block
/.d/                 @company/teams/a
/.c/                 @company/teams/b

/.b/workflows        @company/teams/c @company/teams/d
/.a/CODEOWNERS       @company/teams/e
        "#,
        r#"
# keepsorted: ignore block
/.d/                 @company/teams/a
/.c/                 @company/teams/b

/.a/CODEOWNERS       @company/teams/e
/.b/workflows        @company/teams/c @company/teams/d
        "#
    );
}

#[test]
fn codeowners_1() {
    test_inner!(
        Gitignore,
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

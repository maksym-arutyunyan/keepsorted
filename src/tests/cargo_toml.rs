use super::common::check;
use crate::Strategy::CargoToml;

#[test]
fn cargo_toml_simple() {
    check(
        CargoToml,
        r#"
[dependencies]
b = "2"
a = "1"
        "#,
        r#"
[dependencies]
a = "1"
b = "2"
        "#
    );
}

#[test]
fn cargo_toml_list_with_item_comment() {
    check(
        CargoToml,
        r#"
[dependencies]
c = "3"
b = "2"
# Some comment related to line below.
a = "1"
        "#,
        r#"
[dependencies]
# Some comment related to line below.
a = "1"
b = "2"
c = "3"
        "#
    );
}

#[test]
fn cargo_toml_list_with_inline_comment() {
    check(
        CargoToml,
        r#"
[dependencies]
c = "3"
b = "2"
a = "1"  # Some in-line comment.
        "#,
        r#"
[dependencies]
a = "1"  # Some in-line comment.
b = "2"
c = "3"
        "#
    );
}

#[test]
fn cargo_toml_two_scopes() {
    check(
        CargoToml,
        r#"
[dependencies]
b = "2"
a = "1"
[lib]
name = "some_name"
path = "src/lib.rs"
        "#,
        r#"
[dependencies]
a = "1"
b = "2"
[lib]
name = "some_name"
path = "src/lib.rs"
        "#
    );
}

#[test]
fn cargo_toml_block_with_newline_inside() {
    check(
        CargoToml,
        r#"
[dependencies]
b = "2"
a = "1"

y = "4"
x = "3"
        "#,
        r#"
[dependencies]
a = "1"
b = "2"

y = "4"
x = "3"
        "#
    );
}

#[test]
fn cargo_toml_two_blocks() {
    check(
        CargoToml,
        r#"
[dependencies]
b = "2"
a = "1"

[dev-dependencies]
y = "4"
x = "3"
        "#,
        r#"
[dependencies]
a = "1"
b = "2"

[dev-dependencies]
x = "3"
y = "4"
        "#
    );
}

#[test]
fn cargo_toml_ignore_file() {
    check(
        CargoToml,
        r#"
# keepsorted: ignore file
[dependencies]
b = "2"
a = "1"

[dev-dependencies]
y = "4"
x = "3"
        "#,
        r#"
# keepsorted: ignore file
[dependencies]
b = "2"
a = "1"

[dev-dependencies]
y = "4"
x = "3"
        "#
    );
}

#[test]
fn cargo_toml_ignore_block_inside() {
    check(
        CargoToml,
        r#"
[dependencies]
# keepsorted: ignore block
b = "2"
a = "1"

[dev-dependencies]
y = "4"
x = "3"
        "#,
        r#"
[dependencies]
# keepsorted: ignore block
b = "2"
a = "1"

[dev-dependencies]
x = "3"
y = "4"
        "#
    );
}

#[test]
fn cargo_toml_ignore_block_before() {
    check(
        CargoToml,
        r#"
# keepsorted: ignore block
[dependencies]
b = "2"
a = "1"

[dev-dependencies]
y = "4"
x = "3"
        "#,
        r#"
# keepsorted: ignore block
[dependencies]
b = "2"
a = "1"

[dev-dependencies]
x = "3"
y = "4"
        "#
    );
}

#[test]
fn cargo_toml_nested_list() {
    check(
        CargoToml,
        r#"
[dependencies]
b = { workspace = true, default-features = false, features = [
    "z",
    "y",
    "x",
] } # some comment.
a = "1"
        "#,
        r#"
[dependencies]
a = "1"
b = { workspace = true, default-features = false, features = [
    "z",
    "y",
    "x",
] } # some comment.
        "#
    );
}

#[test]
fn cargo_toml_features() {
    check(
        CargoToml,
        r#"
[dependencies]
workspace = true
features = [
  # lots of features here
  "a", "b", "c",
  "d", # comment
  "e",
  # trailing comment
]
        "#,
        r#"
[dependencies]
features = [
  # lots of features here
  "a", "b", "c",
  "d", # comment
  "e",
  # trailing comment
]
workspace = true
        "#
    );
}

#[test]
fn cargo_toml_hash_in_string() {
    check(
        CargoToml,
        r#"
[package]
authors = ["Name # Surname"]
version = "1.0"
        "#,
        r#"
[package]
authors = ["Name # Surname"]
version = "1.0"
        "#
    );
}

#[test]
fn cargo_toml_git_url_with_hash() {
    check(
        CargoToml,
        r#"
[dependencies]
dep_b = "1.0"
dep_a = { git = "https://example.com/foo#bar" }
        "#,
        r#"
[dependencies]
dep_a = { git = "https://example.com/foo#bar" }
dep_b = "1.0"
        "#
    );
}

#[test]
fn cargo_toml_bracket_in_inline_comment() {
    // A dep with '[' only in its comment must not trigger multiline mode,
    // causing the next dependency to be swallowed into the same item.
    check(
        CargoToml,
        r#"
[dependencies]
b = "2" # see [dev-dependencies] for the test version
a = "1"
        "#,
        r#"
[dependencies]
a = "1"
b = "2" # see [dev-dependencies] for the test version
        "#
    );
}

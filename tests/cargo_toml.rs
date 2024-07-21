#[macro_use]
mod common;

use keepsorted::SortStrategy::CargoToml;

#[test]
fn cargo_toml_simple() {
    test_inner!(
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
    test_inner!(
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
    test_inner!(
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
    test_inner!(
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
    test_inner!(
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
    test_inner!(
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
#[ignore]
fn cargo_toml_nested_list() {
    test_inner!(
        CargoToml,
        r#"
[dependencies]
b = { workspace = true, default-features = false, features = [
    "z",
    "y",
    "x",
] }
a = "1"
        "#,
        r#"
[dependencies]
a = "1"
b = { workspace = true, default-features = false, features = [
    "z",
    "y",
    "x",
] }
        "#
    );
}

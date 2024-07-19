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

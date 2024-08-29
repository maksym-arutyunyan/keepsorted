#[macro_use]
mod common;

use keepsorted::Strategy::RustDeriveAlphabetical;

#[test]
fn rust_derive_simple() {
    test_inner!(
        RustDeriveAlphabetical,
        r#"
#[derive(b, c, a)]
struct Data {}
        "#,
        r#"
#[derive(a, b, c)]
struct Data {}
        "#
    );
}

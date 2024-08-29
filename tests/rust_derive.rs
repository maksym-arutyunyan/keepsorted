#[macro_use]
mod common;

use keepsorted::Strategy::{RustDeriveAlphabetical, RustDeriveCanonical};

#[test]
fn rust_derive_alphabetical() {
    test_inner!(
        RustDeriveAlphabetical,
        r#"
#[derive(C, B, A, Ord, Copy)]
struct Data {}
        "#,
        r#"
#[derive(A, B, C, Copy, Ord)]
struct Data {}
        "#
    );
}

#[test]
fn rust_derive_canonical() {
    test_inner!(
        RustDeriveCanonical,
        r#"
#[derive(C, B, A, Ord, Copy)]
struct Data {}
        "#,
        r#"
#[derive(Copy, Ord, A, B, C)]
struct Data {}
        "#
    );
}

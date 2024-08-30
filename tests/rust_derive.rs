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

//       1         2         3         4         5         6         7         8         9
//34567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567
#[test]
fn rust_derive_long_1() {
    test_inner!(
        RustDeriveAlphabetical,
        //         2         3         4         5         6         7         8         9
        //123456789012345678901234567890123456789012345678901234567890123456789012345678901234567
        r#"
#[derive(A01, A02, A03, A04, A05, A06, A07, A08, A09, A10, A11, A12, A13, A14, A15, A16, A17xx)]
struct Data {}
        "#,
        r#"
#[derive(A01, A02, A03, A04, A05, A06, A07, A08, A09, A10, A11, A12, A13, A14, A15, A16, A17xx)]
struct Data {}
        "#
    );
}

//       1         2         3         4         5         6         7         8         9
//34567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567
#[test]
fn rust_derive_long_2() {
    test_inner!(
        RustDeriveAlphabetical,
        //         2         3         4         5         6         7         8         9
        //123456789012345678901234567890123456789012345678901234567890123456789012345678901234567
        r#"
#[derive(A01, A02, A03, A04, A05, A06, A07, A08, A09, A10, A11, A12, A13, A14, A15, A16, A17xxx)]
struct Data {}
        "#,
        r#"
#[derive(
    A01, A02, A03, A04, A05, A06, A07, A08, A09, A10, A11, A12, A13, A14, A15, A16, A17xxx,
)]
struct Data {}
        "#
    );
}

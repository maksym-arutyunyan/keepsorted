use super::common::check;
use crate::Strategy::{RustDeriveAlphabetical, RustDeriveCanonical};

#[test]
fn rust_derive_alphabetical() {
    check(
        RustDeriveAlphabetical,
        r#"
#[derive(serde::Serialize, C, B, A, Ord, Copy, c, b, a, Serialize)]
struct Data {}
        "#,
        r#"
#[derive(A, B, C, Copy, Ord, Serialize, serde::Serialize, a, b, c)]
struct Data {}
        "#,
    );
}

#[test]
fn rust_derive_canonical() {
    check(
        RustDeriveCanonical,
        r#"
#[derive(serde::Serialize, C, B, A, Ord, Copy, c, b, a, Serialize)]
struct Data {}
        "#,
        r#"
#[derive(Copy, Ord, A, B, C, Serialize, serde::Serialize, a, b, c)]
struct Data {}
        "#,
    );
}

#[test]
fn rust_derive_alphabetical_indented() {
    check(
        RustDeriveAlphabetical,
        r#"
mod foo {
    #[derive(C, B, A, Ord, Copy)]
    struct Data {}
}
        "#,
        r#"
mod foo {
    #[#[derive(A, B, C, Copy, Ord)]
    struct Data {}
}
        "#,
    );
}

#[test]
fn rust_derive_canonical_indented() {
    check(
        RustDeriveCanonical,
        r#"
mod foo {
    #[derive(C, B, A, Ord, Copy)]
    struct Data {}
}
        "#,
        r#"
mod foo {
    #[derive(Copy, Ord, A, B, C)]
    struct Data {}
}
        "#,
    );
}

//       1         2         3         4         5         6         7         8         9
//3456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456
#[test]
fn rust_derive_long_stays_one_line() {
    check(
        RustDeriveAlphabetical,
        //         2         3         4         5         6         7         8         9
        //12345678901234567890123456789012345678901234567890123456789012345678901234567890123456
        r#"
#[derive(A01, A02, A03, A04, A05, A06, A07, A08, A09, A10, A11, A12, A13, A14, A15, A16, A17xx)]
struct Data {}
        "#,
        r#"
#[derive(A01, A02, A03, A04, A05, A06, A07, A08, A09, A10, A11, A12, A13, A14, A15, A16, A17xx)]
struct Data {}
        "#,
    );
}

//       1         2         3         4         5         6         7         8         9
//34567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567
#[test]
fn rust_derive_long_breaks_into_three_lines() {
    check(
        RustDeriveAlphabetical,
        //         2         3         4         5         6         7         8         9
        //123456789012345678901234567890123456789012345678901234567890123456789012345678901234567
        r#"
#[derive(
    A01, A02, A03, A04, A05, A06, A07, A08, A09, A10, A11, A12, A13, A14, A15, A16, A17xxx,
)]
struct Data {}
        "#,
        r#"
#[derive(
    A01, A02, A03, A04, A05, A06, A07, A08, A09, A10, A11, A12, A13, A14, A15, A16, A17xxx,
)]
struct Data {}
        "#,
    );
}

//       1         2         3         4         5         6         7         8         9         0
//345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901
#[test]
fn rust_derive_long_stays_three_lines() {
    check(
        RustDeriveAlphabetical,
        //         2         3         4         5         6         7         8         9         0
        //1234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901
        r#"
#[derive(
    A01, A02, A03, A04, A05, A06, A07, A08, A09, A10, A11, A12, A13, A14, A15, A16, A17xx, B01, B02x,
)]
struct Data {}
        "#,
        r#"
#[derive(
    A01, A02, A03, A04, A05, A06, A07, A08, A09, A10, A11, A12, A13, A14, A15, A16, A17xx, B01, B02x,
)]
struct Data {}
        "#,
    );
}

//       1         2         3         4         5         6         7         8         9         0
//3456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012
#[test]
fn rust_derive_long_breaks_into_many_lines() {
    check(
        RustDeriveAlphabetical,
        //         2         3         4         5         6         7         8         9         0
        //12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012
        r#"
#[derive(
    A01,
    A02,
    A03,
    A04,
    A05,
    A06,
    A07,
    A08,
    A09,
    A10,
    A11,
    A12,
    A13,
    A14,
    A15,
    A16,
    A17xx,
    B01,
    B02xx,
)]
struct Data {}
        "#,
        r#"
#[derive(
    A01,
    A02,
    A03,
    A04,
    A05,
    A06,
    A07,
    A08,
    A09,
    A10,
    A11,
    A12,
    A13,
    A14,
    A15,
    A16,
    A17xx,
    B01,
    B02xx,
)]
struct Data {}
        "#,
    );
}

#[test]
fn rust_derive_one_line_ignored() {
    check(
        RustDeriveAlphabetical,
        r#"
// keepsorted: ignore block
#[derive(C, B, A, Ord, Copy)]
struct Data {}
        "#,
        r#"
// keepsorted: ignore block
#[derive(C, B, A, Ord, Copy)]
struct Data {}
        "#,
    );
}

#[test]
fn rust_derive_three_lines_ignored() {
    check(
        RustDeriveAlphabetical,
        r#"
// keepsorted: ignore block
#[derive(
    C, B, A, Ord, Copy,
)]
struct Data {}
        "#,
        r#"
// keepsorted: ignore block
#[derive(
    C, B, A, Ord, Copy,
)]
struct Data {}
        "#,
    );
}

#[test]
fn rust_derive_many_lines_ignored() {
    check(
        RustDeriveAlphabetical,
        r#"
// keepsorted: ignore block
#[derive(
    C,
    B,
    A,
    Ord,
    Copy,
)]
struct Data {}
        "#,
        r#"
// keepsorted: ignore block
#[derive(
    C,
    B,
    A,
    Ord,
    Copy,
)]
struct Data {}
        "#,
    );
}

#[test]
fn rust_derive_issue_25_1() {
    check(
        RustDeriveAlphabetical,
        r#"
#[derive(Debug, Parser)] // Some comment.
struct Data {}
        "#,
        r#"
#[derive(Debug, Parser)] // Some comment.
struct Data {}
        "#,
    );
}

#[test]
fn rust_derive_issue_25_2() {
    check(
        RustDeriveAlphabetical,
        r#"
#[derive(Debug, Parser)] // Some comment.
#[command(about = "description", long_about = None)]
struct Data {}
        "#,
        r#"
#[derive(Debug, Parser)] // Some comment.
#[command(about = "description", long_about = None)]
struct Data {}
        "#,
    );
}

#[test]
fn rust_derive_issue_25_3() {
    check(
        RustDeriveAlphabetical,
        r#"
#[derive(Debug, Parser)] // Some comment comment with #[derive(Parser, Debug)].
#[command(about = "description", long_about = None)]
struct Data {}
        "#,
        r#"
#[derive(Debug, Parser)] // Some comment comment with #[derive(Parser, Debug)].
#[command(about = "description", long_about = None)]
struct Data {}
        "#,
    );
}

#[test]
fn rust_both_derive_and_generic_sort() {
    check(
        RustDeriveAlphabetical,
        r#"
fn setup_systems(app: &mut App) {
    app.add_plugins((
        // keepsorted: keep sorted
        a,
        b,
    ));
}

#[derive(Copy, Clone)]
struct Data {}
        "#,
        r#"
fn setup_systems(app: &mut App) {
    app.add_plugins((
        // keepsorted: keep sorted
        a,
        b,
    ));
}

#[derive(Copy, Clone)]
struct Data {}
        "#,
    );
}

#[test]
fn rust_derive_with_url() {
    check(
        RustDeriveAlphabetical,
        r#"
#[derive(A, B, Note = "http://example.com")]
struct Data {}
        "#,
        r#"
#[derive(A, B, Note = "http://example.com")]
struct Data {}
        "#,
    );
}

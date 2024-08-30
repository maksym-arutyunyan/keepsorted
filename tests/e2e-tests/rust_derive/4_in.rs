//       1         2         3         4         5         6         7         8         9
//3456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456
#[derive(A01, A02, A03, A04, A05, A06, A07, A08, A09, A10, A11, A12, A13, A14, A15, A16, A17xx)]
struct Data {}

#[derive(
    A01, A02, A03, A04, A05, A06, A07, A08, A09, A10, A11, A12, A13, A14, A15, A16, A17xxx,
)]
struct Data {}

//       1         2         3         4         5         6         7         8         9
//3456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456
mod foo {
    #[derive(A01, A02, A03, A04, A05, A06, A07, A08, A09, A10, A11, A12, A13, A14, A15, A16xxx)]
    struct Data {}

    #[derive(
        A01, A02, A03, A04, A05, A06, A07, A08, A09, A10, A11, A12, A13, A14, A15, A16xxxx,
    )]
    struct Data {}
}

//       1         2         3         4         5         6         7         8         9
//3456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456
mod foo {
    mod bar {
        #[derive(A01, A02, A03, A04, A05, A06, A07, A08, A09, A10, A11, A12, A13, A14, A15xxxx)]
        struct Data {}

        #[derive(
            A01, A02, A03, A04, A05, A06, A07, A08, A09, A10, A11, A12, A13, A14, A15xxxxx,
        )]
        struct Data {}
    }
}

//       1         2         3         4         5         6         7         8         9         0
//345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901
#[derive(
    A01, A02, A03, A04, A05, A06, A07, A08, A09, A10, A11, A12, A13, A14, A15, A16, A17xx, B01, B02x,
)]
struct Data {}

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

//       1         2         3         4         5         6         7         8         9
//34567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890
#[derive(Axxxxxxx01, Axxxxxxx02, Axxxxxxx03, Axxxxxxx04, Axxxxxxx05, Axxxxxxx06, Axxxxxxx07eee)]
struct Data {}

#[derive(
    Axxxxxxx01, Axxxxxxx02, Axxxxxxx03, Axxxxxxx04, Axxxxxxx05, Axxxxxxx06, Axxxxxxx07eeee,
)]
struct Data {}

//       1         2         3         4         5         6         7         8         9         0
//345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901
#[derive(
    Axxxxxxx01, Axxxxxxx02, Axxxxxxx03, Axxxxxxx04, Axxxxxxx05, Axxxxxxx06, Axxxxxxx07, Axxxxxxx08ee,
)]
struct Data {}

#[derive(
    Axxxxxxx01,
    Axxxxxxx02,
    Axxxxxxx03,
    Axxxxxxx04,
    Axxxxxxx05,
    Axxxxxxx06,
    Axxxxxxx07,
    Axxxxxxx08eee,
)]
struct Data {}

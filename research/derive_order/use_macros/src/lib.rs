#[cfg(test)]
mod tests {
    use derive_macros::{First, Second};

    trait First {
        fn is_first_implemented() -> bool;
    }

    trait Second {
        fn check_first_is_implemented() -> bool;
    }

    #[test]
    fn test_first_second() {
        #[derive(First, Second)] // <-- This should work correctly.
        struct MyStruct;

        assert!(<MyStruct as Second>::check_first_is_implemented());
    }

    #[test]
    fn test_second_first() {
        #[derive(Second, First)] // <-- Clarify if this should work or fail.
        struct MyStruct;

        assert!(<MyStruct as Second>::check_first_is_implemented());
    }
}

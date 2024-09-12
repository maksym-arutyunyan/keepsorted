// This test file checks if the order of Rust derive traits matters.
// Since the keepsorted tool could reorder these traits in CI, we must ignore this file.
// keepsorted: ignore file

#[cfg(test)]
mod tests {
    use derive_macros::{First, Second};

    trait First {
        fn is_first_implemented() -> bool;
    }

    trait Second: First {
        fn check_first_is_implemented() -> bool;
    }

    #[test]
    fn test_first_second() {
        #[derive(First, Second)] // <- This order should work.
        struct MyStruct;

        assert!(MyStruct::check_first_is_implemented());
    }

    #[test]
    fn test_second_first() {
        #[derive(Second, First)] // <- This order might not work.
        struct MyStruct;

        assert!(MyStruct::check_first_is_implemented());
    }
}

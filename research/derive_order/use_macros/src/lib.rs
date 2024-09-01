// keepsorted: ignore file

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
        #[derive(First, Second)] // <- This is expected to work.
        struct MyStruct;

        assert!(MyStruct::check_first_is_implemented());
    }

    #[test]
    fn test_second_first() {
        #[derive(Second, First)] // <- Seems the order is not important.
        struct MyStruct;

        assert!(MyStruct::check_first_is_implemented());
    }
}

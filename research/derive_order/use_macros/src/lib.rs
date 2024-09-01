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
        #[derive(First, Second)]
        struct MyStruct;

        assert!(MyStruct::check_first_is_implemented());
    }

    #[test]
    fn test_second_first() {
        #[derive(Second, First)]
        struct MyStruct;

        assert!(MyStruct::check_first_is_implemented());
    }
}

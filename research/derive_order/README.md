# Rust Derive Order Example

This project shows how the order of Rust derive traits can impact a struct's behavior. 
It includes two custom procedural macros `First` and `Second`.

## Structure

- **derive_macros/**: Defines the `First` and `Second` macros
- **use_macros/**: Contains tests to verify the behavior of the macros

## Tests

Two tests demonstrate the importance of derive order:

1. **`test_first_second`**: `First` is derived before `Second`. This should pass.
2. **`test_second_first`**: `Second` is derived before `First`. This may fail, but it actually passes. **Seems like the derive order does not matter!**

### Run Tests

To run the tests:

```shell
cargo test -p use_macros -- --nocapture
```

Output:

```shell
running 2 tests
First is implemented, so Second works!
First is implemented, so Second works!
test tests::test_second_first ... ok
test tests::test_first_second ... ok
```

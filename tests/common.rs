use keepsorted::{process_lines, Strategy};
use std::io::{self};

// Helper function to hide text-lines conversion.
pub fn process_input(strategy: Strategy, text: &str) -> io::Result<String> {
    let lines: Vec<String> = text.lines().map(String::from).collect();
    let processed_lines = process_lines(strategy, lines)?;
    Ok(processed_lines.join("\n"))
}

// Macro for defining the core test logic.
#[macro_export]
macro_rules! test_inner {
    ($strategy:expr, $input:expr, $expected:expr) => {{
        let strategy = $strategy;
        let input = $input;
        let expected = $expected;
        let result = common::process_input(strategy, input).unwrap();
        assert!(
            result == expected,
            "Expected: {}\nActual: {}",
            expected,
            result
        );
    }};
}

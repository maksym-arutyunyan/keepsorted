use keepsorted::{process_lines, Strategy};
use std::io::{self};

// Helper function to hide text-lines conversion.
pub fn process_input(strategy: Strategy, text: &str) -> io::Result<String> {
    let lines: Vec<_> = text.lines().map(|line| format!("{}\n", line)).collect();
    let mut processed_lines = process_lines(strategy, lines)?;
    if let Some(last) = processed_lines.last_mut() {
        last.truncate(last.trim_end_matches('\n').len());
    }
    Ok(processed_lines.concat())
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

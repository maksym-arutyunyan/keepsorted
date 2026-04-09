use crate::{process_lines, Strategy};

pub fn check(strategy: Strategy, input: &str, expected: &str) {
    let lines: Vec<_> = input.lines().map(|l| format!("{l}\n")).collect();
    let mut result = process_lines(strategy, lines).unwrap();
    if let Some(last) = result.last_mut() {
        *last = last.trim_end_matches('\n').to_string();
    }
    assert_eq!(result.concat(), expected);
}

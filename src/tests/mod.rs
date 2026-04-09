mod bazel;
mod cargo_toml;
mod generic;
mod gitignore;
mod lib;
mod rust_derive;
mod common;


// use crate::{process_lines, Strategy};
// use std::io;

// pub(super) fn process_input(strategy: Strategy, text: &str) -> io::Result<String> {
//     let lines: Vec<_> = text.lines().map(|line| format!("{}\n", line)).collect();
//     let mut processed_lines = process_lines(strategy, lines)?;
//     if let Some(last) = processed_lines.last_mut() {
//         last.truncate(last.trim_end_matches('\n').len());
//     }
//     Ok(processed_lines.concat())
// }

// #[macro_export]
// macro_rules! test_inner {
//     ($strategy:expr, $input:expr, $expected:expr) => {{
//         let result = super::common::process_input($strategy, $input).unwrap();
//         assert!(
//             result == $expected,
//             "Expected:\n{}\nActual:\n{}",
//             $expected,
//             result
//         );
//     }};
// }

use crate::Strategy;
use once_cell::sync::Lazy;
use regex::Regex;
use std::io;

use crate::is_ignore_block;

static RE_DERIVE_BEGIN: Lazy<Regex> = Lazy::new(re_derive_begin);
static RE_DERIVE_END: Lazy<Regex> = Lazy::new(re_derive_end);

// These values count the number of characters and an extra '\n'.
const STAY_ONE_LINE_LEN: usize = 97;
const BREAK_INTO_MANY_LINES_LEN: usize = 101;

pub(crate) fn process(lines: Vec<String>, strategy: Strategy) -> io::Result<Vec<String>> {
    let mut output_lines: Vec<String> = Vec::new();
    let mut block = Vec::new();
    let mut is_sorting_block = false;
    let mut is_ignore_block_prev_line = false;

    for line in lines {
        let mut is_derive_begin = false;
        if RE_DERIVE_BEGIN.is_match(&line) {
            if let Some(prev_line) = output_lines.last() {
                is_ignore_block_prev_line = is_ignore_block(&[prev_line.clone()]);
            }
            is_derive_begin = true;
            is_sorting_block = true;
            block.push(line.clone());
        }
        if is_sorting_block && RE_DERIVE_END.is_match(&line) {
            if !is_derive_begin {
                block.push(line.clone());
            }
            block = sort(block, is_ignore_block_prev_line, strategy);
            is_ignore_block_prev_line = false;
            is_sorting_block = false;
            output_lines.append(&mut block);
        } else if is_sorting_block {
            if !is_derive_begin {
                block.push(line);
            }
        } else {
            output_lines.push(line);
        }
    }

    if is_sorting_block {
        block = sort(block, is_ignore_block_prev_line, strategy);
        output_lines.append(&mut block);
    }

    Ok(output_lines)
}

fn sort(block: Vec<String>, is_ignore_block_prev_line: bool, strategy: Strategy) -> Vec<String> {
    if is_ignore_block_prev_line || is_ignore_block(&block) {
        return block;
    }
    let line: String = block
        .iter()
        .map(|line| line.trim_end_matches('\n'))
        .collect();
    let line = format!("{}\n", line);
    let trimmed_line = line.trim();

    let mut result = Vec::new();
    // Check if the line contains a #[derive(...)] statement
    if let Some(derive_start) = trimmed_line.find("#[derive(") {
        if let Some(derive_end) = trimmed_line[derive_start..].find(")]") {
            let derive_content = &trimmed_line[derive_start + 9..derive_start + derive_end];
            let mut traits: Vec<&str> = derive_content.split(',').map(str::trim).collect();

            match strategy {
                Strategy::RustDeriveAlphabetical => {
                    traits.sort_unstable();
                }
                Strategy::RustDeriveCanonical => {
                    traits = canonical_sort(traits);
                }
                _ => (),
            }
            traits.retain(|t| !t.is_empty());
            let sorted_traits = traits.join(", ");
            let new_derive = format!("#[derive({})]", sorted_traits);

            // Reconstruct the line with preserved whitespaces
            let prefix_whitespace = &line[..line.find(trimmed_line).unwrap_or(0)];
            let suffix_whitespace =
                &line[line.rfind(trimmed_line).unwrap_or(line.len()) + trimmed_line.len()..];

            let new_line = format!("{}{}{}", prefix_whitespace, new_derive, suffix_whitespace);
            if new_line.len() <= STAY_ONE_LINE_LEN {
                result.push(new_line);
            } else {
                let mid_line = format!("{}    {},", prefix_whitespace, sorted_traits);
                if mid_line.len() <= BREAK_INTO_MANY_LINES_LEN {
                    result.push(format!("{}#[derive(\n", prefix_whitespace));
                    result.push(format!("{}\n", mid_line));
                    result.push(format!("{})]\n", prefix_whitespace));
                } else {
                    result.push(format!("{}#[derive(\n", prefix_whitespace));
                    for x in traits {
                        result.push(format!("{}    {},\n", prefix_whitespace, x));
                    }
                    result.push(format!("{})]\n", prefix_whitespace));
                }
            }
        }
    }

    result
}

fn canonical_sort(traits: Vec<&str>) -> Vec<&str> {
    // Define the canonical order of traits
    let canonical_traits = [
        "Copy",
        "Clone",
        "Eq",
        "PartialEq",
        "Ord",
        "PartialOrd",
        "Hash",
        "Debug",
        "Display",
        "Default",
    ];

    // Partition traits into canonical and non-canonical
    let mut canonical_sorted: Vec<&str> = traits
        .iter()
        .filter(|&&t| canonical_traits.contains(&t))
        .cloned()
        .collect();
    let mut non_canonical_sorted: Vec<&str> = traits
        .iter()
        .filter(|&&t| !canonical_traits.contains(&t))
        .cloned()
        .collect();

    // Sort canonical traits by the predefined order
    canonical_sorted.sort_by_key(|t| canonical_traits.iter().position(|&ct| ct == *t).unwrap());

    // Sort non-canonical traits alphabetically
    non_canonical_sorted.sort_unstable();

    // Combine the two sorted lists
    canonical_sorted.extend(non_canonical_sorted);

    canonical_sorted
}

fn re_derive_begin() -> Regex {
    Regex::new(r"^\s*#\[derive\(").expect("Failed to build regex for rust derive begin")
}

fn re_derive_end() -> Regex {
    Regex::new(r"\)\]\s*$").expect("Failed to build regex for rust derive end")
}
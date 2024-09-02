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
        let line_without_comment = line.trim().split("//").next().unwrap_or("").trim();
        if is_sorting_block && RE_DERIVE_END.is_match(line_without_comment) {
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
    let line_without_comment = line.trim().split("//").next().unwrap_or("").trim();

    // Check if the line contains a #[derive(...)] statement
    if let Some(derive_range) = line_without_comment
        .find("#[derive(")
        .map(|start| {
            let end = line_without_comment[start..].find(")]")?;
            Some(start + 9..start + end)
        })
        .flatten()
    {
        let derive_content = &line_without_comment[derive_range.clone()];
        let mut traits: Vec<&str> = derive_content
            .split(',')
            .map(str::trim)
            .filter(|t| !t.is_empty())
            .collect();

        match strategy {
            Strategy::RustDeriveAlphabetical => traits.sort_unstable(),
            Strategy::RustDeriveCanonical => traits = canonical_sort(traits),
            _ => return block,
        }

        let sorted_traits = traits.join(", ");
        let new_derive = format!("#[derive({})]", sorted_traits);

        // Preserve the prefix and suffix whitespace
        let prefix_whitespace = &line[..line.find(line_without_comment).unwrap_or(0)];
        let suffix_whitespace =
            &line[line_without_comment.len() + line.find(line_without_comment).unwrap_or(0)..];

        let new_line = format!("{}{}{}", prefix_whitespace, new_derive, suffix_whitespace);
        if new_line.len() <= STAY_ONE_LINE_LEN {
            return vec![new_line];
        }

        let mid_line = format!("{}    {},", prefix_whitespace, sorted_traits);
        let mut result = vec![format!("{}#[derive(\n", prefix_whitespace)];

        if mid_line.len() <= BREAK_INTO_MANY_LINES_LEN {
            result.push(format!("{}\n{})]\n", mid_line, prefix_whitespace));
        } else {
            for trait_item in traits {
                result.push(format!("{}    {},\n", prefix_whitespace, trait_item));
            }
            result.push(format!("{})]\n", prefix_whitespace));
        }

        return result;
    }

    block
}

fn canonical_sort(traits: Vec<&str>) -> Vec<&str> {
    // Define the canonical order of traits
    let canonical_order = [
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

    // Create a mapping from trait to its canonical index
    let canonical_index: std::collections::HashMap<_, _> = canonical_order
        .iter()
        .enumerate()
        .map(|(i, &trait_name)| (trait_name, i))
        .collect();

    // Sort traits by canonical index, and by trait name if indices are the same
    let mut sorted_traits = traits;
    sorted_traits.sort_by(|a, b| {
        let index_a = canonical_index.get(a).unwrap_or(&usize::MAX);
        let index_b = canonical_index.get(b).unwrap_or(&usize::MAX);
        (index_a, a).cmp(&(index_b, b))
    });

    sorted_traits
}

fn re_derive_begin() -> Regex {
    Regex::new(r"^\s*#\[derive\(").expect("Failed to build regex for rust derive begin")
}

fn re_derive_end() -> Regex {
    Regex::new(r"\)\]\s*$").expect("Failed to build regex for rust derive end")
}

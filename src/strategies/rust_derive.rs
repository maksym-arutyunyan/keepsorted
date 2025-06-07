use crate::TraitGroups;
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;
use std::{collections::HashMap, io};

use crate::is_ignore_block;

static RE_DERIVE_BEGIN: Lazy<Regex> = Lazy::new(re_derive_begin);
static RE_DERIVE_END: Lazy<Regex> = Lazy::new(re_derive_end);

// These values count the number of characters and an extra '\n'.
const STAY_ONE_LINE_LEN: usize = 97;
const BREAK_INTO_MANY_LINES_LEN: usize = 101;

const CANONICAL_TRAITS: &[&str] = &[
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

#[derive(Copy, Clone)]
pub enum RustDeriveStrategy {
    Alphabetical,
    Canonical,
}

pub(crate) fn process(
    lines: Vec<String>,
    strategy: RustDeriveStrategy,
    groups: Option<&TraitGroups>,
) -> io::Result<Vec<String>> {
    let mut output_lines: Vec<String> = Vec::new();
    let mut block = Vec::new();
    let mut is_sorting_block = false;
    let mut is_ignore_block_prev_line = false;

    let priority_index = match strategy {
        RustDeriveStrategy::Alphabetical => build_priority_index(&[], groups),
        RustDeriveStrategy::Canonical => build_priority_index(CANONICAL_TRAITS, groups),
    };

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
            block = sort(block, is_ignore_block_prev_line, &priority_index);
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
        block = sort(block, is_ignore_block_prev_line, &priority_index);
        output_lines.append(&mut block);
    }

    Ok(output_lines)
}

fn sort(
    block: Vec<String>,
    is_ignore_block_prev_line: bool,
    priority_index: &HashMap<&str, usize>,
) -> Vec<String> {
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
    if let Some(derive_range) = line_without_comment.find("#[derive(").and_then(|start| {
        let end = line_without_comment[start..].find(")]")?;
        Some(start + 9..start + end)
    }) {
        let derive_content = &line_without_comment[derive_range.clone()];
        let mut traits: Vec<&str> = derive_content
            .split(',')
            .map(str::trim)
            .filter(|t| !t.is_empty())
            .collect();

        traits = priority_sort(traits, priority_index);

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

fn extract_last_token(s: &str) -> &str {
    s.split("::").last().unwrap_or(s)
}

fn priority_sort<'a>(traits: Vec<&'a str>, priority_index: &HashMap<&str, usize>) -> Vec<&'a str> {
    // Sort traits by priority index, and by trait name if indices are the same
    let mut sorted_traits = traits;
    sorted_traits.sort_by(|a, b| {
        let index_a = priority_index.get(a).unwrap_or(&usize::MAX);
        let index_b = priority_index.get(b).unwrap_or(&usize::MAX);
        (index_a, extract_last_token(a), a).cmp(&(index_b, extract_last_token(b), b))
    });

    sorted_traits
}

fn build_priority_index<'a>(
    priority_traits: &[&'a str],
    groups: Option<&'a TraitGroups>,
) -> HashMap<&'a str, usize> {
    if let Some(groups) = groups {
        let grouped_traits: Vec<&'a str> = groups
            .keys()
            .sorted()
            .flat_map(|key| groups.get(key).unwrap().iter().sorted())
            .map(String::as_str)
            .collect();
        priority_traits
            .iter()
            .copied()
            .chain(grouped_traits)
            .enumerate()
            .map(|(i, trait_name)| (trait_name, i))
            .collect()
    } else {
        priority_traits
            .iter()
            .copied()
            .enumerate()
            .map(|(i, trait_name)| (trait_name, i))
            .collect()
    }
}

fn re_derive_begin() -> Regex {
    Regex::new(r"^\s*#\[derive\(").expect("Failed to build regex for rust derive begin")
}

fn re_derive_end() -> Regex {
    Regex::new(r"\)\]\s*$").expect("Failed to build regex for rust derive end")
}

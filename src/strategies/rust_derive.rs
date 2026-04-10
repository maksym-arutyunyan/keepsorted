use crate::{Strategy, RE_KEEP_SORTED};
use regex::Regex;
use std::io;
use std::sync::LazyLock;

use super::common::split_code_and_line_comment;
use crate::{is_ignore_block, is_ignore_block_line};

static RE_DERIVE_BEGIN: LazyLock<Regex> = LazyLock::new(re_derive_begin);
static RE_DERIVE_END: LazyLock<Regex> = LazyLock::new(re_derive_end);

// Max line width (characters, excluding '\n') for rustfmt-compatible derive formatting.
// At or below 96: keep on a single line.
// At or below 101: use three-line form (#[derive(\n    ...,\n)]).
// Above 101: expand to one trait per line.
const MAX_SINGLE_LINE_WIDTH: usize = 96;
const MAX_THREE_LINE_WIDTH: usize = 101;

pub(crate) type ProcessResult = (Vec<String>, bool);

pub(crate) fn process(lines: Vec<String>, strategy: Strategy) -> io::Result<ProcessResult> {
    let mut output_lines: Vec<String> = Vec::new();
    let mut block = Vec::new();
    let mut is_sorting_block = false;
    let mut is_ignore_block_prev_line = false;
    let mut requires_generic_sort = false;

    for line in lines {
        // if we see at least on match, we also need to apply generic sort
        if !requires_generic_sort && RE_KEEP_SORTED.is_match(&line) {
            requires_generic_sort = true;
        }
        if RE_DERIVE_BEGIN.is_match(&line) {
            if let Some(prev_line) = output_lines.last() {
                is_ignore_block_prev_line = is_ignore_block_line(prev_line);
            }
            is_sorting_block = true;
        }
        let (line_without_comment, _comment) = split_code_and_line_comment(line.trim());
        let line_without_comment = line_without_comment.trim();
        if is_sorting_block && RE_DERIVE_END.is_match(line_without_comment) {
            block.push(line);
            block = sort(block, is_ignore_block_prev_line, strategy);
            is_ignore_block_prev_line = false;
            is_sorting_block = false;
            output_lines.append(&mut block);
        } else if is_sorting_block {
            block.push(line);
        } else {
            output_lines.push(line);
        }
    }

    if is_sorting_block {
        block = sort(block, is_ignore_block_prev_line, strategy);
        output_lines.append(&mut block);
    }

    Ok((output_lines, requires_generic_sort))
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
    let (code_raw, _comment) = split_code_and_line_comment(line.trim());
    let line_without_comment = code_raw.trim();

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

        match strategy {
            Strategy::RustDeriveAlphabetical => traits = alphabetical_sort(traits),
            Strategy::RustDeriveCanonical => traits = canonical_sort(traits),
            _ => return block,
        }

        let sorted_traits = traits.join(", ");
        let new_derive = format!("#[derive({})]", sorted_traits);

        // Compute prefix and suffix by measuring leading whitespace directly.
        // Using find() would be fragile if line_without_comment appeared earlier in line.
        let prefix_len = line.len() - line.trim_start().len();
        let prefix_whitespace = &line[..prefix_len];
        let suffix_whitespace = &line[prefix_len + line_without_comment.len()..];

        let new_line = format!("{}{}{}", prefix_whitespace, new_derive, suffix_whitespace);
        if new_line.trim_end_matches('\n').len() <= MAX_SINGLE_LINE_WIDTH {
            return vec![new_line];
        }

        let mid_line = format!("{}    {},", prefix_whitespace, sorted_traits);
        let mut result = vec![format!("{}#[derive(\n", prefix_whitespace)];

        if mid_line.len() <= MAX_THREE_LINE_WIDTH {
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

fn priority_sort<'a>(traits: Vec<&'a str>, priority_traits: &[&'a str]) -> Vec<&'a str> {
    // Create a mapping from trait to its priority index
    let priority_index: std::collections::HashMap<_, _> = priority_traits
        .iter()
        .enumerate()
        .map(|(i, &trait_name)| (trait_name, i))
        .collect();

    // Sort traits by priority index, and by trait name if indices are the same
    let mut sorted_traits = traits;
    sorted_traits.sort_by(|a, b| {
        let index_a = priority_index.get(a).unwrap_or(&usize::MAX);
        let index_b = priority_index.get(b).unwrap_or(&usize::MAX);
        (index_a, extract_last_token(a), a).cmp(&(index_b, extract_last_token(b), b))
    });

    sorted_traits
}

fn alphabetical_sort(traits: Vec<&str>) -> Vec<&str> {
    priority_sort(traits, &[])
}

fn canonical_sort(traits: Vec<&str>) -> Vec<&str> {
    priority_sort(
        traits,
        // Define the canonical order of traits
        &[
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
        ],
    )
}

fn re_derive_begin() -> Regex {
    Regex::new(r"^\s*#\[derive\(").expect("Failed to build regex for rust derive begin")
}

fn re_derive_end() -> Regex {
    Regex::new(r"\)\]\s*$").expect("Failed to build regex for rust derive end")
}

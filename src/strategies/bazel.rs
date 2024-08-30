use std::cmp::Ordering;
use std::io;

use crate::{is_ignore_block, re_keyword_keep_sorted};

pub(crate) fn process(lines: Vec<String>) -> io::Result<Vec<String>> {
    let re = re_keyword_keep_sorted();
    let mut output_lines = Vec::new();
    let mut block = Vec::new();
    let mut is_scope = false;
    let mut is_sorting_block = false;
    let mut is_ignore_block_prev_line = false;

    for line in lines {
        // Trim the input line
        let trimmed = line.trim();

        // Find and remove the portion of the line starting from the '#' character
        let line_without_comment = trimmed.split('#').next().unwrap_or("").trim();

        if line_without_comment.contains('[') {
            is_scope = true;
            output_lines.push(line);
        } else if is_scope {
            if re.is_match(&line) {
                if let Some(prev_line) = output_lines.last() {
                    is_ignore_block_prev_line = is_ignore_block(&[prev_line.clone()]);
                }
                is_sorting_block = true;
                output_lines.push(line);
            } else if is_sorting_block
                && (line_without_comment.contains(']') || line.trim().is_empty())
            {
                block = sort(block, is_ignore_block_prev_line);
                is_ignore_block_prev_line = false;
                is_sorting_block = false;
                output_lines.append(&mut block);
                output_lines.push(line);
            } else if is_sorting_block {
                block.push(line);
            } else {
                output_lines.push(line);
            }
        } else {
            output_lines.push(line);
        }
    }

    if is_sorting_block {
        block = sort(block, is_ignore_block_prev_line);
        output_lines.append(&mut block);
    }

    Ok(output_lines)
}

#[derive(Default)]
struct Item {
    comment: Vec<String>,
    code: String,
    sort_key: BazelSortKey,
}

/// Sorts a block of lines, keeping associated comments with their items.
fn sort(block: Vec<String>, is_ignore_block_prev_line: bool) -> Vec<String> {
    if is_ignore_block_prev_line || is_ignore_block(&block) {
        return block;
    }
    let n = block.len();
    let mut items = Vec::with_capacity(n);
    let mut current_item = Item::default();
    for line in block {
        if is_single_line_comment(&line) {
            current_item.comment.push(line);
        } else {
            items.push(Item {
                comment: std::mem::take(&mut current_item.comment),
                code: line.clone(),
                sort_key: BazelSortKey::new(&line),
            });
        }
    }
    let trailing_comments = current_item.comment;

    items.sort_by(|a, b| a.sort_key.cmp(&b.sort_key));

    let mut result = Vec::with_capacity(n);
    for item in items {
        result.extend(item.comment);
        result.push(item.code);
    }
    result.extend(trailing_comments);

    result
}

fn is_single_line_comment(line: &str) -> bool {
    line.trim().starts_with('#')
}

// From: https://sourcegraph.com/github.com/bazelbuild/buildtools@92a716d768c05fa90e241fd2c2b0411125a0ef89/-/blob/build/rewrite.go
//
// A stringSortKey records information about a single string literal to be
// sorted. The strings are first grouped into four phases: most strings,
// strings beginning with ":", strings beginning with "//", and strings
// beginning with "@". The next significant part of the comparison is the list
// of elements in the value, where elements are split at `.' and `:'. Finally
// we compare by value and break ties by original index.
#[derive(Eq, PartialEq, Debug, Default)]
pub struct BazelSortKey {
    phase: i16,
    split: Vec<String>,
}

impl BazelSortKey {
    pub(crate) fn new(line: &str) -> Self {
        let line_without_comment = line.trim().split('#').next().unwrap_or("").trim();

        let phase = match line_without_comment {
            l if l.starts_with("\":") => 1,
            l if l.starts_with("\"//") => 2,
            l if l.starts_with("\"@") => 3,
            l if l.starts_with('"') => 0,
            _ => 4,
        };

        let split = line_without_comment
            .split(|c| c == '.' || c == ':' || c == '"')
            .map(ToString::to_string)
            .collect();

        Self { phase, split }
    }
}

impl Ord for BazelSortKey {
    fn cmp(&self, other: &Self) -> Ordering {
        self.phase
            .cmp(&other.phase)
            .then_with(|| self.split.cmp(&other.split))
    }
}

impl PartialOrd for BazelSortKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bazel_sort_key() {
        let ordered_items = [
            r#""nested","#,
            r#"":aaa","#,
            r#"":bbb","#,
            r#""//dir/subdir:aaa","#,
            r#""//dir/subdir:bbb","#,
            r#""//dir/subdir/folder",  # Some in-line comment."#,
            r#""//dir/subdir/folder:xxx","#,
            r#""//dir/subdir/folder:yyy",  # TODO[yyy]"#,
            r#""@crate_index//:aaa","#,
            r#""@crate_index//:base","#,
            r#""@crate_index//:base32","#,
            r#""@crate_index//:base64-bytestring","#,
            r#""@crate_index//:bbb","#,
            r#""@crate_index//project","#,
            r#"requirement("gitpython"),"#,
            r#"requirement("python-gitlab"),"#,
            r#"requirement("pyyaml"),"#,
        ];
        for window in ordered_items.windows(2) {
            let left = BazelSortKey::new(window[0]);
            let right = BazelSortKey::new(window[1]);
            assert!(
                left <= right,
                "Sort order incorrect: {:?} > {:?}",
                left,
                right
            );
        }
    }
}

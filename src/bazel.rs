use crate::block::{sort, SortStrategy};
use regex::Regex;
use std::cmp::Ordering;
use std::io::{self};
use std::path::Path;

pub(crate) fn is_bazel(path: &Path) -> bool {
    match path.extension().and_then(|s| s.to_str()) {
        Some(ext) => matches!(ext, "bazel" | "bzl" | "BUILD" | "WORKSPACE"),
        None => false,
    }
}

pub(crate) fn process_lines_bazel(lines: Vec<&str>) -> io::Result<Vec<&str>> {
    let re = Regex::new(r"^\s*#\s*Keep\s*sorted\.\s*$")
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    let mut output_lines = Vec::new();
    let mut block = Vec::<&str>::new();
    let mut is_scope = false;
    let mut is_sorting_block = false;

    for line in lines {
        // Trim the input line
        let trimmed = line.trim();

        // Find and remove the portion of the line starting from the '#' character
        let line_without_comment = trimmed.split('#').next().unwrap_or("").trim();

        if line_without_comment.contains('[') {
            is_scope = true;
            output_lines.push(line);
        } else if is_scope {
            if re.is_match(line) {
                is_sorting_block = true;
                output_lines.push(line);
            } else if is_sorting_block
                && (line_without_comment.contains(']') || line.trim().is_empty())
            {
                is_sorting_block = false;
                sort(&mut block, SortStrategy::Bazel);
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
        sort(&mut block, SortStrategy::Bazel);
        output_lines.append(&mut block);
    }

    Ok(output_lines)
}

// From: https://sourcegraph.com/github.com/bazelbuild/buildtools@92a716d768c05fa90e241fd2c2b0411125a0ef89/-/blob/build/rewrite.go
//
// A stringSortKey records information about a single string literal to be
// sorted. The strings are first grouped into four phases: most strings,
// strings beginning with ":", strings beginning with "//", and strings
// beginning with "@". The next significant part of the comparison is the list
// of elements in the value, where elements are split at `.' and `:'. Finally
// we compare by value and break ties by original index.
#[derive(Debug, PartialEq, Eq)]
pub struct BazelSortKey<'a> {
    phase: i16,
    split: Vec<&'a str>,
}

impl<'a> BazelSortKey<'a> {
    pub(crate) fn new(line: &'a str) -> Self {
        let trimmed = line.trim();
        let line_without_comment = trimmed.split('#').next().unwrap_or("").trim();

        let phase = match line_without_comment {
            l if l.starts_with("\":") => 1,
            l if l.starts_with("\"//") => 2,
            l if l.starts_with("\"@") => 3,
            l if l.starts_with('"') => 0,
            _ => 4,
        };

        let split: Vec<&str> = line_without_comment
            .split(|c| c == '.' || c == ':' || c == '"')
            .collect();

        Self { phase, split }
    }
}

impl<'a> Ord for BazelSortKey<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.phase
            .cmp(&other.phase)
            .then_with(|| self.split.cmp(&other.split))
    }
}

impl<'a> PartialOrd for BazelSortKey<'a> {
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

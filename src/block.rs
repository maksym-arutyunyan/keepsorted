use std::cmp::Ordering;

#[derive(Debug)]
pub enum SortStrategy {
    Bazel,
    Default,
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
struct SortKey<'a> {
    phase: i16,
    split: Vec<&'a str>,
}

impl<'a> SortKey<'a> {
    fn new(line: &'a str) -> Self {
        // Trim the input line
        let trimmed = line.trim();

        // Find and remove the portion of the line starting from the '#' character
        let line_without_comment = trimmed.split('#').next().unwrap_or("").trim();

        // Determine the phase based on the beginning of the line
        let phase = if line_without_comment.starts_with("\":") {
            1
        } else if line_without_comment.starts_with("\"//") {
            2
        } else if line_without_comment.starts_with("\"@") {
            3
        } else if line_without_comment.starts_with('"') {
            0
        } else {
            4
        };

        // Split the line into components using '.' and ':' as delimiters
        let split: Vec<&str> = line_without_comment
            .split(|c| c == '.' || c == ':' || c == '"')
            .collect();

        // Create and return the SortKey instance
        Self { phase, split }
    }
}

impl<'a> Ord for SortKey<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.phase
            .cmp(&other.phase)
            .then_with(|| self.split.cmp(&other.split))
    }
}

impl<'a> PartialOrd for SortKey<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct LineGroup<'a> {
    comments: Vec<&'a str>,
    code: &'a str,
    sort_key: SortKey<'a>,
}

impl<'a> LineGroup<'a> {
    fn new() -> Self {
        Self {
            comments: Vec::new(),
            code: "",
            sort_key: SortKey::new(""),
        }
    }

    fn set_code(&mut self, line: &'a str) {
        self.code = line;
        self.sort_key = SortKey::new(line);
    }
}

fn is_single_line_comment(line: &str) -> bool {
    let trimmed = line.trim();
    ["#", "//"].iter().any(|&token| trimmed.starts_with(token))
}

pub fn sort(block: &mut [&str], strategy: SortStrategy) {
    let mut groups = Vec::new();
    let mut current_group = LineGroup::new();

    for &line in block.iter() {
        if is_single_line_comment(line) {
            current_group.comments.push(line);
        } else {
            current_group.set_code(line);
            groups.push(current_group);
            current_group = LineGroup::new();
        }
    }
    let trailing_comments = current_group.comments;

    match strategy {
        SortStrategy::Bazel => groups.sort_by(|a, b| a.sort_key.cmp(&b.sort_key)),
        _ => groups.sort_by(|a, b| a.code.cmp(b.code)),
    }

    let sorted_block: Vec<&str> = groups
        .into_iter()
        .flat_map(|group| {
            group
                .comments
                .into_iter()
                .chain(std::iter::once(group.code))
        })
        .chain(trailing_comments)
        .collect();

    block.copy_from_slice(&sorted_block);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let mut input = vec!["b", "a"];
        let expected = vec!["a", "b"];
        sort(&mut input, SortStrategy::Default);
        assert_eq!(input, expected);
    }

    #[test]
    fn sort_key() {
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
            let left = SortKey::new(window[0]);
            let right = SortKey::new(window[1]);
            assert!(
                left <= right,
                "Sort order incorrect: {:?} > {:?}",
                left,
                right
            );
        }
    }

    #[test]
    fn bazel_order() {
        let mut input = vec![
            r#"":bbb","#,
            r#"":aaa","#,
            r#""nested","#,
            r#""//dir/subdir/folder:yyy",  # TODO[yyy]"#,
            r#""//dir/subdir/folder:xxx","#,
            r#""//dir/subdir/folder",  # Some in-line comment."#,
            r#""//dir/subdir:bbb","#,
            r#""//dir/subdir:aaa","#,
            r#""@crate_index//project","#,
            r#""@crate_index//:base64-bytestring","#,
            r#""@crate_index//:base32","#,
            r#""@crate_index//:base","#,
            r#""@crate_index//:bbb","#,
            r#""@crate_index//:aaa","#,
            r#"requirement("gitpython"),"#,
            r#"requirement("python-gitlab"),"#,
            r#"requirement("pyyaml"),"#,
        ];
        let expected = vec![
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
        sort(&mut input, SortStrategy::Bazel);
        assert_eq!(input, expected);
    }

    #[test]
    fn with_inline_comment_bazel() {
        let mut input = vec!["y", "x  # Some in-line comment.", "b", "a"];
        let expected = vec!["a", "b", "x  # Some in-line comment.", "y"];
        sort(&mut input, SortStrategy::Default);
        assert_eq!(input, expected);
    }

    #[test]
    fn with_multi_line_comment_bazel() {
        let mut input = vec![
            "y",
            "# Some multi-line comment",
            "# for the line below.",
            "x",
            "b",
            "a",
        ];
        let expected = vec![
            "a",
            "b",
            "# Some multi-line comment",
            "# for the line below.",
            "x",
            "y",
        ];
        sort(&mut input, SortStrategy::Default);
        assert_eq!(input, expected);
    }

    #[test]
    fn with_multi_line_start_comment_bazel() {
        let mut input = vec![
            "# Some multi-line comment",
            "# for the line below.",
            "b",
            "a",
        ];
        let expected = vec![
            "a",
            "# Some multi-line comment",
            "# for the line below.",
            "b",
        ];
        sort(&mut input, SortStrategy::Default);
        assert_eq!(input, expected);
    }

    #[test]
    fn with_multi_line_trailing_comment_bazel() {
        let mut input = vec!["b", "a", "# Some multi-line comment", "# trailing comment."];
        let expected = vec!["a", "b", "# Some multi-line comment", "# trailing comment."];
        sort(&mut input, SortStrategy::Default);
        assert_eq!(input, expected);
    }

    #[test]
    fn with_several_single_line_comments_rust() {
        let mut input = vec![
            "y",
            "// Some multi-line comment",
            "// for the line below.",
            "x",
            "b",
            "a",
            "// Some multi-line comment",
            "// trailing comment.",
        ];
        let expected = vec![
            "a",
            "b",
            "// Some multi-line comment",
            "// for the line below.",
            "x",
            "y",
            "// Some multi-line comment",
            "// trailing comment.",
        ];
        sort(&mut input, SortStrategy::Default);
        assert_eq!(input, expected);
    }

    #[test]
    #[ignore]
    fn with_multi_line_comment_rust() {
        let mut input = vec![
            "y",
            "/* Some multi-line comment",
            "   for the line below.  */",
            "x",
            "b",
            "a",
        ];
        let expected = vec![
            "a",
            "b",
            "/* Some multi-line comment",
            "   for the line below.  */",
            "x",
            "y",
        ];
        sort(&mut input, SortStrategy::Default);
        assert_eq!(input, expected);
    }
}

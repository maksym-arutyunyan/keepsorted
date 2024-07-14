#[derive(Debug)]
pub enum SortStrategy {
    Bazel,
    Default,
}

#[derive(Debug, PartialEq, Eq)]
struct LineGroup<'a> {
    comments: Vec<&'a str>,
    code: &'a str,
}

impl<'a> LineGroup<'a> {
    fn new() -> Self {
        Self {
            comments: Vec::new(),
            code: "",
        }
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
            current_group.code = line;
            groups.push(current_group);
            current_group = LineGroup::new();
        }
    }
    let trailing_comments = current_group.comments;

    match strategy {
        SortStrategy::Bazel => groups.sort_by(|a, b| custom_comparator(a.code, b.code)),
        _ => groups.sort_by(|a, b| a.code.cmp(b.code)),
    }

    let sorted_block: Vec<&str> = groups.into_iter()
        .flat_map(|group| group.comments.into_iter().chain(std::iter::once(group.code)))
        .chain(trailing_comments)
        .collect();

    block.copy_from_slice(&sorted_block);
}

fn custom_comparator(a: &str, b: &str) -> std::cmp::Ordering {
    sorting_key(a).cmp(&sorting_key(b))
}

fn sorting_key(line: &str) -> (u8, &str) {
    let trimmed = line.trim();
    if trimmed.starts_with(r#"":"#) {
        (0, trimmed)
    } else if trimmed.starts_with(r#""//"#) {
        (1, trimmed)
    } else if trimmed.starts_with(r#""@"#) {
        (2, trimmed)
    } else {
        (3, trimmed)
    }
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
    fn bazel_order() {
        let mut input = vec![
            r#"":bbb""#,
            r#"":aaa""#,
            r#""//dir/subdir:bbb""#,
            r#""//dir/subdir:aaa""#,
            r#""@crate_index//:bbb""#,
            r#""@crate_index//:aaa""#,
        ];
        let expected = vec![
            r#"":aaa""#,
            r#"":bbb""#,
            r#""//dir/subdir:aaa""#,
            r#""//dir/subdir:bbb""#,
            r#""@crate_index//:aaa""#,
            r#""@crate_index//:bbb""#,
        ];
        sort(&mut input, SortStrategy::Bazel);
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
        let mut input = vec![
            "b", 
            "a",
            "# Some multi-line comment",
            "# trailing comment.",
        ];
        let expected = vec![
            "a",
            "b", 
            "# Some multi-line comment",
            "# trailing comment.",
        ];
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

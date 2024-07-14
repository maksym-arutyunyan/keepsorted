
pub enum SortStrategy {
    Default = 0,
    Bazel = 1,
}

pub fn sort(block: &mut [&str], strategy: SortStrategy) {
    match strategy {
        SortStrategy::Bazel => block.sort_by(|&a, &b| custom_comparator(a, b)),
        _ => block.sort_unstable(),
    }
}

fn custom_comparator(a: &str, b: &str) -> std::cmp::Ordering {
    let key_a = sorting_key(a);
    let key_b = sorting_key(b);
    key_a.cmp(&key_b)
}

fn sorting_key(line: &str) -> (u8, &str) {
    let line = line.trim();
    if line.starts_with(r#"":"#) {
        (0, line)
    } else if line.starts_with(r#""//"#) {
        (1, line)
    } else if line.starts_with(r#""@"#) {
        (2, line)
    } else {
        (3, line)
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
    #[ignore]
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
    #[ignore]
    fn with_several_single_line_comments_rust() {
        let mut input = vec![
            "y",
            "// Some multi-line comment",
            "// for the line below.",
            "x",
            "b", 
            "a",
        ];
        let expected = vec![
            "a",
            "b", 
            "// Some multi-line comment",
            "// for the line below.",
            "x",
            "y",
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


pub fn sort(block: &mut [&str]) {
    block.sort_unstable();
}

pub fn sort_bazel(block: &mut [&str]) {
    block.sort_by(|&a, &b| custom_comparator(a, b));
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
mod block {
    use super::*;

    #[test]
    fn simple() {
        let mut input = vec!["b", "a"];
        let expected = vec!["a", "b"];
        sort(&mut input);
        assert!(input == expected, "Expected: {expected:?}\nActual: {input:?}");
    }

    #[test]
    #[ignore]
    fn with_multi_line_comment() {
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
        sort(&mut input);
        assert_eq!(input, expected);
    }
}

use crate::block;

#[test]
fn simple() {
    let mut input = vec!["b", "a"];
    let expected = vec!["a", "b"];
    block::sort(&mut input, block::SortStrategy::Default);
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
        let left = block::SortKey::new(window[0]);
        let right = block::SortKey::new(window[1]);
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
    block::sort(&mut input, block::SortStrategy::Bazel);
    assert_eq!(input, expected);
}

#[test]
fn with_inline_comment_bazel() {
    let mut input = vec!["y", "x  # Some in-line comment.", "b", "a"];
    let expected = vec!["a", "b", "x  # Some in-line comment.", "y"];
    block::sort(&mut input, block::SortStrategy::Default);
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
    block::sort(&mut input, block::SortStrategy::Default);
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
    block::sort(&mut input, block::SortStrategy::Default);
    assert_eq!(input, expected);
}

#[test]
fn with_multi_line_trailing_comment_bazel() {
    let mut input = vec!["b", "a", "# Some multi-line comment", "# trailing comment."];
    let expected = vec!["a", "b", "# Some multi-line comment", "# trailing comment."];
    block::sort(&mut input, block::SortStrategy::Default);
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
    block::sort(&mut input, block::SortStrategy::Default);
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
    block::sort(&mut input, block::SortStrategy::Default);
    assert_eq!(input, expected);
}

use crate::bazel::BazelSortKey;

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

use std::cmp::Ordering;
use std::path::Path;

pub(crate) fn is_bazel(path: &Path) -> bool {
    match path.extension().and_then(|s| s.to_str()) {
        Some(ext) => matches!(ext, "bazel" | "bzl" | "BUILD" | "WORKSPACE"),
        None => false,
    }
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

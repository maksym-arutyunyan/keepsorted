use std::cmp::Ordering;

#[derive(Debug)]
pub enum SortStrategy {
    Default,
    Bazel,
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
    pub fn new(line: &'a str) -> Self {
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

#[derive(Debug, PartialEq, Eq)]
struct LineGroup<'a> {
    comments: Vec<&'a str>,
    code: &'a str,
    bazel_sort_key: BazelSortKey<'a>,
}

impl<'a> LineGroup<'a> {
    fn new() -> Self {
        Self {
            comments: Vec::new(),
            code: "",
            bazel_sort_key: BazelSortKey::new(""),
        }
    }

    fn set_code(&mut self, line: &'a str) {
        self.code = line;
        self.bazel_sort_key = BazelSortKey::new(line);
    }
}

fn is_single_line_comment(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with('#') || trimmed.starts_with("//")
}

pub fn sort(block: &mut [&str], strategy: SortStrategy) {
    let mut groups = Vec::with_capacity(block.len());
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
        SortStrategy::Default => groups.sort_by(|a, b| a.code.cmp(b.code)),
        SortStrategy::Bazel => groups.sort_by(|a, b| a.bazel_sort_key.cmp(&b.bazel_sort_key)),
    }

    let mut sorted_block = Vec::with_capacity(block.len());
    for group in groups {
        sorted_block.extend(group.comments);
        sorted_block.push(group.code);
    }
    sorted_block.extend(trailing_comments);

    block.copy_from_slice(&sorted_block);
}

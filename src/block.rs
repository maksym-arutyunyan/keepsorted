use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SortStrategy {
    Default,
    Bazel,
}

// A SortKey enum to handle different sorting strategies.
#[derive(Debug, PartialEq, Eq)]
enum SortKey<'a> {
    Default(&'a str),
    Bazel(BazelSortKey<'a>),
}

impl<'a> Ord for SortKey<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (SortKey::Default(a), SortKey::Default(b)) => a.cmp(b),
            (SortKey::Bazel(a), SortKey::Bazel(b)) => a.cmp(b),
            _ => Ordering::Equal, // This should not happen if used correctly
        }
    }
}

impl<'a> PartialOrd for SortKey<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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
    strategy: SortStrategy,
    sort_key: SortKey<'a>,
}

impl<'a> LineGroup<'a> {
    fn new(strategy: SortStrategy) -> Self {
        let sort_key = match strategy {
            SortStrategy::Default => SortKey::Default(""),
            SortStrategy::Bazel => SortKey::Bazel(BazelSortKey::new("")),
        };
        Self {
            comments: Default::default(),
            code: Default::default(),
            strategy,
            sort_key,
        }
    }

    fn set_code(&mut self, line: &'a str) {
        self.code = line;
        self.sort_key = match self.strategy {
            SortStrategy::Default => SortKey::Default(line),
            SortStrategy::Bazel => SortKey::Bazel(BazelSortKey::new(line)),
        };
    }
}

impl<'a> Ord for LineGroup<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.sort_key.cmp(&other.sort_key)
    }
}

impl<'a> PartialOrd for LineGroup<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn is_single_line_comment(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with('#') || trimmed.starts_with("//")
}

pub fn sort(block: &mut [&str], strategy: SortStrategy) {
    let mut groups = Vec::with_capacity(block.len());
    let mut current_group = LineGroup::new(strategy.clone());

    for &line in block.iter() {
        if is_single_line_comment(line) {
            current_group.comments.push(line);
        } else {
            current_group.set_code(line);
            groups.push(current_group);
            current_group = LineGroup::new(strategy.clone());
        }
    }
    let trailing_comments = current_group.comments;

    groups.sort();

    let mut sorted_block = Vec::with_capacity(block.len());
    for group in groups {
        sorted_block.extend(group.comments);
        sorted_block.push(group.code);
    }
    sorted_block.extend(trailing_comments);

    block.copy_from_slice(&sorted_block);
}

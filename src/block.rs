use crate::bazel::BazelSortKey;
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SortStrategy {
    Default,
    Bazel,
    CargoToml,
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

#[derive(Debug, PartialEq, Eq)]
struct ItemGroup<'a> {
    comment: Vec<&'a str>,
    code: &'a str,
    strategy: SortStrategy,
    sort_key: SortKey<'a>,
}

impl<'a> ItemGroup<'a> {
    fn new(strategy: SortStrategy) -> Self {
        let sort_key = match strategy {
            SortStrategy::Default => SortKey::Default(""),
            SortStrategy::Bazel => SortKey::Bazel(BazelSortKey::new("")),
            SortStrategy::CargoToml => SortKey::Default(""),
        };
        Self {
            comment: Default::default(),
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
            SortStrategy::CargoToml => SortKey::Default(line),
        };
    }
}

impl<'a> Ord for ItemGroup<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.sort_key.cmp(&other.sort_key)
    }
}

impl<'a> PartialOrd for ItemGroup<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn is_single_line_comment(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with('#') || trimmed.starts_with("//")
}

pub(crate) fn sort(block: &mut [&str], strategy: SortStrategy) {
    let mut groups = Vec::with_capacity(block.len());
    let mut current_group = ItemGroup::new(strategy.clone());

    for &line in block.iter() {
        if is_single_line_comment(line) {
            current_group.comment.push(line);
        } else {
            current_group.set_code(line);
            groups.push(current_group);
            current_group = ItemGroup::new(strategy.clone());
        }
    }
    let trailing_comment = current_group.comment;

    groups.sort();

    let mut sorted_block = Vec::with_capacity(block.len());
    for group in groups {
        sorted_block.extend(group.comment);
        sorted_block.push(group.code);
    }
    sorted_block.extend(trailing_comment);

    block.copy_from_slice(&sorted_block);
}

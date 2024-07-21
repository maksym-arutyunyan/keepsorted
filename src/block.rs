use crate::bazel::BazelSortKey;
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SortStrategy {
    Generic,
    Bazel,
    CargoToml,
}

// A SortKey enum to handle different sorting strategies.
#[derive(Debug, PartialEq, Eq)]
enum SortKey<'a> {
    Generic(&'a str),
    Bazel(BazelSortKey<'a>),
}

impl<'a> Ord for SortKey<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (SortKey::Generic(a), SortKey::Generic(b)) => a.cmp(b),
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
struct Item<'a> {
    comment: Vec<&'a str>,
    item: Vec<&'a str>,
    strategy: SortStrategy,
    sort_key: SortKey<'a>,
}

impl<'a> Item<'a> {
    fn new(strategy: SortStrategy) -> Self {
        let sort_key = match strategy {
            SortStrategy::Generic => SortKey::Generic(""),
            SortStrategy::Bazel => SortKey::Bazel(BazelSortKey::new("")),
            SortStrategy::CargoToml => SortKey::Generic(""),
        };
        Self {
            comment: Default::default(),
            item: Default::default(),
            strategy,
            sort_key,
        }
    }

    fn add_item(&mut self, line: &'a str) {
        self.item.push(line);
        self.sort_key = match self.strategy {
            SortStrategy::Generic => SortKey::Generic(line),
            SortStrategy::Bazel => SortKey::Bazel(BazelSortKey::new(line)),
            SortStrategy::CargoToml => SortKey::Generic(line),
        };
    }
}

impl<'a> Ord for Item<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.sort_key.cmp(&other.sort_key)
    }
}

impl<'a> PartialOrd for Item<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn is_single_line_comment(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with('#') || trimmed.starts_with("//")
}

pub(crate) fn sort(block: &mut [&str], strategy: SortStrategy) {
    let mut items = Vec::with_capacity(block.len());
    let mut current_item = Item::new(strategy.clone());

    for &line in block.iter() {
        if is_single_line_comment(line) {
            current_item.comment.push(line);
        } else {
            current_item.add_item(line);
            items.push(current_item);
            current_item = Item::new(strategy.clone());
        }
    }
    let trailing_comment = current_item.comment;

    items.sort();

    let mut sorted_block = Vec::with_capacity(block.len());
    for group in items {
        sorted_block.extend(group.comment);
        sorted_block.extend(group.item);
    }
    sorted_block.extend(trailing_comment);

    block.copy_from_slice(&sorted_block);
}

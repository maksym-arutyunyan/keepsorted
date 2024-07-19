use std::cmp::Ordering;

#[derive(Debug)]
pub enum SortStrategy {
    Bazel,
    Default,
}

#[derive(Debug, PartialEq, Eq)]
pub struct SortKey<'a> {
    phase: i16,
    split: Vec<&'a str>,
}

impl<'a> SortKey<'a> {
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

impl<'a> Ord for SortKey<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.phase
            .cmp(&other.phase)
            .then_with(|| self.split.cmp(&other.split))
    }
}

impl<'a> PartialOrd for SortKey<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct LineGroup<'a> {
    comments: Vec<&'a str>,
    code: &'a str,
    sort_key: SortKey<'a>,
}

impl<'a> LineGroup<'a> {
    fn new() -> Self {
        Self {
            comments: Vec::new(),
            code: "",
            sort_key: SortKey::new(""),
        }
    }

    fn set_code(&mut self, line: &'a str) {
        self.code = line;
        self.sort_key = SortKey::new(line);
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
        SortStrategy::Bazel => groups.sort_by(|a, b| a.sort_key.cmp(&b.sort_key)),
        SortStrategy::Default => groups.sort_by(|a, b| a.code.cmp(b.code)),
    }

    let mut sorted_block = Vec::with_capacity(block.len());
    for group in groups {
        sorted_block.extend(group.comments);
        sorted_block.push(group.code);
    }
    sorted_block.extend(trailing_comments);

    block.copy_from_slice(&sorted_block);
}

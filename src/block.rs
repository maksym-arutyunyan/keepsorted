use std::cmp::Ordering;

#[derive(Debug)]
pub enum SortStrategy {
    Bazel,
    Default,
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
pub struct SortKey<'a> {
    phase: i16,
    split: Vec<&'a str>,
}

impl<'a> SortKey<'a> {
    pub fn new(line: &'a str) -> Self {
        // Trim the input line
        let trimmed = line.trim();

        // Find and remove the portion of the line starting from the '#' character
        let line_without_comment = trimmed.split('#').next().unwrap_or("").trim();

        // Determine the phase based on the beginning of the line
        let phase = if line_without_comment.starts_with("\":") {
            1
        } else if line_without_comment.starts_with("\"//") {
            2
        } else if line_without_comment.starts_with("\"@") {
            3
        } else if line_without_comment.starts_with('"') {
            0
        } else {
            4
        };

        // Split the line into components using '.' and ':' as delimiters
        let split: Vec<&str> = line_without_comment
            .split(|c| c == '.' || c == ':' || c == '"')
            .collect();

        // Create and return the SortKey instance
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
    ["#", "//"].iter().any(|&token| trimmed.starts_with(token))
}

pub fn sort(block: &mut [&str], strategy: SortStrategy) {
    let mut groups = Vec::new();
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
        _ => groups.sort_by(|a, b| a.code.cmp(b.code)),
    }

    let sorted_block: Vec<&str> = groups
        .into_iter()
        .flat_map(|group| {
            group
                .comments
                .into_iter()
                .chain(std::iter::once(group.code))
        })
        .chain(trailing_comments)
        .collect();

    block.copy_from_slice(&sorted_block);
}

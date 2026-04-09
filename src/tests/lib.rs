use crate::{classify, Strategy};
use std::path::Path;

#[test]
fn keep_sorted_keyword() {
    let re = crate::re_keyword_keep_sorted();
    for line in [
        "  #Keep sorted",
        "  # Keep sorted  ",
        "  # Keep   sorted .  ",
        "  #   keepsorted  : keep   sorted  .  ",
        "  //  Keep sorted   .  ",
        "  //keepsorted: keep sorted",
        "  //   keepsorted  : keep   sorted  .  ",
        "--keepsorted: keep sorted",
        "-- keep sorted",
    ] {
        assert!(
            re.is_match(line),
            "The regex failed to match the valid line: '{}'",
            line
        );
    }
}

#[test]
fn ignore_file_keyword() {
    let re = crate::re_keyword_ignore_file();
    for line in [
        "  #   keepsorted  : ignore   file  .  ",
        "#keepsorted:ignore file",
        "  //   keepsorted  : ignore   file  .  ",
        "//keepsorted:ignore file",
        "  --   keepsorted  : ignore   file  .  ",
        "--keepsorted:ignore file",
    ] {
        assert!(
            re.is_match(line),
            "The regex failed to match the valid line: '{}'",
            line
        );
    }
}

#[test]
fn ignore_block_keyword() {
    let re = crate::re_keyword_ignore_block();
    for line in [
        "  #   keepsorted  : ignore   block  .  ",
        "#keepsorted:ignore block",
        "  //   keepsorted  : ignore   block  .  ",
        "//keepsorted:ignore block",
        "  --   keepsorted  : ignore   block  .  ",
        "--keepsorted:ignore block",
    ] {
        assert!(
            re.is_match(line),
            "The regex failed to match the valid line: '{}'",
            line
        );
    }
}

#[test]
fn classify_bazel_files() {
    assert!(matches!(
        classify(Path::new("BUILD"), &[]).unwrap(),
        Strategy::Bazel
    ));
    assert!(matches!(
        classify(Path::new("WORKSPACE"), &[]).unwrap(),
        Strategy::Bazel
    ));
    assert!(matches!(
        classify(Path::new("foo.bazel"), &[]).unwrap(),
        Strategy::Bazel
    ));
    assert!(matches!(
        classify(Path::new("BUILD.bazel"), &[]).unwrap(),
        Strategy::Bazel
    ));
    assert!(matches!(
        classify(Path::new("WORKSPACE.bazel"), &[]).unwrap(),
        Strategy::Bazel
    ));
    assert!(matches!(
        classify(Path::new("foo.bzl"), &[]).unwrap(),
        Strategy::Bazel
    ));
}

#[test]
fn classify_cargo_toml() {
    assert!(matches!(
        classify(Path::new("Cargo.toml"), &[]).unwrap(),
        Strategy::CargoToml
    ));
    assert!(matches!(
        classify(Path::new("not_cargo.toml"), &[]).unwrap(),
        Strategy::Generic
    ));
}

#[test]
fn classify_gitignore() {
    let features = vec!["gitignore".to_string()];
    assert!(matches!(
        classify(Path::new(".gitignore"), &features).unwrap(),
        Strategy::Gitignore
    ));
    assert!(matches!(
        classify(Path::new(".gitignore"), &[]).unwrap(),
        Strategy::Generic
    ));
}

#[test]
fn classify_codeowners() {
    let features = vec!["codeowners".to_string()];
    assert!(matches!(
        classify(Path::new("CODEOWNERS"), &features).unwrap(),
        Strategy::Gitignore
    ));
    assert!(matches!(
        classify(Path::new("CODEOWNERS"), &[]).unwrap(),
        Strategy::Generic
    ));
}

#[test]
fn classify_rust() {
    assert!(matches!(
        classify(Path::new("main.rs"), &[]).unwrap(),
        Strategy::Generic
    ));
    let alphabetical = vec!["rust_derive_alphabetical".to_string()];
    assert!(matches!(
        classify(Path::new("main.rs"), &alphabetical).unwrap(),
        Strategy::RustDeriveAlphabetical
    ));
    let canonical = vec!["rust_derive_canonical".to_string()];
    assert!(matches!(
        classify(Path::new("main.rs"), &canonical).unwrap(),
        Strategy::RustDeriveCanonical
    ));
}

#[test]
fn classify_rust_derive_mutually_exclusive() {
    let features = vec![
        "rust_derive_alphabetical".to_string(),
        "rust_derive_canonical".to_string(),
    ];
    assert!(classify(Path::new("main.rs"), &features).is_err());
}

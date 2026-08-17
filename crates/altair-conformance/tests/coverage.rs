//! The document and the suite name the same scenarios.
//!
//! Ungated on purpose: this runs in the default `cargo test --workspace` and
//! stays green. It is how "all of them run" is checkable without running the
//! red suite, and it is what fails if someone adds a scenario to
//! `conformance/scenarios.md` without adding a test, or deletes a test.

use std::collections::BTreeSet;
use std::path::PathBuf;

/// `conformance/scenarios.md` states thirty four scenarios across sections A
/// to G: A1–A5, B1–B5, C1–C3, D1–D4, E1–E4, F1–F9, G1–G4. If that number
/// changes, the document changed, and this constant is where it is noticed.
const EXPECTED: usize = 34;

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
}

fn suite_source() -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("scenarios.rs");
    std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("could not read {}: {error}", path.display()))
}

/// Scenario ids as the document states them: a line opening `**A1. `.
fn ids_in_the_document() -> BTreeSet<String> {
    let path = repository_root().join("conformance").join("scenarios.md");
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("could not read {}: {error}", path.display()));
    let mut ids = BTreeSet::new();
    for line in text.lines() {
        let Some(rest) = line.strip_prefix("**") else {
            continue;
        };
        let Some((head, _)) = rest.split_once(". ") else {
            continue;
        };
        if is_scenario_id(head) {
            ids.insert(head.to_string());
        }
    }
    ids
}

/// Every `scenario!(id, test_name, …)` declaration in the suite.
///
/// Parsed whitespace-insensitively, because rustfmt decides for itself whether
/// an invocation fits on one line and the mapping must not depend on that.
fn declarations(source: &str) -> Vec<(String, String)> {
    let mut found = Vec::new();
    for chunk in source.split("scenario!(").skip(1) {
        let rest = chunk.trim_start();
        let rest = rest
            .strip_prefix('"')
            .expect("a scenario id, quoted, comes first");
        let (id, rest) = rest.split_once('"').expect("the id is closed");
        let rest = rest.trim_start().strip_prefix(',').expect("then a comma");
        let name: String = rest
            .trim_start()
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        found.push((id.to_string(), name));
    }
    found
}

fn is_scenario_id(candidate: &str) -> bool {
    let mut characters = candidate.chars();
    let Some(section) = characters.next() else {
        return false;
    };
    matches!(section, 'A'..='G')
        && characters.clone().count() > 0
        && characters.all(|character| character.is_ascii_digit())
}

#[test]
fn every_scenario_in_the_document_has_a_test_and_every_test_has_a_scenario() {
    let document = ids_in_the_document();
    let mut suite = BTreeSet::new();
    for (id, _) in declarations(&suite_source()) {
        assert!(is_scenario_id(&id), "{id} is not a scenario id");
        assert!(suite.insert(id.clone()), "{id} is declared twice");
    }

    let missing: Vec<_> = document.difference(&suite).collect();
    assert!(missing.is_empty(), "scenarios with no test: {missing:?}");

    let invented: Vec<_> = suite.difference(&document).collect();
    assert!(
        invented.is_empty(),
        "tests naming scenarios the document does not have: {invented:?}"
    );

    assert_eq!(
        document.len(),
        EXPECTED,
        "the document states {EXPECTED} scenarios"
    );
}

#[test]
fn every_test_is_named_for_its_scenario() {
    for (id, name) in declarations(&suite_source()) {
        assert!(
            name.starts_with(&format!("{}_", id.to_lowercase())),
            "the test for {id} is called {name}, so the mapping is not mechanical"
        );
    }
}

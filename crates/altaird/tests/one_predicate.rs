//! The audience predicate appears in exactly one place.
//!
//! This is the lane's done-when condition, and it is executable rather than a
//! comment promising it. It fails if somebody pastes a second copy, and it also
//! fails on a *paraphrased* second copy — a hand-rolled
//! `author_member_id = $1 OR ...` that an exact-string check would miss —
//! because it additionally refuses any source outside `store/audience.rs` that
//! names the audience column at all.
//!
//! The component model requires the same predicate on both paths. One
//! implementation is the cheap way to keep that true. Two is how it stops being
//! true in month four, and by then the divergence is a leak rather than a
//! refactor.
//!
//! Neither needle is written as a literal here, or this file would count as an
//! occurrence of itself.

use std::path::{Path, PathBuf};

/// Where the predicate is allowed to live.
const HOME: &str = "crates/altaird/src/store/audience.rs";

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("repo root above crates/altaird")
        .to_path_buf()
}

/// Every `.rs` and `.sql` source in the repository, except the migrations,
/// which are where the column is declared and must name it.
fn sources(root: &Path) -> Vec<PathBuf> {
    fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if path.is_dir() {
                if name.starts_with('.') || name == "target" || name == "migrations" {
                    continue;
                }
                walk(&path, out);
            } else if matches!(
                path.extension().and_then(|e| e.to_str()),
                Some("rs") | Some("sql")
            ) {
                out.push(path);
            }
        }
    }
    let mut out = Vec::new();
    walk(root, &mut out);
    assert!(!out.is_empty(), "found no sources under {}", root.display());
    out
}

#[test]
fn the_predicate_sql_exists_once_in_the_tree() {
    let needle = altaird::store::audience::predicate_sql();

    // A predicate that had been emptied would pass the count trivially.
    assert!(
        needle.contains("author_member_id") && needle.contains("= ANY"),
        "the predicate no longer looks like one: {needle}"
    );

    let root = repo_root();
    let mut hits: Vec<(PathBuf, usize)> = Vec::new();
    for path in sources(&root) {
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        let n = text.matches(needle).count();
        if n > 0 {
            hits.push((path, n));
        }
    }

    let total: usize = hits.iter().map(|(_, n)| n).sum();
    assert_eq!(
        total,
        1,
        "the audience predicate appears {total} times, in {:?}. It is written once, \
         in {HOME}, and both paths call it.",
        hits.iter()
            .map(|(p, n)| format!("{} x{n}", p.display()))
            .collect::<Vec<_>>()
    );
    assert!(
        hits[0].0.ends_with(HOME),
        "the predicate has moved out of {HOME}, to {}",
        hits[0].0.display()
    );
}

#[test]
fn nothing_else_in_the_tree_names_the_audience_column() {
    // Assembled rather than written, so the scan does not find this file.
    let column = ["audience", "member", "ids"].join("_");

    let root = repo_root();
    let offenders: Vec<String> = sources(&root)
        .into_iter()
        .filter(|p| !p.ends_with(HOME))
        .filter(|p| {
            std::fs::read_to_string(p)
                .map(|t| t.contains(&column))
                .unwrap_or(false)
        })
        .map(|p| p.display().to_string())
        .collect();

    assert!(
        offenders.is_empty(),
        "{offenders:?} name the audience column. Reasoning about audience \
         belongs in {HOME}, and a paraphrase of the predicate is a second \
         implementation whatever it is spelled like. If a query needs the \
         column's value, select it through the constant there."
    );

    // And the one place still does.
    let home = std::fs::read_to_string(root.join(HOME)).expect("audience.rs");
    assert!(home.contains(&column), "{HOME} no longer names the column");
}

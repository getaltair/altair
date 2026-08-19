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
//! A third check closes the escape the builder cannot: a second, unscoped
//! `FROM` over `entity` outside the store layer. That one also covers most of what the
//! first two would miss, because a second implementation has to reach the table
//! however cleverly it spells the column.
//!
//! **The known limit.** A determined developer wins: a computed table name, a
//! view, or a second implementation written inside `store/audience.rs` itself
//! would all pass. These are aimed at the paste and the paraphrase, which are
//! what actually happen. The gap is known rather than missed.
//!
//! No needle is written as a literal here, or this file would count as an
//! occurrence of itself.

use std::path::{Path, PathBuf};

/// Where the predicate is allowed to live.
const HOME: &str = "crates/altaird/src/store/audience.rs";

/// Whether a source is test code rather than the instance.
///
/// **Two of the three checks below apply to the instance only, and this is the
/// line.** The claim being protected is that the write path and the read path
/// call one predicate; a test asserting that they do has to be able to see
/// past it. A test that could only read the store through the audience-scoped
/// surface would be checking the predicate against itself, and it would report
/// success for a predicate that admitted everybody.
///
/// So a test may query `entity` directly and may name the column, and the
/// suite is where the ground truth an assertion compares against comes from.
///
/// The first check — the predicate's own SQL, counted across the whole tree —
/// is deliberately **not** scoped this way, because a literal paste is a
/// literal paste wherever it lands and a test has no business holding one.
fn is_test_source(path: &Path) -> bool {
    let separator = std::path::MAIN_SEPARATOR_STR;
    path.to_string_lossy()
        .contains(&["", "tests", ""].join(separator))
}

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

/// Whether a source belongs to a crate that could reach the instance's
/// structured store at all.
///
/// **The rule this scopes is about one table in one database**: the `entity`
/// table in the instance's PostgreSQL, where a candidate set that reached it
/// without the audience predicate would be a leak. A crate that does not link
/// `altaird` cannot name that store, so "this went round the predicate" is a
/// sentence that cannot be said about it — Cargo forbids the dependency cycle
/// that would let it.
///
/// **`altair-tui` is the live example and the reason this scope is written
/// down.** The terminal client keeps a SQLite file on a device holding what
/// that member has already been shown, and it has an `entity` table because
/// that is what the thing is called. A substring scan reads its `FROM entity`
/// as an instance query going round the predicate, and the only ways to
/// silence that without this scope are to rename a client's table for the
/// instance's benefit or to move client code into the instance's store layer.
/// Both are worse than the check being narrower.
///
/// **What this deliberately gives up.** A crate that never links `altaird` and
/// talks to the same PostgreSQL anyway — a reporting tool handed
/// `DATABASE_URL` — is not scanned. That is a different mistake from the one
/// this guards, and a harder one to make by accident: such a tool has to be
/// given the connection string, and being given it is itself the reviewable
/// moment.
///
/// The first check in this file, the predicate's own SQL counted across the
/// whole tree, is deliberately **not** scoped this way. A literal paste is a
/// literal paste wherever it lands.
fn could_reach_the_instances_store(root: &Path, path: &Path) -> bool {
    let Ok(relative) = path.strip_prefix(root) else {
        return true;
    };
    let mut parts = relative
        .components()
        .map(|c| c.as_os_str().to_string_lossy());
    if parts.next().as_deref() != Some("crates") {
        // Not in a crate at all, so nothing is known about it. Scanned.
        return true;
    }
    let Some(name) = parts.next() else {
        return true;
    };
    if name == "altaird" {
        return true;
    }
    let Ok(manifest) = std::fs::read_to_string(root.join("crates").join(&*name).join("Cargo.toml"))
    else {
        return true;
    };
    declares_the_instance(&manifest)
}

/// Whether a manifest declares a dependency on the instance, in any of the
/// three tables.
///
/// **A bare mention is not enough, and finding that out cost a run.** The
/// terminal client's manifest carries a comment naming `altaird` — it explains
/// why `rusqlite` is pinned below the latest, which is a `links = "sqlite3"`
/// conflict with the `sqlx-sqlite` that `altaird` drags into the resolve graph.
/// A substring scan reads that comment as a dependency. The same shape is in
/// `object_store_boundary.rs::links_instance`, for the same reason.
fn declares_the_instance(manifest: &str) -> bool {
    manifest.lines().map(str::trim).any(|line| {
        (line.starts_with("altaird") && line.contains('='))
            || (line.starts_with('[') && line.contains("altaird"))
    })
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

/// Whether a source issues SQL at all.
///
/// The wire calls its audience field by the same word the column uses, because
/// they are the same word, so the write path's reader of `EntityContent` names
/// it while having nothing to do with the store. That is a transcription and
/// not a predicate, and the thing that tells the two apart is that a predicate
/// has to reach the database.
///
/// **What this catches and what it does not.** A paraphrased predicate has to
/// run, so it has to be in a file that queries; such a file naming the column
/// fails. A file that only *defines* a paraphrase as a constant for another
/// file to run would slip through, and that is a known gap of the same kind as
/// the ones the module documentation already records.
fn issues_sql(text: &str) -> bool {
    let flat = text.to_ascii_lowercase();
    // Two needles, both unmistakable. Every query in this workspace goes
    // through sqlx, and a bare SQL constant carries a bound position. Prose
    // needles were tried and rejected: "where" is an ordinary English word and
    // these files are written in prose.
    //
    // Assembled rather than written, like the other needles here.
    ["sqlx".to_owned(), ["$", "1"].concat()]
        .iter()
        .any(|needle| flat.contains(needle))
}

#[test]
fn nothing_else_in_the_tree_names_the_audience_column() {
    // Assembled rather than written, so the scan does not find this file.
    let column = ["audience", "member", "ids"].join("_");

    let root = repo_root();
    let offenders: Vec<String> = sources(&root)
        .into_iter()
        .filter(|p| !p.ends_with(HOME))
        .filter(|p| !is_test_source(p))
        .filter(|p| {
            std::fs::read_to_string(p)
                .map(|t| t.contains(&column) && issues_sql(&t))
                .unwrap_or(false)
        })
        .map(|p| p.display().to_string())
        .collect();

    assert!(
        offenders.is_empty(),
        "{offenders:?} name the audience column and issue SQL. Reasoning about \
         audience belongs in {HOME}, and a paraphrase of the predicate is a \
         second implementation whatever it is spelled like. If a query needs \
         the column's value, select it through the constant there."
    );

    // And the one place still does.
    let home = std::fs::read_to_string(root.join(HOME)).expect("audience.rs");
    assert!(home.contains(&column), "{HOME} no longer names the column");
}

#[test]
fn nothing_outside_the_store_layer_queries_the_entity_table() {
    // `CandidateQuery` puts the predicate on the candidate set it builds. It
    // cannot put one on a second `FROM` over `entity` smuggled into a projection or a
    // UNION arm — it refuses such a fragment at runtime, and this refuses one
    // at rest, anywhere in the tree.
    //
    // This is also the check that survives a cleverly spelled column: a second
    // implementation still has to reach the table.
    //
    // Writes are deliberately not caught. `INSERT INTO entity` and
    // `UPDATE entity` are the write path's, and audience on a write is enforced
    // by looking the entity up first, not by a predicate on the statement.
    let store = ["crates", "altaird", "src", "store"].join(std::path::MAIN_SEPARATOR_STR);

    let root = repo_root();
    let mut offenders: Vec<String> = Vec::new();
    for path in sources(&root) {
        if path.to_string_lossy().contains(&store)
            || is_test_source(&path)
            || !could_reach_the_instances_store(&root, &path)
        {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        let flat: String = text
            .to_ascii_lowercase()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        // Assembled, not written, like the other needles.
        //
        // Matched at a word boundary. `FROM entity_date` and
        // `FROM entity_part_counter` are queries over other tables that happen
        // to start with the same word, and a substring scan calls them
        // offenders — which is a false alarm that would be silenced by moving
        // real work into the store layer for no reason.
        for shape in [["from", "entity"].join(" "), ["join", "entity"].join(" ")] {
            let mut rest = flat.as_str();
            while let Some(at) = rest.find(&shape) {
                let after = &rest[at + shape.len()..];
                let continues = after.starts_with(|c: char| c.is_ascii_alphanumeric() || c == '_');
                if !continues {
                    offenders.push(format!("{} ({shape})", path.display()));
                    break;
                }
                rest = after;
            }
        }
    }

    assert!(
        offenders.is_empty(),
        "{offenders:?} query the entity table outside the store layer. Every \
         candidate set over `entity` carries the audience predicate, and the \
         only thing that puts it there is CandidateQuery, beside {HOME}."
    );
}

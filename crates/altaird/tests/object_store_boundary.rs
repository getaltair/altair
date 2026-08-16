//! "Nothing outside the interface reads a path" is a condition, so it is a
//! test rather than a comment.
//!
//! DR-003 makes the four operations the whole boundary, and says why: the
//! moment something reads a path directly, replacing what sits behind the
//! interface stops being contained. A note asking people not to do that fails
//! silently. This fails loudly, on the line that did it.
//!
//! Two checks, because there are two ways to break it. A path can *escape* —
//! the interface hands one out, and callers do the rest innocently. Or a
//! caller can *reach around* — it never asks the store for anything and opens
//! the file itself.
//!
//! Which crates the reaching-around check covers, and what that scope
//! deliberately gives up, is on `crates_that_could_hold_an_object_store`. Read
//! it before adding a crate to the workspace that touches the filesystem.

use std::fs;
use std::path::{Path, PathBuf};

/// The crate the interface lives in, and the one this is a test of.
const INSTANCE: &str = "altaird";

/// Crates that are scanned even though they do not link the instance.
///
/// Empty, and here so that the answer to "my crate holds the object root but
/// does not depend on `altaird`" is an entry rather than a puzzle. See
/// [`crates_that_could_hold_an_object_store`] for when that applies. Names are
/// directory names under `crates/`.
const ALSO_SCANNED: &[&str] = &[];

/// Files that may name a filesystem path or perform filesystem I/O.
///
/// Adding to this list is the deliberate act the whole test exists to make
/// visible: a reviewer sees the entry in the diff and has to agree that the
/// file has business touching the disk. Paths are relative to `crates/`.
const ALLOWED: &[&str] = &[
    // The store itself, which is the only thing permitted to know where the
    // bytes are.
    "altaird/src/objects/",
    // Its own tests, which create the roots they hand to `open`, and check
    // that a failed upload left nothing on disk.
    "altaird/tests/object_store.rs",
    // This file, which reads sources in order to make the assertion.
    "altaird/tests/object_store_boundary.rs",
];

/// Reaching the bytes needs one of these. Naming a path is enough on its own:
/// a path that never gets opened here gets opened somewhere else later.
const REACHING_AROUND: &[&str] = &[
    "std::fs",
    "tokio::fs",
    "fs::",
    "PathBuf",
    "Path::",
    "AsRef<Path>",
    "&Path",
    "File::open",
    "File::create",
    "read_to_string",
    "read_dir",
];

fn crates_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the crate sits inside crates/")
        .to_path_buf()
}

/// Which crates are scanned, and why it is not simply all of them.
///
/// In scope is every crate that could hold an [`ObjectStore`]: `altaird`,
/// where the four operations live, and any workspace crate that depends on it.
/// Cargo forbids dependency cycles, so a crate that does not depend on
/// `altaird` cannot name the interface at all, which makes "this went round
/// the interface" a sentence that cannot be said about it. Its filesystem work
/// is its own business and belongs to whatever it is a component of.
///
/// `altair-conformance` is the live example and the reason this scope is
/// written down. It runs a client under test as a subprocess and hands it a
/// durable state directory and an ephemeral runtime one — a distinction that
/// is load bearing for the scenario that tells a device restart from a process
/// kill. That is filesystem work with nothing to do with file bodies, in a
/// crate that links `altair-proto` and not the instance.
///
/// **What this deliberately gives up.** A crate that never links `altaird` and
/// reads the body directory anyway — a backup or migration tool handed the
/// object root — is not scanned and would not be caught. That is a different
/// mistake from the one DR-003 is guarding here, which is the instance going
/// round its own boundary, and it is a harder one to make by accident: such a
/// tool has to be given the root from configuration, and being given it is
/// itself the reviewable moment. If one is ever written, name it in
/// [`ALSO_SCANNED`] and this check covers it again.
fn crates_that_could_hold_an_object_store() -> Vec<PathBuf> {
    let mut scanned = Vec::new();
    for entry in fs::read_dir(crates_dir()).expect("read crates/") {
        let dir = entry.expect("entry").path();
        let Ok(manifest) = fs::read_to_string(dir.join("Cargo.toml")) else {
            continue;
        };
        let name = dir
            .file_name()
            .expect("a crate directory has a name")
            .to_string_lossy()
            .to_string();

        if name == INSTANCE || ALSO_SCANNED.contains(&name.as_str()) || links_instance(&manifest) {
            scanned.push(dir);
        }
    }
    assert!(
        scanned.iter().any(|dir| dir.ends_with(INSTANCE)),
        "the crate holding the interface is not being scanned, so this test asserts nothing"
    );
    scanned
}

/// Whether a manifest declares a dependency on the instance, in any of the
/// three tables. Deliberately generous: a manifest that mentions `altaird` at
/// all is scanned, because over-scanning costs someone an explanation and
/// under-scanning costs the boundary.
fn links_instance(manifest: &str) -> bool {
    manifest.lines().map(str::trim).any(|line| {
        (line.starts_with(INSTANCE) && line.contains('='))
            || (line.starts_with('[') && line.contains(INSTANCE))
    })
}

/// Every `.rs` file under `src` and `tests` of the crates in scope. Build
/// scripts are excluded on purpose: they run before anything exists and cannot
/// reach a body.
fn instance_sources() -> Vec<PathBuf> {
    let mut found = Vec::new();
    for crate_dir in crates_that_could_hold_an_object_store() {
        for area in ["src", "tests"] {
            collect(&crate_dir.join(area), &mut found);
        }
    }
    found.sort();
    assert!(
        found.len() > 3,
        "the walk found almost nothing, so it is not testing anything: {found:?}"
    );
    found
}

fn collect(dir: &Path, found: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries {
        let path = entry.expect("entry").path();
        if path.is_dir() {
            collect(&path, found);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            found.push(path);
        }
    }
}

fn relative(path: &Path) -> String {
    path.strip_prefix(crates_dir())
        .expect("under crates/")
        .to_string_lossy()
        .replace('\\', "/")
}

fn is_allowed(path: &Path) -> bool {
    let relative = relative(path);
    ALLOWED.iter().any(|allowed| relative.starts_with(allowed))
}

/// Reach around the interface and this fails, naming the file and the line.
#[test]
fn nothing_outside_the_object_store_touches_the_bytes() {
    let mut violations = Vec::new();

    for path in instance_sources() {
        if is_allowed(&path) {
            continue;
        }
        let source = fs::read_to_string(&path).expect("read a source file");
        for (number, line) in source.lines().enumerate() {
            for token in REACHING_AROUND {
                if line.contains(token) {
                    violations.push(format!(
                        "{}:{}: `{token}` in `{}`",
                        relative(&path),
                        number + 1,
                        line.trim()
                    ));
                }
            }
        }
    }

    assert!(
        violations.is_empty(),
        "the object store's four operations are the whole boundary (DR-003), and this code goes \
         round them. These files are scanned because their crate links `{INSTANCE}` and can \
         therefore hold an ObjectStore. Use `altaird::objects::ObjectStore`, or, if the file \
         genuinely has business with the filesystem that is not a file body, add it to ALLOWED in \
         {} and say why.\n\n{}",
        file!(),
        violations.join("\n")
    );
}

/// The scope follows the dependency, so nothing is excluded by name and a
/// crate joins the scan the moment it can hold an `ObjectStore`. A crate added
/// to the workspace next year does not need anyone to remember this test
/// exists.
#[test]
fn a_crate_is_scanned_the_moment_it_links_the_instance() {
    assert!(links_instance(
        "[dependencies]\naltaird = { path = \"../altaird\" }\n"
    ));
    assert!(links_instance(
        "[dev-dependencies]\naltaird = { path = \"../altaird\", features = [\"testing\"] }\n"
    ));
    assert!(links_instance(
        "[dependencies.altaird]\npath = \"../altaird\"\n"
    ));

    // And a crate that links only the contract is a component of something
    // else, whose filesystem work is not this boundary's business.
    assert!(!links_instance(
        "[dependencies]\naltair-proto = { path = \"../altair-proto\" }\ntokio = \"1\"\n"
    ));
}

fn object_store_source(name: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src/objects")
        .join(name);
    fs::read_to_string(&path).unwrap_or_else(|_| panic!("read {}", path.display()))
}

/// The other half: a path cannot leak *out* of the interface either, because
/// the interface never names one.
#[test]
fn no_path_escapes_the_interface() {
    let interface = object_store_source("interface.rs");
    for token in ["Path", "fs::", "File"] {
        assert!(
            !interface.contains(token),
            "`{token}` appears in the object store interface. The four operations stop being the \
             whole boundary the moment one of them hands a caller a path, so the boundary names \
             no filesystem type at all — not in a signature, and not in a returned value."
        );
    }
}

/// Beneath the interface, paths are ordinary and expected. What must stay true
/// is that only opening the store names one: it is the single place a path
/// enters, and there is no way back out.
#[test]
fn the_filesystem_implementation_names_a_path_only_where_the_store_is_opened() {
    let source = object_store_source("filesystem.rs");
    let lines: Vec<&str> = source.lines().collect();
    let mut violations = Vec::new();

    let mut index = 0;
    while index < lines.len() {
        if lines[index].trim_start().starts_with("pub ") {
            // Take the whole declaration, which may be wrapped over lines.
            let start = index;
            let mut declaration = String::new();
            while index < lines.len() {
                declaration.push_str(lines[index]);
                declaration.push(' ');
                if lines[index].contains('{') || lines[index].contains(';') {
                    break;
                }
                index += 1;
            }
            if declaration.contains("Path") && !declaration.contains("fn open") {
                violations.push(format!("{}: {}", start + 1, declaration.trim()));
            }
        }
        index += 1;
    }

    assert!(
        violations.is_empty(),
        "the root directory is named once, when the store is opened, and never leaves again. \
         These public items in filesystem.rs name a path:\n{}",
        violations.join("\n")
    );
}

/// DR-003 decides an interface of *exactly* four operations, and its smallness
/// is what makes replacing the filesystem contained work rather than a
/// migration. A fifth is a decision, not an addition.
#[test]
fn the_interface_is_still_four_operations() {
    let interface = object_store_source("interface.rs");
    let (_, trait_body) = interface
        .split_once("pub trait ObjectStore")
        .expect("the trait is declared here");

    let operations: Vec<&str> = trait_body
        .lines()
        .filter(|line| {
            let indented = line.starts_with("    ") && !line.starts_with("     ");
            let declaration = line.trim_start();
            indented && (declaration.starts_with("fn ") || declaration.starts_with("async fn "))
        })
        .collect();

    assert_eq!(
        operations.len(),
        4,
        "put, get, delete, enumerate — and nothing else (DR-003). Found:\n{}",
        operations.join("\n")
    );
}

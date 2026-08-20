//! "Nothing below the interceptor ever sees a token" is a condition, so it is
//! a test rather than a comment.
//!
//! A credential enters the instance in exactly one place —
//! `src/auth/identify.rs` — which resolves it, removes the header, and puts an
//! `Identity` in the request's extensions. Everything below is handed an
//! identity. The served surface has nothing to read even if it wanted to.
//!
//! That is easy to say and easy to lose. The next call added to the service
//! wants something the identity does not carry; the token is right there in
//! the metadata; two lines later there are two places that read credentials
//! and only one of them is the one with the reasoning attached. This fails on
//! that line.
//!
//! # What is scanned and what is not
//!
//! The instance's own sources, under `src`. Tests are excluded on the same
//! reasoning `tests/one_predicate.rs` records for the audience predicate: a
//! test asserting how the instance behaves for an expired credential has to be
//! able to mint one, present one, and name the header it arrives in. A test
//! that could only reach the instance through the surface it is testing would
//! be checking the edge against itself.
//!
//! **The known limit**, in the same spirit as the other structural checks
//! here: a determined developer wins. A header name assembled at runtime, or
//! read through a constant defined elsewhere, would pass. These are aimed at
//! the paste and at the convenient shortcut, which are what actually happen.
//!
//! No needle is written as a literal, or this file would be an occurrence of
//! itself.

use std::path::{Path, PathBuf};

/// Where a credential is allowed to be read.
const EDGE: &str = "src/auth/identify.rs";

/// Where the shape of a credential is allowed to be known.
///
/// `bearer_token` parses one, which is the same knowledge by a different name,
/// and it lives with the rest of validation.
const VALIDATION: &str = "src/auth/mod.rs";

fn instance_sources() -> Vec<PathBuf> {
    fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, out);
            } else if path.extension().is_some_and(|e| e == "rs") {
                out.push(path);
            }
        }
    }
    let mut out = Vec::new();
    walk(&Path::new(env!("CARGO_MANIFEST_DIR")).join("src"), &mut out);
    assert!(
        out.len() > 10,
        "the walk found almost nothing, so it is not testing anything: {out:?}"
    );
    out
}

fn relative(path: &Path) -> String {
    path.strip_prefix(env!("CARGO_MANIFEST_DIR"))
        .expect("under the crate")
        .to_string_lossy()
        .replace('\\', "/")
}

#[test]
fn only_the_edge_names_the_header_a_credential_arrives_in() {
    // Assembled rather than written, so the scan does not find this file.
    let header = ["author", "ization"].concat();

    // `VALIDATION` is permitted too, and narrowly: `bearer_token` takes the
    // header's value as its argument and is named after it. That is the parser
    // knowing what it parses, in the module whose subject is credentials.
    // Nothing there reads a request.
    let offenders: Vec<String> = instance_sources()
        .into_iter()
        .filter(|p| {
            let at = relative(p);
            !at.ends_with(EDGE) && !at.ends_with(VALIDATION)
        })
        .filter(|p| {
            std::fs::read_to_string(p)
                .map(|t| t.to_ascii_lowercase().contains(&header))
                .unwrap_or(false)
        })
        .map(|p| relative(&p))
        .collect();

    assert!(
        offenders.is_empty(),
        "{offenders:?} name the header a credential arrives in. It is read once, in {EDGE}, \
         which removes it and puts an Identity in its place. Anything below that is handed \
         an identity and has no business with a credential."
    );

    // And the one place still does, or this would pass over an edge that had
    // stopped reading the header at all.
    let edge = std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join(EDGE))
        .expect("the edge");
    assert!(
        edge.to_ascii_lowercase().contains(&header),
        "{EDGE} no longer names the header, so nothing in the instance reads a credential"
    );
}

#[test]
fn only_validation_knows_what_a_credential_looks_like() {
    // Assembled, like the other needles.
    let parser = ["bearer", "_", "token"].concat();

    let offenders: Vec<String> = instance_sources()
        .into_iter()
        .filter(|p| {
            let at = relative(p);
            !at.ends_with(EDGE) && !at.ends_with(VALIDATION)
        })
        .filter(|p| {
            std::fs::read_to_string(p)
                .map(|t| t.contains(&parser))
                .unwrap_or(false)
        })
        .map(|p| relative(&p))
        .collect();

    assert!(
        offenders.is_empty(),
        "{offenders:?} take a credential apart. That happens in {VALIDATION} and is called \
         from {EDGE}, and from nowhere else."
    );
}

/// The served surface is the file this is really about, so it is asserted
/// directly as well as by the sweep — a rename of the module, or a second
/// service file, would move the sweep's target without anybody noticing.
#[test]
fn the_served_surface_reads_an_identity_and_nothing_else() {
    let service =
        std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("src/service.rs"))
            .expect("the served surface");

    for shape in [
        ["author", "ization"].concat(),
        ["bearer", "_", "token"].concat(),
        ["Authentic", "ator"].concat(),
    ] {
        assert!(
            !service.contains(&shape),
            "the served surface names `{shape}`. Every call there takes the Identity the \
             edge resolved; reaching for a credential, or for the thing that resolves one, \
             is how the single place a token is read becomes two."
        );
    }

    assert!(
        service.contains("Identity"),
        "the served surface no longer reads an identity, so it is deciding who is asking \
         some other way"
    );
}

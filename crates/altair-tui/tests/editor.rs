//! Handing composition to the person's editor, and keeping the file.

use altair_tui::editor::{Editor, Handoff};

fn editor() -> (tempfile::TempDir, Editor) {
    let directory = tempfile::tempdir().expect("a temporary directory");
    let editor = Editor::new(directory.path());
    (directory, editor)
}

#[test]
fn the_file_is_on_disk_before_composing_begins() {
    let (_directory, editor) = editor();
    let composition = editor.begin("what was already there").expect("begin");
    assert!(
        composition.path().exists(),
        "the person's words exist outside the store only here, so the file is \
         written before anything else happens"
    );
    assert_eq!(
        editor.read(&composition).expect("read"),
        "what was already there"
    );
}

#[test]
fn a_composition_something_died_in_the_middle_of_is_offered_back() {
    let directory = tempfile::tempdir().expect("a temporary directory");
    {
        let editor = Editor::new(directory.path());
        editor.begin("half a paragraph").expect("begin");
        // And then the process is gone. Nothing was cleaned up, deliberately.
    }
    let editor = Editor::new(directory.path());
    let abandoned = editor.abandoned().expect("abandoned");
    assert_eq!(abandoned.len(), 1, "the file is still there");
    assert_eq!(
        editor.read(&abandoned[0]).expect("read"),
        "half a paragraph",
        "and it is what the person had written"
    );
}

#[test]
fn a_finished_composition_stops_being_offered_back() {
    let (_directory, editor) = editor();
    let composition = editor.begin("done").expect("begin");
    editor.finish(&composition).expect("finish");
    assert!(
        editor.abandoned().expect("abandoned").is_empty(),
        "once the words reached the store the file is no longer the only copy"
    );
    editor
        .finish(&composition)
        .expect("finishing twice is fine");
}

#[test]
fn nowhere_to_compose_is_a_fault_and_not_a_wait() {
    let (_directory, editor) = editor();
    let composition = editor.begin("").expect("begin");
    // A name nothing on any machine answers to.
    let missing = "altair-no-such-editor-9f3c".to_string();
    match editor.edit_with(&missing, &composition) {
        Err(Handoff::CannotStart { program, .. }) => assert_eq!(program, missing),
        other => panic!(
            "an editor that is not installed will not become installed because \
             time passed, so it is said at the moment of the attempt; got {other:?}"
        ),
    }
    assert!(
        composition.path().exists(),
        "and the person's file is untouched by the attempt"
    );
}

#[test]
fn a_round_trip_returns_what_was_written() {
    let (directory, editor) = editor();
    let composition = editor.begin("before").expect("begin");
    let double = write_editor_double(directory.path());

    let returned = editor
        .edit_with(&double, &composition)
        .expect("the double runs");
    assert_eq!(
        returned, "after\n",
        "what comes back is what is in the file the editor was handed"
    );
    assert_eq!(editor.read(&composition).expect("read"), "after\n");
}

/// A stand-in for the person's editor: a program that writes into the file it
/// is given and exits.
///
/// **A real process on each platform, rather than a stub in this test.** The
/// handoff is a spawn, and a spawn is the part that behaves differently on
/// Windows; testing it without one would be testing the half that was never in
/// doubt.
#[cfg(unix)]
fn write_editor_double(directory: &std::path::Path) -> String {
    use std::os::unix::fs::PermissionsExt;
    let path = directory.join("double.sh");
    std::fs::write(&path, "#!/bin/sh\nprintf 'after\\n' > \"$1\"\n").expect("write the double");
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755))
        .expect("make it runnable");
    path.to_string_lossy().to_string()
}

#[cfg(windows)]
fn write_editor_double(directory: &std::path::Path) -> String {
    let path = directory.join("double.cmd");
    std::fs::write(&path, "@echo off\r\n(echo after)> %1\r\n").expect("write the double");
    // Through `cmd /C`, because a batch file is not something CreateProcess
    // will start on its own — and going through the shell is also how a person
    // configuring a `.cmd` editor on Windows would have to write it.
    format!("cmd /C {}", path.to_string_lossy())
}

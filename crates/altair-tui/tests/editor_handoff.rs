//! What the screen holds after the person's editor has been all over it.
//!
//! **This is the one thing no other test in this crate can see.** Every screen
//! is snapshot-checked by rendering into a buffer, which says what the client
//! *meant* to draw and nothing about what a terminal is actually showing.
//! Drawing is a diff against what ratatui believes is on screen, and an editor
//! running in the middle of that makes the belief wrong about every cell —
//! so the failure is not a wrong frame, it is a correct frame painted over
//! somebody else's screen.
//!
//! It happened. The first version of the handoff called `ratatui::restore()`
//! and then `ratatui::try_init()`, which builds a second terminal and drops
//! it, leaving the one the loop draws through still believing the old screen
//! was intact. Coming back from `nvim` gave a title and a section rule floating
//! in the editor's leftovers, with no header and no status line.
//!
//! So this drives the real binary through a real pty, with a stand-in editor
//! that paints the whole terminal the way an editor does, and asks what the
//! client emitted afterwards.

#![cfg(unix)]

use std::io::{Read, Write};
use std::os::fd::{AsFd, OwnedFd};
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// A stand-in editor: paints the terminal, then writes the file it was given.
///
/// The painting is the point. An editor that only wrote the file would leave
/// the screen exactly as ratatui remembered it, and the bug this guards would
/// not appear.
const DOUBLE: &str = "#!/bin/sh\n\
                      printf '\\033[2J\\033[H'\n\
                      i=1\n\
                      while [ $i -le 30 ]; do printf 'EDITOR PAINTED %02d\\n' \"$i\"; i=$((i+1)); done\n\
                      printf 'written by the editor\\n' > \"$1\"\n";

/// The last thing the stand-in prints, so the assertions can look at what came
/// after it.
const LAST_PAINTED: &str = "EDITOR PAINTED 30";

fn pty() -> (OwnedFd, std::fs::File) {
    use rustix::pty::{OpenptFlags, grantpt, openpt, ptsname, unlockpt};
    let controller = openpt(OpenptFlags::RDWR | OpenptFlags::NOCTTY).expect("open a pty");
    grantpt(&controller).expect("grant");
    unlockpt(&controller).expect("unlock");
    let name = ptsname(&controller, Vec::new()).expect("name");
    let device = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(std::ffi::OsStr::new(
            std::str::from_utf8(name.as_bytes()).expect("a pty name is ascii"),
        ))
        .expect("open the other end");
    rustix::termios::tcsetwinsize(
        controller.as_fd(),
        rustix::termios::Winsize {
            ws_row: 40,
            ws_col: 110,
            ws_xpixel: 0,
            ws_ypixel: 0,
        },
    )
    .expect("say how big the terminal is");
    (controller, device)
}

#[test]
fn the_screen_is_repainted_in_full_after_an_editor_has_drawn_on_it() {
    let home = tempfile::tempdir().expect("a temporary directory");
    let script = home.path().join("editor.sh");
    std::fs::write(&script, DOUBLE).expect("write the stand-in");
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&script, std::fs::Permissions::from_mode(0o755))
            .expect("make it runnable");
    }

    let (controller, device) = pty();
    let mut child = std::process::Command::new(env!("CARGO_BIN_EXE_altair"))
        .env("ALTAIR_STATE_DIR", home.path())
        .env("TERM", "xterm-256color")
        // Both, because a client reads VISUAL first and a developer's shell
        // very often has one.
        .env("EDITOR", &script)
        .env("VISUAL", &script)
        .stdin(device.try_clone().expect("clone"))
        .stdout(device.try_clone().expect("clone"))
        .stderr(device)
        .spawn()
        .expect("start the client");

    let seen = Arc::new(Mutex::new(Vec::<u8>::new()));
    let reading = Arc::clone(&seen);
    let mut source = std::fs::File::from(controller.try_clone().expect("clone"));
    std::thread::spawn(move || {
        let mut chunk = [0_u8; 8192];
        while let Ok(read) = source.read(&mut chunk) {
            if read == 0 {
                return;
            }
            reading
                .lock()
                .expect("seen")
                .extend_from_slice(&chunk[..read]);
        }
    });

    let mut sink = std::fs::File::from(controller);
    let mut press = |keys: &str, settle: u64| {
        sink.write_all(keys.as_bytes()).expect("press");
        sink.flush().expect("flush");
        std::thread::sleep(Duration::from_millis(settle));
    };
    std::thread::sleep(Duration::from_millis(900));
    press("c", 500);
    press("a note to edit", 500);
    press("\r", 900); // accepted, and back on the captures list
    press("\r", 900); // opened
    press("e", 2200); // handed over, painted on, and taken back
    press("q", 900);
    let _ = child.kill();
    let _ = child.wait();

    let out = seen.lock().expect("seen").clone();
    let text = String::from_utf8_lossy(&out).to_string();
    let painted = text.rfind(LAST_PAINTED).unwrap_or_else(|| {
        panic!("the stand-in editor never ran. What the client showed:\n{text}")
    });
    let after = strip_escapes(&text[painted..]);

    assert!(
        after.contains("A L T A I R"),
        "the client came back from the editor and did not redraw its own \
         header, so what the person is looking at is the editor's screen with \
         a few of the client's cells painted over it. What it emitted after \
         the editor finished:\n{after}"
    );
    assert!(
        after.contains("written by the editor"),
        "and the body it went to fetch is not on screen either"
    );
}

/// Text with the control sequences taken out, which is near enough for asking
/// whether something was drawn.
fn strip_escapes(text: &str) -> String {
    let mut out = String::new();
    let mut rest = text;
    while let Some(at) = rest.find('\u{1b}') {
        out.push_str(&rest[..at]);
        let tail = &rest[at + 1..];
        match tail.find(|c: char| c.is_ascii_alphabetic()) {
            Some(end) => rest = &tail[end + 1..],
            None => return out,
        }
    }
    out.push_str(rest);
    out
}

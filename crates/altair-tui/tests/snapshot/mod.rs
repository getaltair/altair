//! Rendering a widget in both themes and checking what it drew.
//!
//! **A terminal is a surface nobody can review from a diff**, so what is
//! checked in is what it actually drew: the palette, the grid, and a colour key
//! naming the token behind every run.
//!
//! All three, and each catches something the others do not. The grid alone goes
//! green on a theme that renders entirely in one colour. The key alone names the
//! *token* behind a run, so it goes green when a token keeps its name and
//! changes its value — which is exactly what editing a palette does. The palette
//! block is what closes that: it is the values, written out, so a changed hex is
//! a changed line. Both failure modes were induced and watched before this was
//! trusted.
//!
//! Regenerate with `ALTAIR_UPDATE_SNAPSHOTS=1`, and read the diff before
//! staging it.

#![allow(dead_code)]

use std::path::PathBuf;

use altair_tui::ui::chrome::Shell;
use altair_tui::ui::{Glyphs, Mode, Set, Theme};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;

/// How wide a snapshot is drawn. Wide enough for the design's own columns and
/// narrow enough to read in a diff.
pub const WIDTH: u16 = 100;

fn shell(mode: Mode) -> Shell {
    Shell::new(Theme::of(mode), Glyphs::of(Set::Signature))
}

/// Every token, once, so the palette block and the colour key cannot drift
/// apart or quietly stop covering one.
type Token = (&'static str, fn(&Theme) -> Color);
const TOKENS: &[Token] = &[
    ("ground", |t| t.ground),
    ("body", |t| t.body),
    ("bright", |t| t.bright),
    ("mid", |t| t.mid),
    ("dim", |t| t.dim),
    ("faint", |t| t.faint),
    ("rule", |t| t.rule),
    ("rule-faint", |t| t.rule_faint),
    ("selected", |t| t.selected),
    ("identity", |t| t.identity),
    ("attention", |t| t.attention),
    ("worked", |t| t.worked),
    ("refused", |t| t.refused),
    ("table-head", |t| t.table_head),
    ("keycap", |t| t.keycap),
    ("keycap-border", |t| t.keycap_border),
    ("sheet", |t| t.sheet),
];

/// Which token a colour is, so a snapshot says "identity" rather than a hex
/// triple that means nothing until you look it up. The value is in the palette
/// block; this says which role put it there.
fn token(theme: &Theme, colour: Color) -> &'static str {
    for (name, value) in TOKENS {
        if value(theme) == colour {
            return name;
        }
    }
    match colour {
        Color::Reset => "-",
        _ => "?",
    }
}

/// Every token and its value, so that changing what a token *is* shows up.
fn palette(theme: &Theme) -> String {
    let mut out = String::from("palette\n");
    for (name, colour) in TOKENS {
        out.push_str(&format!("  {name}: {}\n", hex(colour(theme))));
    }
    out.push('\n');
    out
}

fn hex(colour: Color) -> String {
    match colour {
        Color::Rgb(r, g, b) => format!("#{r:02x}{g:02x}{b:02x}"),
        other => format!("{other:?}"),
    }
}

/// The palette, the grid, then every run of one style, named.
fn dump(theme: &Theme, buffer: &Buffer) -> String {
    let area = buffer.area();
    let mut out = palette(theme);
    for y in 0..area.height {
        out.push('|');
        for x in 0..area.width {
            out.push_str(buffer[(x, y)].symbol());
        }
        out.push_str("|\n");
    }
    out.push_str("\ncolours\n");
    for y in 0..area.height {
        let mut run: Option<(u16, String)> = None;
        let mut runs = Vec::new();
        for x in 0..area.width {
            let cell = &buffer[(x, y)];
            let style = format!(
                "{}/{}{}",
                token(theme, cell.fg),
                token(theme, cell.bg),
                if cell.modifier.is_empty() {
                    String::new()
                } else {
                    format!("+{:?}", cell.modifier)
                }
            );
            match run.as_mut() {
                Some((_, current)) if *current == style => {}
                _ => {
                    if let Some((from, current)) = run.take() {
                        runs.push(format!("{from}..{x} {current}"));
                    }
                    run = Some((x, style));
                }
            }
        }
        if let Some((from, current)) = run {
            runs.push(format!("{from}..{} {current}", area.width));
        }
        out.push_str(&format!("  {y}: {}\n", runs.join("  ")));
    }
    out
}

pub fn snapshot_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/snapshots")
        .join(format!("{name}.txt"))
}

/// Compare against what is checked in, or write it when asked to.
pub fn assert_snapshot(name: &str, drawn: &str) {
    let path = snapshot_path(name);
    if std::env::var("ALTAIR_UPDATE_SNAPSHOTS").is_ok() {
        std::fs::create_dir_all(path.parent().expect("a parent")).expect("make the directory");
        std::fs::write(&path, drawn).expect("write the snapshot");
        return;
    }
    let held = std::fs::read_to_string(&path).unwrap_or_else(|_| {
        panic!(
            "no snapshot at {}. If this element is new, regenerate with \
             ALTAIR_UPDATE_SNAPSHOTS=1 and read the diff before staging it.",
            path.display()
        )
    });
    assert_eq!(
        held,
        drawn,
        "\n{} changed. What it draws now:\n{drawn}",
        path.display()
    );
}

/// Draw one thing in both themes, and check each.
pub fn in_both_themes(name: &str, height: u16, draw: impl Fn(&Shell, Rect, &mut Buffer)) {
    in_both_themes_wide(name, WIDTH, height, draw);
}

/// The same, at a width this one thing needs.
pub fn in_both_themes_wide(
    name: &str,
    width: u16,
    height: u16,
    draw: impl Fn(&Shell, Rect, &mut Buffer),
) {
    for mode in [Mode::Starfield, Mode::Daylight] {
        let shell = shell(mode);
        let area = Rect::new(0, 0, width, height);
        let mut buffer = Buffer::empty(area);
        shell.ground(area, &mut buffer);
        draw(&shell, area, &mut buffer);
        let suffix = match mode {
            Mode::Starfield => "starfield",
            Mode::Daylight => "daylight",
        };
        assert_snapshot(&format!("{name}.{suffix}"), &dump(&shell.theme, &buffer));
    }
}

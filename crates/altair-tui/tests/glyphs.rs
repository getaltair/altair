//! The glyph table, and the width hazard it exists to contain.
//!
//! **Checked mechanically rather than by eye**, because East-Asian-Width is
//! not something a reader can see in a source file. A character is Ambiguous
//! when a CJK-aware terminal gives it two cells and everything else gives it
//! one, which `unicode-width` exposes as `width` and `width_cjk` disagreeing.
//! That disagreement is exactly the hazard, so it is also the test.

use altair_tui::ui::glyphs::{Glyphs, MARKER_CELL, STATE_CELL, Set, State};
use unicode_width::UnicodeWidthStr;

/// Whether every terminal will agree how wide this is.
fn agreed(glyph: &str) -> bool {
    glyph.width() == glyph.width_cjk()
}

#[test]
fn every_narrow_safe_glyph_is_one_terminals_agree_about() {
    let narrow = Glyphs::narrow_safe();
    for glyph in narrow.all() {
        assert!(
            agreed(glyph),
            "{glyph:?} is in the narrow-safe set and terminals disagree about \
             its width, which is the one thing that set exists to avoid"
        );
        assert_eq!(
            glyph.width(),
            1,
            "{glyph:?} is in the narrow-safe set and is not one cell wide"
        );
    }
}

#[test]
fn the_signature_set_does_carry_the_hazard() {
    // Not a complaint about the design — a statement of why the other set
    // exists. If this ever stops being true, the narrow-safe set has become
    // dead weight and should go rather than linger as decoration.
    let signature = Glyphs::signature();
    let disputed: Vec<_> = signature
        .all()
        .into_iter()
        .filter(|glyph| !agreed(glyph))
        .collect();
    assert!(
        !disputed.is_empty(),
        "no glyph in the signature set is Ambiguous any more, so the \
         narrow-safe set is no longer earning its place"
    );
}

#[test]
fn the_glyphs_no_terminal_disputes_are_the_same_in_both_sets() {
    let signature = Glyphs::signature();
    let narrow = Glyphs::narrow_safe();
    for (name, a, b) in [
        ("mark", signature.mark, narrow.mark),
        ("closed", signature.closed, narrow.closed),
        ("open", signature.open, narrow.open),
        ("crumb", signature.crumb, narrow.crumb),
        ("enter", signature.enter, narrow.enter),
        ("backspace", signature.backspace, narrow.backspace),
    ] {
        assert!(agreed(a), "{name} was listed as safe and is not");
        assert_eq!(
            a, b,
            "{name} is a glyph no terminal disputes, so substituting it damages \
             a settled design over a hazard it does not have"
        );
    }
}

#[test]
fn every_glyph_fits_the_cell_it_is_drawn_into() {
    // The rule that actually prevents drift: a column's width is declared by
    // the layout and never derived from what goes in it. Two cells is enough
    // for a glyph a terminal decides to render wide.
    for set in [Set::Signature, Set::NarrowSafe] {
        let glyphs = Glyphs::of(set);
        for state in [State::Waiting, State::Working, State::Worked] {
            let glyph = glyphs.state(state);
            assert!(
                glyph.width_cjk() <= usize::from(STATE_CELL),
                "{glyph:?} can be {} cells wide and the state column is {STATE_CELL}",
                glyph.width_cjk()
            );
        }
        for glyph in [glyphs.mark, glyphs.closed, glyphs.open, glyphs.toward] {
            assert!(
                glyph.width_cjk() <= usize::from(MARKER_CELL),
                "{glyph:?} can be {} cells wide and a marker column is {MARKER_CELL}",
                glyph.width_cjk()
            );
        }
    }
}

#[test]
fn a_state_is_always_available_in_words() {
    // Which is what lets the narrow-safe set be plain without losing meaning:
    // the glyph reinforces the word, and never replaces it.
    for state in [State::Waiting, State::Working, State::Worked] {
        assert!(!state.word().is_empty());
    }
}

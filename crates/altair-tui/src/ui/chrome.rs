//! The shell every screen sits in: the header, the section rules, the status
//! line, and the keys.
//!
//! **The chrome is the whole of the design's signature**, and it is written
//! once here so that a screen cannot invent its own. What crosses into a
//! screen is the component model's list, not what the first screen happened to
//! need.
//!
//! # Letterspacing, in a grid
//!
//! The locked design letterspaces the wordmark, the domain pill and every
//! section label. A terminal has no tracking, so the translation is a space
//! between characters — which is what letterspacing looks like when the grid
//! is the unit. [`letterspaced`] is that translation, in one place.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Widget;

use super::glyphs::Glyphs;
use super::theme::Theme;

/// What the person is doing, as the status line says it.
///
/// **Plain words on a pill**, and deliberately not a mode block: there is no
/// `NORMAL`, no `INSERT`, and nothing that reads as somebody else's editor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Modal {
    Browsing,
    Writing,
    Capturing,
    Asserting,
    Confirming,
    Erasing,
    Faults,
    Help,
}

impl Modal {
    #[must_use]
    pub const fn word(self) -> &'static str {
        match self {
            Self::Browsing => "browsing",
            Self::Writing => "writing",
            Self::Capturing => "capturing",
            Self::Asserting => "asserting",
            Self::Confirming => "confirming",
            Self::Erasing => "erasing",
            Self::Faults => "faults",
            Self::Help => "help",
        }
    }
}

/// Everything the chrome of one screen says.
///
/// One value rather than six arguments, because it is one thing: what this
/// screen is, where the person is in it, and what they can do next.
pub struct Frame<'a> {
    /// The domain pill: which of the three this screen belongs to.
    pub domain: &'a str,
    /// Quiet meta on the right of the header. **Never a backlog.**
    pub meta: &'a str,
    pub modal: Modal,
    /// The trail, outermost first. The last one is where the person is.
    pub crumbs: &'a [&'a str],
    /// Quiet meta on the right of the status line. Also never a backlog.
    pub position: &'a str,
    pub keys: &'a [(&'a str, &'a str)],
}

/// The theme and the glyph set, together, because nothing draws without both.
#[derive(Debug, Clone, Copy, Default)]
pub struct Shell {
    pub theme: Theme,
    pub glyphs: Glyphs,
}

impl Shell {
    #[must_use]
    pub const fn new(theme: Theme, glyphs: Glyphs) -> Self {
        Self { theme, glyphs }
    }

    /// The header: the mark, the wordmark, the domain, and quiet meta on the
    /// right. On every screen, unchanged.
    pub fn header(&self, domain: &str, meta: &str, area: Rect, buffer: &mut Buffer) {
        let t = &self.theme;
        let mut spans = vec![
            Span::styled(self.glyphs.mark, Style::new().fg(t.identity)),
            Span::raw(" "),
            Span::styled(
                letterspaced("ALTAIR"),
                Style::new().fg(t.bright).add_modifier(Modifier::BOLD),
            ),
            Span::raw("  "),
        ];
        if !domain.is_empty() {
            // The pill. A terminal has no radius, so what carries it is the
            // fill and a space of padding either side.
            spans.push(Span::styled(
                format!(" {} ", letterspaced(&domain.to_uppercase())),
                Style::new().fg(t.identity).bg(t.selected),
            ));
        }
        // The signature is fixed and the meta is quiet, so when the line is
        // too narrow for both it is the meta that gives way. A wordmark that
        // slid off the left would be the one thing on this line that cannot.
        let used: usize = spans.iter().map(|span| span.content.chars().count()).sum();
        let (gap, meta) = fit(usize::from(area.width), used, meta);
        spans.push(Span::raw(" ".repeat(gap)));
        spans.push(Span::styled(meta, Style::new().fg(t.dim)));
        Line::from(spans).render(area, buffer);
    }

    /// A section break: an uppercase letterspaced label, then a hairline to
    /// the end of the line. **Never a box rule** — nothing here is framed.
    pub fn section(&self, label: &str, area: Rect, buffer: &mut Buffer) {
        self.ruled(label, self.theme.dim, self.theme.rule, area, buffer);
    }

    /// A quieter break, inside a section one of these already opened.
    pub fn subsection(&self, label: &str, area: Rect, buffer: &mut Buffer) {
        self.ruled(label, self.theme.faint, self.theme.rule_faint, area, buffer);
    }

    fn ruled(
        &self,
        label: &str,
        ink: ratatui::style::Color,
        rule: ratatui::style::Color,
        area: Rect,
        buffer: &mut Buffer,
    ) {
        let written = letterspaced(&label.to_uppercase());
        let used = written.chars().count() + 1;
        let room = usize::from(area.width).saturating_sub(used);
        Line::from(vec![
            Span::styled(written, Style::new().fg(ink)),
            Span::raw(" "),
            Span::styled("─".repeat(room), Style::new().fg(rule)),
        ])
        .render(area, buffer);
    }

    /// A rule with no label, which is what closes the header.
    pub fn rule(&self, area: Rect, buffer: &mut Buffer) {
        Line::from(Span::styled(
            "─".repeat(area.width as usize),
            Style::new().fg(self.theme.rule),
        ))
        .render(area, buffer);
    }

    /// The status line: what the person is doing, where they are, and one
    /// piece of quiet meta.
    ///
    /// **Whatever goes in `meta` is not a backlog.** How many are waiting to
    /// reach the instance is not a thing this client knows how to say.
    pub fn status(
        &self,
        modal: Modal,
        crumbs: &[&str],
        meta: &str,
        area: Rect,
        buffer: &mut Buffer,
    ) {
        let t = &self.theme;
        // The discipline holds here too: amber is the one thing wanting an
        // answer, red is only what cannot be taken back, blue is everything
        // else.
        let pill = match modal {
            Modal::Confirming => t.attention,
            Modal::Erasing | Modal::Faults => t.refused,
            _ => t.identity,
        };
        let word = format!(" {} ", letterspaced(&modal.word().to_uppercase()));
        let mut spans = vec![
            Span::styled(word.clone(), Style::new().fg(t.ground).bg(pill)),
            Span::raw(" "),
        ];

        // **The trail is what gives way**, and it gives way from the far end.
        // What the person is looking at is the last crumb, so that one is kept
        // and the path to it is dropped a step at a time; the pill and the
        // meta on the right are fixed. Without this the two ends collide and
        // the person is shown a word with a number welded onto it.
        let fixed = word.chars().count() + 1 + meta.chars().count() + 1;
        let mut room = usize::from(area.width).saturating_sub(fixed);
        let mut kept: Vec<&str> = Vec::new();
        for crumb in crumbs.iter().rev() {
            let step = crumb.chars().count() + if kept.is_empty() { 0 } else { 3 };
            if step > room {
                break;
            }
            room -= step;
            kept.push(crumb);
        }
        kept.reverse();
        let dropped = crumbs.len() - kept.len();
        if dropped > 0 && !kept.is_empty() {
            spans.push(Span::styled("...", Style::new().fg(t.faint)));
            spans.push(Span::styled(
                format!(" {} ", self.glyphs.crumb),
                Style::new().fg(t.faint),
            ));
        }
        for (index, crumb) in kept.iter().enumerate() {
            let last = index + 1 == kept.len();
            if index > 0 {
                spans.push(Span::styled(
                    format!(" {} ", self.glyphs.crumb),
                    Style::new().fg(t.faint),
                ));
            }
            spans.push(Span::styled(
                (*crumb).to_string(),
                Style::new().fg(if last { t.body } else { t.dim }),
            ));
        }
        let used: usize = spans.iter().map(|span| span.content.chars().count()).sum();
        let (gap, meta) = fit(usize::from(area.width), used, meta);
        spans.push(Span::raw(" ".repeat(gap)));
        spans.push(Span::styled(meta, Style::new().fg(t.faint)));
        Line::from(spans).render(area, buffer);
    }

    /// The keys, as real keycaps with plain-language labels.
    ///
    /// **Plain language, and no hint string.** There is no `j k h l`, and a
    /// label says what happens rather than naming a motion.
    pub fn keys(&self, keys: &[(&str, &str)], area: Rect, buffer: &mut Buffer) {
        let t = &self.theme;
        let mut spans = Vec::new();
        for (cap, says) in keys {
            if !spans.is_empty() {
                spans.push(Span::raw("   "));
            }
            spans.push(Span::styled(
                format!(" {cap} "),
                Style::new().fg(t.body).bg(t.keycap),
            ));
            spans.push(Span::raw(" "));
            spans.push(Span::styled((*says).to_string(), Style::new().fg(t.dim)));
        }
        Line::from(spans).render(area, buffer);
    }

    /// Draw a whole screen: the chrome, then whatever goes between it.
    ///
    /// **Every screen is framed the same way and none of them draws its own
    /// chrome.** The header, the status line and the keys are the design's
    /// signature, and a screen that assembled its own would be where the
    /// signature starts to drift — one screen at a time, each change
    /// defensible on its own.
    pub fn frame(
        &self,
        frame: &Frame<'_>,
        area: Rect,
        buffer: &mut Buffer,
        body: impl FnOnce(Rect, &mut Buffer),
    ) {
        // **A terminal can be any size, including none.** A person dragging a
        // window narrow, a multiplexer splitting a pane to nothing, a pty
        // opened before anyone said how big it is — all of them arrive here as
        // an area with no room in it, and the client's answer has to be to draw
        // less rather than to fall over. Nothing below subtracts from a
        // dimension without knowing there is something to subtract from.
        if area.width == 0 || area.height == 0 {
            return;
        }
        let line = |y: u16| Rect {
            y,
            height: 1,
            ..area
        };
        self.ground(area, buffer);
        self.header(frame.domain, frame.meta, line(area.top()), buffer);
        if area.height >= 2 {
            self.rule(line(area.top() + 1), buffer);
        }
        // The chrome is two lines at the top and two at the bottom; the body is
        // whatever is left, and there may be none.
        if area.height > 6 {
            body(
                Rect {
                    y: area.top() + 3,
                    height: area.height - 6,
                    ..area
                },
                buffer,
            );
        }
        if area.height >= 4 {
            self.status(
                frame.modal,
                frame.crumbs,
                frame.position,
                line(area.bottom() - 2),
                buffer,
            );
        }
        if area.height >= 3 {
            self.keys(frame.keys, line(area.bottom() - 1), buffer);
        }
    }

    /// The help surface, drawn.
    ///
    /// **The promises come first and the keys come second**, because a person
    /// who opens this while something looks wrong is asking what the client
    /// guarantees, not which key does what.
    pub fn help_surface(&self, area: Rect, buffer: &mut Buffer) {
        let t = &self.theme;
        let mut y = area.top();
        let line = |y: u16| Rect {
            y,
            height: 1,
            ..area
        };
        self.section("what this client promises", line(y), buffer);
        y += 2;
        for promise in super::help::PROMISES {
            if y >= area.bottom() {
                return;
            }
            Line::from(Span::styled(promise.says, Style::new().fg(t.bright)))
                .render(line(y), buffer);
            y += 1;
            for wrapped in wrap(promise.means, usize::from(area.width).saturating_sub(2)) {
                if y >= area.bottom() {
                    return;
                }
                Line::from(vec![
                    Span::raw("  "),
                    Span::styled(wrapped, Style::new().fg(t.mid)),
                ])
                .render(line(y), buffer);
                y += 1;
            }
            y += 1;
        }
        for group in super::help::KEYS {
            if y + 1 >= area.bottom() {
                return;
            }
            self.subsection(group.about, line(y), buffer);
            y += 1;
            for (cap, says) in group.keys {
                if y >= area.bottom() {
                    return;
                }
                self.keys(&[(cap, says)], line(y), buffer);
                y += 1;
            }
            y += 1;
        }
    }

    /// One row of a list, indented by hairlines rather than by a glyph tree.
    ///
    /// `depth` is how many levels in, and each one is a hairline and the space
    /// after it. Nothing draws a corner, a tee, or a branch: the design holds
    /// an indent with a rule, and a tree drawn in box characters is the thing
    /// it is deliberately not.
    #[must_use]
    pub fn indent(&self, depth: usize) -> Line<'static> {
        let mut spans = Vec::new();
        for _ in 0..depth {
            spans.push(Span::styled(
                self.glyphs.hairline,
                Style::new().fg(self.theme.rule),
            ));
            spans.push(Span::raw("  "));
        }
        Line::from(spans)
    }

    /// Fill an area with the ground, so a sheet drawn over a screen actually
    /// covers it.
    pub fn ground(&self, area: Rect, buffer: &mut Buffer) {
        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                if let Some(cell) = buffer.cell_mut((x, y)) {
                    cell.set_char(' ').set_bg(self.theme.ground);
                }
            }
        }
    }

    /// An inset sheet: a dialog, an overlay. One hairline border and a fill a
    /// shade off the ground.
    pub fn sheet(&self, title: &str, area: Rect, buffer: &mut Buffer) {
        let t = &self.theme;
        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                if let Some(cell) = buffer.cell_mut((x, y)) {
                    cell.set_char(' ').set_bg(t.sheet).set_fg(t.body);
                }
            }
        }
        let edge = Style::new().fg(t.keycap_border).bg(t.sheet);
        let width = area.width as usize;
        if area.height >= 2 && width >= 2 {
            let heading = format!(" {} ", letterspaced(&title.to_uppercase()));
            let across = width
                .saturating_sub(2)
                .saturating_sub(heading.chars().count());
            Line::from(vec![
                Span::styled("┌─", edge),
                Span::styled(heading, Style::new().fg(t.mid).bg(t.sheet)),
                Span::styled(format!("{}┐", "─".repeat(across)), edge),
            ])
            .render(Rect { height: 1, ..area }, buffer);
            Line::from(Span::styled(
                format!("└{}┘", "─".repeat(width.saturating_sub(2))),
                edge,
            ))
            .render(
                Rect {
                    y: area.bottom() - 1,
                    height: 1,
                    ..area
                },
                buffer,
            );
            for y in area.top() + 1..area.bottom() - 1 {
                if let Some(cell) = buffer.cell_mut((area.left(), y)) {
                    cell.set_char('│').set_fg(t.keycap_border).set_bg(t.sheet);
                }
                if let Some(cell) = buffer.cell_mut((area.right() - 1, y)) {
                    cell.set_char('│').set_fg(t.keycap_border).set_bg(t.sheet);
                }
            }
        }
    }
}

/// How much space is left before the meta, and how much of the meta fits.
///
/// **Nothing on one of these lines is allowed to run into anything else.** A
/// line that overflows is truncated by the terminal at the right edge, which
/// silently eats whichever piece happened to be last and leaves two words
/// welded together where the seam was.
fn fit(width: usize, used: usize, meta: &str) -> (usize, String) {
    let room = width.saturating_sub(used);
    if room == 0 {
        return (0, String::new());
    }
    let wanted = meta.chars().count();
    if wanted < room {
        return (room - wanted, meta.to_string());
    }
    // One space of separation, then as much of the meta as is left.
    (1, meta.chars().take(room - 1).collect())
}

/// A space between characters, which is what letterspacing is in a grid.
///
/// **A word gap has to grow too.** Tracking widens the space between letters
/// and the space between words by the same proportion; in a grid, where a
/// letter gap becomes one column, a word gap that stays one column is
/// indistinguishable from a letter gap and the label reads as one long word.
/// So a word gap becomes three.
#[must_use]
pub fn letterspaced(text: &str) -> String {
    let mut out = String::new();
    let mut following_space = false;
    for (index, character) in text.chars().enumerate() {
        if character == ' ' {
            following_space = true;
            continue;
        }
        if index > 0 {
            out.push_str(if following_space { "   " } else { " " });
        }
        following_space = false;
        out.push(character);
    }
    out
}

/// Break prose at word boundaries, because a person reads this one.
fn wrap(text: &str, width: usize) -> Vec<String> {
    let mut out = Vec::new();
    let mut line = String::new();
    for word in text.split_whitespace() {
        if !line.is_empty() && line.chars().count() + 1 + word.chars().count() > width {
            out.push(std::mem::take(&mut line));
        }
        if !line.is_empty() {
            line.push(' ');
        }
        line.push_str(word);
    }
    if !line.is_empty() {
        out.push(line);
    }
    out
}

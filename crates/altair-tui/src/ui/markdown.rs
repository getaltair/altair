//! A note's body, drawn.
//!
//! # What this is not
//!
//! **It is not storage and it never becomes canonical.** The markdown a person
//! wrote is the body; everything here is a way of looking at it. Nothing
//! rendered is ever read back, edited, or sent — DR-001 requires that nothing
//! forecloses export, and a client that round-tripped prose through its own
//! renderer would have made the rendering authoritative without saying so.
//!
//! **It is not division.** The instance divides a body into blocks, once, and
//! that is the only implementation of the rule — two devices disagreeing about
//! the units reconciliation happens in is exactly what one implementation
//! prevents. Parsing here is for display and produces nothing anybody else
//! sees. `tests/markdown.rs` checks that no part of the send path can reach
//! this module.
//!
//! It does parse under the same options the instance divides under, so what a
//! person sees cannot disagree with how their body was cut up. That coupling
//! is one-way and is pinned from the other side, in the instance's own tests.
//!
//! # Colour, and the one restraint worth stating
//!
//! **A markdown link is not rendered blue.** Blue means identity and relations
//! and nothing else, so that seeing blue in a paragraph means the person formed
//! a relation there. A hyperlink that borrowed the colour would make that
//! signal meaningless, so links are underlined and otherwise ordinary — and the
//! blue underlined runs in a body are relations, which is what the design draws.

use std::ops::Range;

use altair_proto::v1;
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use unicode_width::UnicodeWidthStr;

use super::chrome::Shell;

/// The options a body is parsed under.
///
/// **The same set the instance divides under**, and a constant for the same
/// reason it is one there: parsing under a different set finds different
/// constructs, and a table shown as a paragraph of pipes would be this client
/// disagreeing with its own household about what the person wrote.
fn options() -> Options {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);
    options
}

/// Where in a body relations were formed.
///
/// **Byte ranges, because that is what survives rendering.** A relation anchors
/// to a block and a phrase within it; blocks concatenate to reproduce the body
/// exactly, so walking them in order gives each phrase a position in the body
/// itself, which is the one thing every span drawn below can be checked
/// against. A renderer that lost the mapping from output back to source could
/// not draw this at all.
#[must_use]
pub fn formed_at(
    body: &str,
    blocks: &[v1::Block],
    relations: &[v1::Relation],
) -> Vec<Range<usize>> {
    // **The offsets below are block lengths added up, so they are only
    // positions in this body while the blocks are this body's.** They are
    // supposed to be: a division reproduces the source byte for byte. But
    // blocks arrive from the instance and the body can be a local edit made
    // since, and then every offset here is a little bit wrong — which shows up
    // as an underline sitting across the middle of the wrong words. Marking
    // nothing is the better answer than marking the wrong thing.
    let held: usize = blocks.iter().map(|block| block.text.len()).sum();
    if held != body.len() {
        return Vec::new();
    }

    let mut out = Vec::new();
    let mut at = 0_usize;
    for block in blocks {
        let span = at..at + block.text.len();
        for relation in relations {
            let Some(anchor) = relation.content.as_ref().and_then(|c| c.anchor.as_ref()) else {
                continue;
            };
            // An anchor on the block itself, rather than at a phrase, marks no
            // run of text: the whole block is where it was formed, and there is
            // nothing to underline inside it.
            if anchor.block_id != block.block_id || anchor.phrase.is_empty() {
                continue;
            }
            if let Some(found) = block.text.find(&anchor.phrase) {
                let start = span.start + found;
                out.push(start..start + anchor.phrase.len());
            }
        }
        at = span.end;
    }
    out.sort_by_key(|range| range.start);
    out
}

/// Draw a body.
///
/// Answers the lines to put on screen, wrapped to `width`.
#[must_use]
pub fn render(
    shell: &Shell,
    body: &str,
    formed: &[Range<usize>],
    width: u16,
) -> Vec<Line<'static>> {
    let mut drawing = Drawing {
        shell,
        body,
        formed,
        width: usize::from(width).max(8),
        lines: Vec::new(),
        inline: Vec::new(),
        styles: Vec::new(),
        prefix: Vec::new(),
        marker: None,
        listing: Vec::new(),
        verbatim: None,
        table: None,
    };
    for (event, at) in Parser::new_ext(body, options()).into_offset_iter() {
        drawing.take(&event, &at);
    }
    drawing.flush();
    // Constructs each close by leaving a gap after themselves, and two that
    // end together leave two. One is a paragraph break; more is a hole.
    let mut lines: Vec<Line<'static>> = Vec::new();
    for line in drawing.lines {
        let blank = line.spans.iter().all(|span| span.content.trim().is_empty());
        let after_blank = lines
            .last()
            .is_some_and(|last: &Line<'_>| last.spans.iter().all(|s| s.content.trim().is_empty()));
        if blank && (after_blank || lines.is_empty()) {
            continue;
        }
        lines.push(line);
    }
    while lines
        .last()
        .is_some_and(|line| line.spans.iter().all(|s| s.content.trim().is_empty()))
    {
        lines.pop();
    }
    lines
}

/// Where a list is up to.
enum Listing {
    Bulleted,
    Numbered(u64),
}

/// A table being collected before anything about it can be drawn: column widths
/// are a fact about every row at once.
struct Collecting {
    head: Vec<Vec<Span<'static>>>,
    rows: Vec<Vec<Vec<Span<'static>>>>,
    heading: bool,
}

struct Drawing<'a> {
    shell: &'a Shell,
    body: &'a str,
    formed: &'a [Range<usize>],
    width: usize,
    lines: Vec<Line<'static>>,
    /// The run being built, before it is wrapped.
    inline: Vec<Span<'static>>,
    styles: Vec<Style>,
    /// What every line of the current block begins with: the hairline of a
    /// quote, the indent of a list.
    prefix: Vec<Span<'static>>,
    /// A list item's marker, drawn on its first line only.
    marker: Option<Span<'static>>,
    listing: Vec<Listing>,
    /// A code fence, kept verbatim: it is the one place wrapping would change
    /// what the person wrote.
    verbatim: Option<String>,
    table: Option<Collecting>,
}

impl Drawing<'_> {
    fn theme(&self) -> super::theme::Theme {
        self.shell.theme
    }

    fn style(&self) -> Style {
        self.styles
            .last()
            .copied()
            .unwrap_or_else(|| Style::new().fg(self.theme().body))
    }

    fn push(&mut self, style: Style) {
        self.styles.push(style);
    }

    fn pop(&mut self) {
        self.styles.pop();
    }

    fn take(&mut self, event: &Event<'_>, at: &Range<usize>) {
        match event {
            Event::Start(tag) => self.open(tag),
            Event::End(tag) => self.close(*tag),
            Event::Text(text) => {
                if let Some(code) = self.verbatim.as_mut() {
                    code.push_str(text);
                } else {
                    self.words(text, at);
                }
            }
            Event::Code(text) => {
                let style = Style::new().fg(self.theme().body).bg(self.theme().keycap);
                self.inline.push(Span::styled(format!(" {text} "), style));
            }
            Event::SoftBreak => self.inline.push(Span::raw(" ")),
            Event::HardBreak => {
                let run = std::mem::take(&mut self.inline);
                self.wrap(run);
            }
            Event::Rule => {
                self.flush();
                self.lines.push(Line::from(Span::styled(
                    "─".repeat(self.width),
                    Style::new().fg(self.theme().rule),
                )));
                self.lines.push(Line::default());
            }
            Event::TaskListMarker(done) => {
                // Plainly, and never with the guidance state family: waiting,
                // working and worked mean something precise in this system, and
                // a shopping list is not a quest.
                let style = Style::new().fg(if *done {
                    self.theme().worked
                } else {
                    self.theme().dim
                });
                // It replaces the item's bullet rather than following it: a
                // dash and a box say the same thing twice, and the box is the
                // one carrying information.
                self.marker = Some(Span::styled(
                    if *done { "[x] " } else { "[ ] " }.to_string(),
                    style,
                ));
            }
            _ => {}
        }
    }

    fn open(&mut self, tag: &Tag<'_>) {
        let theme = self.theme();
        match tag {
            Tag::Heading { level, .. } => {
                self.flush();
                // Weight and brightness, and no `#` left in the text: the
                // person wrote structure, not punctuation to look at.
                let style = match level {
                    HeadingLevel::H1 | HeadingLevel::H2 => {
                        Style::new().fg(theme.bright).add_modifier(Modifier::BOLD)
                    }
                    _ => Style::new().fg(theme.body).add_modifier(Modifier::BOLD),
                };
                self.push(style);
            }
            Tag::BlockQuote(_) => {
                self.flush();
                // A hairline holds the indent, as it does everywhere else in
                // this client. Never a `>` and never a frame.
                self.prefix.push(Span::styled(
                    format!("{}  ", self.shell.glyphs.hairline),
                    Style::new().fg(theme.rule),
                ));
                self.push(Style::new().fg(theme.mid));
            }
            Tag::CodeBlock(_) => {
                self.flush();
                self.verbatim = Some(String::new());
            }
            Tag::List(start) => {
                // **Flushed before the nesting changes, not after.** A tight
                // list item's text arrives as bare events and a nested list
                // opens straight after it, so waiting until the item ends
                // welds the parent's words onto the first child's — and by
                // then the depth is the child's too, which indents the parent
                // and not the child. Both of those were visible in one line of
                // a snapshot.
                self.flush();
                self.listing.push(match start {
                    Some(first) => Listing::Numbered(*first),
                    None => Listing::Bulleted,
                });
            }
            Tag::Item => {
                let marker = match self.listing.last_mut() {
                    Some(Listing::Numbered(next)) => {
                        let says = format!("{next}. ");
                        *next += 1;
                        says
                    }
                    // A plain dash, in both glyph sets. It is what a person
                    // types, no terminal disputes its width, and it makes no
                    // claim the design's own marks are reserved for.
                    _ => "- ".to_string(),
                };
                self.marker = Some(Span::styled(marker, Style::new().fg(theme.dim)));
            }
            Tag::Emphasis => self.push(self.style().add_modifier(Modifier::ITALIC)),
            Tag::Strong => self.push(self.style().add_modifier(Modifier::BOLD)),
            Tag::Strikethrough => self.push(self.style().add_modifier(Modifier::CROSSED_OUT)),
            // Underlined and otherwise ordinary. Blue would make it look like a
            // relation, and then blue would stop meaning one.
            Tag::Link { .. } => self.push(self.style().add_modifier(Modifier::UNDERLINED)),
            Tag::Image { .. } => {
                self.push(Style::new().fg(theme.faint));
                self.inline
                    .push(Span::styled("[image] ", Style::new().fg(theme.faint)));
            }
            Tag::Table(_) => {
                self.flush();
                self.table = Some(Collecting {
                    head: Vec::new(),
                    rows: Vec::new(),
                    heading: false,
                });
            }
            Tag::TableHead => {
                if let Some(table) = self.table.as_mut() {
                    table.heading = true;
                }
            }
            Tag::TableRow => {
                if let Some(table) = self.table.as_mut()
                    && !table.heading
                {
                    table.rows.push(Vec::new());
                }
            }
            _ => {}
        }
    }

    fn close(&mut self, tag: TagEnd) {
        match tag {
            TagEnd::Heading(_) => {
                self.flush();
                self.pop();
                self.lines.push(Line::default());
            }
            TagEnd::Paragraph => {
                self.flush();
                if self.listing.is_empty() {
                    self.lines.push(Line::default());
                }
            }
            TagEnd::BlockQuote(_) => {
                self.flush();
                self.pop();
                self.prefix.pop();
                self.lines.push(Line::default());
            }
            TagEnd::CodeBlock => {
                let code = self.verbatim.take().unwrap_or_default();
                let style = Style::new().fg(self.theme().body).bg(self.theme().keycap);
                let indent: usize = self.prefix.iter().map(|span| span.content.width()).sum();
                // **Clipped rather than wrapped.** A fence is the one place
                // where re-flowing changes what the person wrote — a broken
                // line of code is a different line of code — so a line too
                // long for the column is cut and says so. It is padded to the
                // same room either way, so the fence reads as one surface
                // rather than as ragged coloured text.
                let room = self.width.saturating_sub(indent + 2).max(4);
                for line in code.lines() {
                    let mut spans = self.prefix.clone();
                    let shown = clipped(line, room);
                    let short = room - shown.width();
                    spans.push(Span::styled(
                        format!(" {shown}{} ", " ".repeat(short)),
                        style,
                    ));
                    self.lines.push(Line::from(spans));
                }
                self.lines.push(Line::default());
            }
            TagEnd::List(_) => {
                self.listing.pop();
                if self.listing.is_empty() {
                    self.lines.push(Line::default());
                }
            }
            TagEnd::Item => self.flush(),
            TagEnd::Emphasis
            | TagEnd::Strong
            | TagEnd::Strikethrough
            | TagEnd::Link
            | TagEnd::Image => self.pop(),
            TagEnd::TableCell => {
                let cell = std::mem::take(&mut self.inline);
                if let Some(table) = self.table.as_mut() {
                    if table.heading {
                        table.head.push(cell);
                    } else if let Some(row) = table.rows.last_mut() {
                        row.push(cell);
                    }
                }
            }
            TagEnd::TableHead => {
                if let Some(table) = self.table.as_mut() {
                    table.heading = false;
                }
            }
            TagEnd::Table => self.draw_table(),
            _ => {}
        }
    }

    /// Text, split where a relation was formed in it.
    fn words(&mut self, text: &str, at: &Range<usize>) {
        let style = self.style();
        // Only split where the event's own source says the same thing it does.
        // An entity reference or an escape makes the two differ, and guessing
        // an offset into text that is not the source is how an underline lands
        // in the middle of a word.
        let faithful = self.body.get(at.clone()).is_some_and(|slice| slice == text);
        if !faithful {
            self.inline.push(Span::styled(text.to_string(), style));
            return;
        }
        let relation = Style::new()
            .fg(self.theme().identity)
            .add_modifier(Modifier::UNDERLINED);
        let mut cut = at.start;
        for formed in self.formed {
            let start = formed.start.max(at.start);
            let end = formed.end.min(at.end);
            if start >= end || start < cut {
                continue;
            }
            if start > cut {
                self.inline
                    .push(Span::styled(self.body[cut..start].to_string(), style));
            }
            self.inline
                .push(Span::styled(self.body[start..end].to_string(), relation));
            cut = end;
        }
        if cut < at.end {
            self.inline
                .push(Span::styled(self.body[cut..at.end].to_string(), style));
        }
    }

    /// Wrap what has been built and put it on the page.
    fn flush(&mut self) {
        let run = std::mem::take(&mut self.inline);
        if run.iter().all(|span| span.content.trim().is_empty()) {
            self.marker = None;
            return;
        }
        self.wrap(run);
    }

    fn wrap(&mut self, run: Vec<Span<'static>>) {
        let depth = self.listing.len().saturating_sub(1);
        let indent: usize = self.prefix.iter().map(|span| span.content.width()).sum();
        let hang = self.marker.as_ref().map_or(0, |m| m.content.width()) + depth * 2;
        let room = self.width.saturating_sub(indent + hang).max(8);

        for (index, line) in wrapped(&run, room).into_iter().enumerate() {
            let mut spans = self.prefix.clone();
            if depth > 0 {
                spans.push(Span::raw("  ".repeat(depth)));
            }
            match (index, self.marker.take()) {
                // The marker sits on the first line and the rest hangs under
                // it, which is what makes a wrapped list item read as one item.
                (0, Some(marker)) => spans.push(marker),
                _ => spans.push(Span::raw(" ".repeat(hang.saturating_sub(depth * 2)))),
            }
            spans.extend(line);
            self.lines.push(Line::from(spans));
        }
        self.marker = None;
    }

    /// A table, once every row is known.
    ///
    /// **No rules and no frame.** The head carries its own ground, which is how
    /// the design separates it, and the columns are held apart by space.
    fn draw_table(&mut self) {
        let Some(table) = self.table.take() else {
            return;
        };
        let columns = table
            .head
            .len()
            .max(table.rows.iter().map(Vec::len).max().unwrap_or(0));
        if columns == 0 {
            return;
        }
        let mut widths = vec![0_usize; columns];
        for row in std::iter::once(&table.head).chain(table.rows.iter()) {
            for (index, cell) in row.iter().enumerate() {
                let wide: usize = cell.iter().map(|span| span.content.width()).sum();
                widths[index] = widths[index].max(wide);
            }
        }
        // Fair shares when there is not enough room, rather than one column
        // taking everything and the rest vanishing.
        let total: usize = widths.iter().sum::<usize>() + columns * 2;
        if total > self.width {
            let each = (self.width / columns).saturating_sub(2).max(4);
            for width in &mut widths {
                *width = (*width).min(each);
            }
        }

        let head = Style::new()
            .fg(self.theme().mid)
            .bg(self.theme().table_head);
        if !table.head.is_empty() {
            self.lines
                .push(Line::from(laid_out(&table.head, &widths, Some(head))));
        }
        for row in &table.rows {
            self.lines.push(Line::from(laid_out(row, &widths, None)));
        }
        self.lines.push(Line::default());
    }
}

/// One row of a table, padded into its columns.
fn laid_out(
    row: &[Vec<Span<'static>>],
    widths: &[usize],
    ground: Option<Style>,
) -> Vec<Span<'static>> {
    let mut spans = Vec::new();
    for (index, width) in widths.iter().enumerate() {
        let empty = Vec::new();
        let cell = row.get(index).unwrap_or(&empty);
        let mut used = 0;
        spans.push(pad(" ", ground));
        for span in cell {
            let room = width.saturating_sub(used);
            if room == 0 {
                break;
            }
            let text = clipped(&span.content, room);
            used += text.width();
            let style = ground.map_or(span.style, |g| span.style.bg(g.bg.unwrap_or_default()));
            spans.push(Span::styled(text, style));
        }
        spans.push(pad(&" ".repeat(width.saturating_sub(used) + 1), ground));
    }
    spans
}

fn pad(text: &str, ground: Option<Style>) -> Span<'static> {
    Span::styled(text.to_string(), ground.unwrap_or_default())
}

fn clipped(text: &str, width: usize) -> String {
    if text.width() <= width {
        return text.to_string();
    }
    let mut out = String::new();
    for character in text.chars() {
        if out.width() + character.to_string().width() > width {
            break;
        }
        out.push(character);
    }
    out
}

/// Break a styled run into lines no wider than `width`.
///
/// **Styled spans are wrapped, not text.** Wrapping a string and styling it
/// afterwards loses which words were emphasised; a word that straddles a break
/// has to keep its style on both sides of it, which is the whole reason this is
/// not one call to a string helper.
fn wrapped(run: &[Span<'static>], width: usize) -> Vec<Vec<Span<'static>>> {
    let mut lines: Vec<Vec<Span<'static>>> = Vec::new();
    let mut line: Vec<Span<'static>> = Vec::new();
    let mut used = 0_usize;

    for span in run {
        for word in split(&span.content) {
            let wide = word.width();
            if word.trim().is_empty() {
                // Space at a break is dropped rather than carried onto the next
                // line, where it would show as an indent nobody asked for.
                if used > 0 && used + wide <= width {
                    used += wide;
                    line.push(Span::styled(word, span.style));
                }
                continue;
            }
            if used + wide > width && used > 0 {
                lines.push(std::mem::take(&mut line));
                used = 0;
            }
            if wide > width {
                // A word longer than the line: broken, because the alternative
                // is a line that runs off the edge and takes the column after
                // it with it.
                for piece in pieces(&word, width) {
                    let piece_width = piece.width();
                    if used + piece_width > width && used > 0 {
                        lines.push(std::mem::take(&mut line));
                        used = 0;
                    }
                    used += piece_width;
                    line.push(Span::styled(piece, span.style));
                }
                continue;
            }
            used += wide;
            line.push(Span::styled(word, span.style));
        }
    }
    if !line.is_empty() {
        lines.push(line);
    }
    if lines.is_empty() {
        lines.push(Vec::new());
    }
    lines
}

/// Words and the runs of space between them, in order.
fn split(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut current = String::new();
    let mut spacing = false;
    for character in text.chars() {
        let is_space = character.is_whitespace();
        if !current.is_empty() && is_space != spacing {
            out.push(std::mem::take(&mut current));
        }
        spacing = is_space;
        current.push(character);
    }
    if !current.is_empty() {
        out.push(current);
    }
    out
}

fn pieces(word: &str, width: usize) -> Vec<String> {
    let mut out = Vec::new();
    let mut current = String::new();
    for character in word.chars() {
        if current.width() + character.to_string().width() > width {
            out.push(std::mem::take(&mut current));
        }
        current.push(character);
    }
    if !current.is_empty() {
        out.push(current);
    }
    out
}

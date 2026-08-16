//! Dividing a body into blocks.
//!
//! # What this function owes
//!
//! From `docs/altair-substrate-spec.md`, "How a body divides into blocks":
//!
//! - *"The division is structural, and derived from the text alone."* — the
//!   only input is the body. No clock, no locale, no hash iteration order, no
//!   floating point, no configuration read at runtime. The parser options are
//!   a fixed set ([`options`]) precisely because changing them
//!   changes boundaries, which is a decision and not a knob.
//! - *"Some constructs are atomic and never split, whatever blank lines they
//!   contain and however large they are. A fenced code block, a diagram, a
//!   table."* — this function descends into exactly one construct, the list,
//!   and treats every other top-level construct as one block. A mermaid
//!   diagram is a fenced code block and is covered by that rule.
//! - *"Some constructs contain independently editable parts and split at them.
//!   A list item is a block while the list holds together around it."*
//! - *"Accepted limitation: a long unbroken stretch of prose is one block."* —
//!   a paragraph is one block however long. This is correct, not a defect, and
//!   [`crate::body`]'s tests pin it so a later reader does not helpfully split
//!   on sentences.
//!
//! # Spans partition the body exactly
//!
//! Every byte of a non-blank body belongs to exactly one block. A block's
//! [`Division::text`] is the verbatim source slice, and concatenating the
//! blocks in order reproduces the body byte for byte. Nothing may foreclose
//! export (DR-001), and a division that dropped the author's blank lines,
//! indentation, or tight/loose list structure would be a lossy one.
//!
//! Whitespace between two blocks is attributed to the block before it; a
//! leading gap is attributed to the block after it, because there is nothing
//! before it. [`Division::content`] is the same slice with that surrounding
//! blank space trimmed off, and is what identity matching compares — a block
//! is not "reworded" because somebody pressed return underneath it.

use std::ops::Range;

use pulldown_cmark::{Event, Options, Parser, Tag};

/// The parser options division is computed under.
///
/// This is a constant rather than a parameter. Two instances parsing under
/// different options would compute different boundaries from the same text,
/// which is the exact failure the "derived from the text alone" rule exists to
/// prevent. Changing this set changes how existing bodies divide, so it is a
/// decision with a migration attached, not a tuning knob.
///
/// - `ENABLE_TABLES` — a table is named in the spec as atomic, and without
///   this a table is parsed as an ordinary paragraph of pipes.
/// - `ENABLE_FOOTNOTES` — a footnote definition is a top-level construct; with
///   the extension off it becomes an unremarkable paragraph, which is still one
///   block, but with it on the definition stays whole when it spans a blank
///   line.
/// - `ENABLE_TASKLISTS` — `- [ ] milk` is a list item, and a shopping list is
///   the named motivating case for splitting list items.
fn options() -> Options {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);
    options
}

/// What kind of construct a block came from.
///
/// This is not persisted and does not cross the wire — the contract's `Block`
/// carries an identifier and text and nothing else. It exists so that tests can
/// assert atomicity directly rather than by inspecting text, and so a reader
/// can see which rule produced a boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BlockKind {
    Paragraph,
    Heading,
    /// A fenced code block. A mermaid diagram is one of these.
    FencedCode,
    IndentedCode,
    Table,
    BlockQuote,
    HtmlBlock,
    ThematicBreak,
    FootnoteDefinition,
    MetadataBlock,
    DefinitionList,
    /// An entry of a list, at any depth.
    ListItem,
    /// Content of a list item resuming after a nested list closed.
    ListItemTail,
}

/// One block of a divided body.
///
/// Borrows the body it was divided from; `span` and `content` are byte ranges
/// into that body.
#[derive(Clone, Debug)]
pub struct Division<'a> {
    body: &'a str,
    kind: BlockKind,
    span: Range<usize>,
    content: Range<usize>,
}

impl<'a> Division<'a> {
    /// Which rule produced this block.
    pub fn kind(&self) -> BlockKind {
        self.kind
    }

    /// The block's byte range in the body. Spans are contiguous and cover the
    /// whole body.
    pub fn span(&self) -> Range<usize> {
        self.span.clone()
    }

    /// The block's text, verbatim, including any blank space attributed to it.
    ///
    /// This is what a block row holds. Concatenating this over the blocks in
    /// order reproduces the body exactly, which is what makes a body "its
    /// blocks in order" true rather than approximately true.
    pub fn text(&self) -> &'a str {
        &self.body[self.span.clone()]
    }

    /// The block's byte range with surrounding blank space trimmed.
    pub fn content_span(&self) -> Range<usize> {
        self.content.clone()
    }

    /// The block's text with surrounding blank space trimmed.
    ///
    /// Identity matching compares this, so that adding a blank line after a
    /// paragraph is not mistaken for rewriting it. Leading indentation on the
    /// first line is *kept*, because indentation is what tells a nested list
    /// item from a top-level one and an indented code block from a paragraph.
    pub fn content(&self) -> &'a str {
        &self.body[self.content.clone()]
    }

    /// True where this block is entirely blank space. Only possible for a body
    /// that is entirely blank, which produces no blocks at all, so this is
    /// always false in practice and exists for the property tests to say so.
    pub fn is_blank(&self) -> bool {
        self.content.is_empty()
    }
}

/// Divide a body into blocks.
///
/// Pure: the same text always yields the same boundaries, on every machine and
/// every run.
///
/// A body that is empty or entirely whitespace has no blocks. That is the one
/// case where concatenating the blocks does not reproduce the input, and it is
/// the right answer: an empty body has no independently addressable parts.
pub fn divide(body: &str) -> Vec<Division<'_>> {
    if body.trim().is_empty() {
        return Vec::new();
    }

    let starts = boundaries(body);
    let mut blocks = Vec::with_capacity(starts.len());
    for (index, (start, kind)) in starts.iter().enumerate() {
        // Whitespace between blocks belongs to the block before it; the gap
        // before the first block belongs to the first block, there being
        // nothing before it. Together these make the spans a partition.
        let from = if index == 0 { 0 } else { *start };
        let to = starts
            .get(index + 1)
            .map(|(next, _)| *next)
            .unwrap_or(body.len());
        let content = trim_span(body, from..to);
        blocks.push(Division {
            body,
            kind: *kind,
            span: from..to,
            content,
        });
    }
    blocks
}

/// Reduce a stored block's text to the same form [`Division::content`] returns.
///
/// A block row holds the verbatim span. Identity matching compares content, so
/// the stored side has to be reduced by the identical rule or the two sides are
/// not comparable.
pub fn content_of(text: &str) -> &str {
    let range = trim_span(text, 0..text.len());
    &text[range]
}

/// Which frame of the parse we are inside.
///
/// The walk descends into exactly one thing — a list, so that its items become
/// blocks — and treats everything else as opaque. `Opaque` is what makes
/// "atomic constructs never split" true by construction rather than by a rule
/// applied afterwards: once inside a table, a quote, or a code block, no
/// boundary can be emitted until it closes.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Frame {
    List,
    /// A list item. Carries whether a nested list has already closed inside it,
    /// which decides whether the item's remaining content starts a new block.
    Item {
        after_sublist: bool,
    },
    Opaque,
}

fn boundaries(body: &str) -> Vec<(usize, BlockKind)> {
    let mut starts: Vec<(usize, BlockKind)> = Vec::new();
    let mut stack: Vec<Frame> = Vec::new();

    let emit = |offset: usize, kind: BlockKind, starts: &mut Vec<(usize, BlockKind)>| {
        let at = line_start(body, offset);
        // Strictly increasing, defensively. A boundary that did not advance
        // would produce an empty span and a block nobody can edit.
        if starts.last().is_none_or(|(previous, _)| at > *previous) {
            starts.push((at, kind));
        }
    };

    for (event, range) in Parser::new_ext(body, options()).into_offset_iter() {
        match event {
            Event::Start(tag) => {
                let frame = match (stack.last().copied(), &tag) {
                    // A list is not itself a block. Its items are.
                    (None | Some(Frame::Item { .. }), Tag::List(_)) => Frame::List,

                    // Every other top-level construct is one block, whole.
                    (None, _) => {
                        emit(range.start, kind_of(&tag), &mut starts);
                        Frame::Opaque
                    }

                    // "A list item is a block while the list holds together
                    // around it." At any depth: a nested list splits into its
                    // own items too, because a sectioned shopping list is still
                    // a shopping list and two people adding to different
                    // sections have to merge.
                    (Some(Frame::List), Tag::Item) => {
                        emit(range.start, BlockKind::ListItem, &mut starts);
                        Frame::Item {
                            after_sublist: false,
                        }
                    }
                    (Some(Frame::List), _) => Frame::Opaque,

                    // Content of an item before its first nested list belongs
                    // to the item's own block. Content resuming after one
                    // starts a further block, because it has no other block it
                    // could belong to without reaching backwards past entries
                    // that are not its own.
                    (Some(Frame::Item { after_sublist }), _) => {
                        if after_sublist {
                            emit(range.start, BlockKind::ListItemTail, &mut starts);
                        }
                        Frame::Opaque
                    }

                    (Some(Frame::Opaque), _) => Frame::Opaque,
                };
                stack.push(frame);
            }
            Event::End(end) => {
                let popped = stack.pop();
                debug_assert!(popped.is_some(), "unbalanced end for {end:?}");
                if popped == Some(Frame::List)
                    && let Some(Frame::Item { after_sublist }) = stack.last_mut()
                {
                    *after_sublist = true;
                }
            }
            // A thematic break is an event rather than a tag, so it needs the
            // same treatment the tag arm gives every other top-level construct.
            Event::Rule => match stack.last().copied() {
                None => emit(range.start, BlockKind::ThematicBreak, &mut starts),
                Some(Frame::Item {
                    after_sublist: true,
                }) => emit(range.start, BlockKind::ListItemTail, &mut starts),
                _ => {}
            },
            _ => {}
        }
    }

    starts
}

fn kind_of(tag: &Tag<'_>) -> BlockKind {
    use pulldown_cmark::CodeBlockKind;
    match tag {
        Tag::Paragraph => BlockKind::Paragraph,
        Tag::Heading { .. } => BlockKind::Heading,
        Tag::BlockQuote(_) => BlockKind::BlockQuote,
        Tag::CodeBlock(CodeBlockKind::Fenced(_)) => BlockKind::FencedCode,
        Tag::CodeBlock(CodeBlockKind::Indented) => BlockKind::IndentedCode,
        Tag::HtmlBlock => BlockKind::HtmlBlock,
        Tag::Table(_) => BlockKind::Table,
        Tag::FootnoteDefinition(_) => BlockKind::FootnoteDefinition,
        Tag::MetadataBlock(_) => BlockKind::MetadataBlock,
        Tag::DefinitionList => BlockKind::DefinitionList,
        Tag::Item => BlockKind::ListItem,
        // A list is never emitted as a block, and every remaining tag is
        // inline and cannot reach here without an enclosing block frame.
        _ => BlockKind::Paragraph,
    }
}

/// The start of the line containing `offset`, provided everything between is
/// blank.
///
/// The parser reports an indented code block from the first non-space byte and
/// a nested list item from its marker, both of which drop the indentation that
/// says what the construct is. Backing up to the line start restores it, which
/// is what keeps division lossless.
fn line_start(body: &str, offset: usize) -> usize {
    let before = &body[..offset];
    let line = before.rfind('\n').map(|at| at + 1).unwrap_or(0);
    if before[line..].chars().all(|c| c == ' ' || c == '\t') {
        line
    } else {
        offset
    }
}

/// Trim blank space from both ends of a span, keeping the indentation of the
/// first line that has content.
fn trim_span(body: &str, span: Range<usize>) -> Range<usize> {
    let slice = &body[span.clone()];
    let Some(first) = slice.find(|c: char| !c.is_whitespace()) else {
        return span.start..span.start;
    };
    let start = span.start + line_start(slice, first);
    let end = span.start + slice.trim_end().len();
    start..end.max(start)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_start_backs_up_over_blank_prefix_only() {
        // The marker of an indented list item backs up over its indentation.
        assert_eq!(line_start("  - a", 2), 0);
        // Anything else on the line before it does not.
        assert_eq!(line_start("x - a", 2), 2);
        assert_eq!(line_start("p\n  - a", 4), 2);
    }
}

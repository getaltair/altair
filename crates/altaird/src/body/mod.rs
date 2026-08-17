//! Bodies: how text divides into blocks, and how block identity survives an edit.
//!
//! A body is a single markdown field. The substrate spec makes the *block*, not
//! the field, the smallest independently addressable part of a body, because
//! "treating the field as the unit would make two people working on different
//! paragraphs of a shared plan look like a disagreement".
//!
//! Two things live here and nothing else:
//!
//! - [`divide`] — a pure function from text to boundaries. "The division is
//!   structural, and derived from the text alone." It reads no clock, no
//!   locale, no environment, and no randomness.
//! - [`reconcile`] — the matching step that carries identity forward. "A block
//!   is recognisable as the same block after the text around it changes, and
//!   after edits to its own text."
//!
//! DR-004 puts both at the instance: a client submits the whole body and the
//! instance divides it, so the division rule has exactly one implementation and
//! devices cannot disagree about the units reconciliation is decided in.

pub mod divide;
pub mod identity;

pub use divide::{BlockKind, Division, content_of, divide};
pub use identity::{Origin, Reconciliation, ResolvedBlock, StoredBlock, reconcile, reconcile_with};

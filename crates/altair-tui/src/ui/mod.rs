//! What the person looks at.
//!
//! One house style in two modes over one set of markup: [`theme`] holds the
//! tokens that differ, [`glyphs`] holds the characters and the width hazard
//! they carry, and [`chrome`] holds the shell every screen sits in.

pub mod chrome;
pub mod glyphs;
pub mod help;
pub mod markdown;
pub mod screens;
pub mod theme;
pub mod view;

pub use chrome::{Frame, Modal, Shell};
pub use glyphs::{Glyphs, Set, State};
pub use theme::{Mode, Theme};

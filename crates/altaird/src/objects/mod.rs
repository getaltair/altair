//! File bodies, and nothing else.
//!
//! Four operations — put, get, delete, enumerate — with the filesystem behind
//! them (DR-003). The interface is the decision: it is small enough that
//! putting something else behind it later is a contained piece of work, and
//! that stays true only while nothing outside it touches the bytes.
//!
//! [`interface`] holds the boundary and names no filesystem type;
//! [`filesystem`] holds the one implementation and everything beneath the
//! boundary, none of which is a decision at this level. [`capacity`] answers
//! one more question — how much room is left — that is not one of the four
//! and is kept apart from them so DR-003's count stays exactly four.

mod capacity;
mod filesystem;
mod interface;

pub use capacity::StorageCapacity;
pub use filesystem::FilesystemObjectStore;
pub use interface::{
    Body, BodyId, BodyListing, BodyStream, BoxError, ByteSource, Error, ObjectStore, StoredBody,
};

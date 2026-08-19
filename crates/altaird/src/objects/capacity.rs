//! Storage headroom: a second question about the same filesystem, answered
//! apart from [`ObjectStore`](super::ObjectStore)'s four operations.
//!
//! "What is held" and "how much room is left for more of it" are different
//! questions. Folding the second into the four-operation interface would make
//! it a fifth, which DR-003 fixes at exactly four. So this is its own trait,
//! implemented only by [`FilesystemObjectStore`](super::FilesystemObjectStore)
//! — the one type under this module that already names the root path — and
//! nothing outside `objects` is handed a path to ask the question itself.

use super::Error;

/// Free space on whatever backs an object store.
#[async_trait::async_trait]
pub trait StorageCapacity: Send + Sync {
    /// Bytes free for more bodies. [`Error::Unavailable`] when the filesystem
    /// could not be asked — the same condition
    /// [`ObjectStore`](super::ObjectStore)'s operations report it as.
    async fn bytes_free(&self) -> Result<u64, Error>;
}

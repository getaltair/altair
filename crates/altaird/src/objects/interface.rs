//! The four operations, and nothing else.
//!
//! This file is the boundary DR-003 calls "the decision". It names no
//! filesystem type on purpose: if a path could cross it, the four operations
//! would stop being the whole boundary and replacing what sits behind them
//! would stop being contained. A test in `tests/object_store_boundary.rs`
//! fails if the word for a filesystem path appears here at all.

use std::time::SystemTime;
use std::{fmt, pin::Pin};

use futures::Stream;
use uuid::Uuid;

/// Whatever went wrong upstream of the store while it was producing bytes —
/// a client that hung up mid-upload, most often. The store does not interpret
/// it; it only has to be able to say a `put` failed for a reason that was not
/// the store's.
pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

/// Bytes on their way in. A stream rather than a slice because `PutBody` is a
/// streaming RPC and a body has no size limit: taking `&[u8]` would put every
/// uploaded file in memory at the boundary, whatever sat behind it.
pub type ByteSource = Pin<Box<dyn Stream<Item = Result<Vec<u8>, BoxError>> + Send + 'static>>;

/// Bytes on their way out, in the same chunked shape `GetBody` returns them.
pub type BodyStream = Pin<Box<dyn Stream<Item = Result<Vec<u8>, Error>> + Send + 'static>>;

/// What the store holds, one item at a time. Borrowed from the store so an
/// implementation may hold a connection open across the walk.
pub type BodyListing<'a> = Pin<Box<dyn Stream<Item = Result<StoredBody, Error>> + Send + 'a>>;

/// The identity of one body.
///
/// A random UUIDv4 assigned by whatever brought the body into existence, and
/// read by nothing (DR-007). It is deliberately not derived from the content:
/// content addressing would make the identity mean something, and something
/// would eventually parse it back out.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct BodyId(Uuid);

impl BodyId {
    /// A fresh identity. Callers that already hold one — the client assigns it
    /// before the upload starts — use [`BodyId::from_uuid`] or [`TryFrom`].
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    #[must_use]
    pub fn from_uuid(id: Uuid) -> Self {
        Self(id)
    }

    /// The sixteen bytes the wire carries in `BodyChunk.body_id`.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8; 16] {
        self.0.as_bytes()
    }

    pub(crate) fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for BodyId {
    fn default() -> Self {
        Self::new()
    }
}

/// Sixteen bytes off the wire. The only failure is a wrong length, which is a
/// malformed request rather than anything the store has an opinion about.
impl TryFrom<&[u8]> for BodyId {
    type Error = uuid::Error;

    fn try_from(bytes: &[u8]) -> Result<Self, uuid::Error> {
        Uuid::from_slice(bytes).map(Self)
    }
}

impl fmt::Debug for BodyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BodyId({})", self.0.simple())
    }
}

/// One body, as [`ObjectStore::enumerate`] sees it: identity, size, and when
/// the bytes landed.
///
/// The timestamp is here because reclamation needs it. Bytes are written
/// before the record that points at them, so at any moment some unreferenced
/// bytes belong to a record that is about to be committed. Without an age, a
/// sweep cannot tell those from genuine orphans and would delete a body out
/// from under a capture in flight. What the grace period should be is Wave
/// 2.4's to decide; being able to have one is this interface's obligation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StoredBody {
    pub id: BodyId,
    pub len: u64,
    pub written_at: SystemTime,
}

/// One body, on its way to a reader.
///
/// The length is separated from the bytes so a caller can say how big the
/// answer is before it has read any of it, and never has to buffer the body to
/// find out.
pub struct Body {
    pub len: u64,
    chunks: BodyStream,
}

impl Body {
    #[must_use]
    pub fn new(len: u64, chunks: BodyStream) -> Self {
        Self { len, chunks }
    }

    #[must_use]
    pub fn into_chunks(self) -> BodyStream {
        self.chunks
    }
}

impl fmt::Debug for Body {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Body").field("len", &self.len).finish()
    }
}

/// What can go wrong.
///
/// The two absent cases are separate on purpose, because the component model
/// makes them different statements to a person: a body that is not there is
/// missing, and a store that cannot be reached means the entity, its title,
/// its relations and its derived text are all still available and only the
/// body is *currently unavailable*.
///
/// It also matters to reclamation, which must never read "the store is gone"
/// as "the bytes are already deleted".
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// No body has that identity. The store answered.
    #[error("no body with that identity")]
    NoSuchBody,

    /// The store could not answer. Says nothing about whether the body exists.
    #[error("the object store is currently unavailable")]
    Unavailable(#[source] std::io::Error),

    /// The bytes being uploaded stopped arriving. Nothing was stored.
    #[error("the byte source failed before the body was complete")]
    Source(#[source] BoxError),
}

impl Error {
    /// True when the store answered and the answer was "not here".
    #[must_use]
    pub fn is_no_such_body(&self) -> bool {
        matches!(self, Self::NoSuchBody)
    }

    /// True when the store could not answer, which is never evidence about
    /// what it holds.
    #[must_use]
    pub fn is_currently_unavailable(&self) -> bool {
        matches!(self, Self::Unavailable(_))
    }
}

/// Write bytes, read a body, remove bytes, and enumerate what is held.
///
/// Deliberately absent, because nothing in the requirements asks for it:
/// content addressing, multipart upload, lifecycle policy, replication,
/// listing by prefix, and any notion of a container or bucket.
///
/// **This cannot participate in a transaction with the structured store**, and
/// the interface offers nothing that pretends otherwise. That is why the
/// ordering rules exist and why they are the caller's to keep: bytes before
/// the record on creation, the record before the bytes on erasure. Every
/// operation here is one step, so a caller can order the two sides itself.
#[async_trait::async_trait]
pub trait ObjectStore: Send + Sync + 'static {
    /// Store `source` under `id`, and answer with how many bytes were stored.
    ///
    /// **Idempotent**, as the wire contract promises: re-uploading the same
    /// identity after a broken connection is ordinary, and repeats the write
    /// rather than refusing it, so an attempt that was cut short is repaired
    /// rather than left half-written. Content is the caller's promise; nothing
    /// here compares bytes.
    ///
    /// **A body becomes visible whole or not at all.** No reader ever observes
    /// a partial one, and a `put` that fails part-way stores nothing.
    ///
    /// Returning means the bytes are durable, which is what makes "bytes
    /// before the record" worth anything.
    async fn put(&self, id: BodyId, source: ByteSource) -> Result<u64, Error>;

    /// Read a body back whole.
    ///
    /// [`Error::NoSuchBody`] when the store answered and holds no such body;
    /// [`Error::Unavailable`] when it could not answer.
    async fn get(&self, id: BodyId) -> Result<Body, Error>;

    /// Remove the bytes.
    ///
    /// **Idempotent, and removing something absent is success**, because
    /// reclamation sweeps repeatedly and erasure may have already removed the
    /// same bytes. It is not success when the store could not be reached —
    /// that is [`Error::Unavailable`], so a sweep never records bytes as gone
    /// on the strength of a store it could not talk to.
    async fn delete(&self, id: BodyId) -> Result<(), Error>;

    /// Every body held, one at a time.
    ///
    /// **Load-bearing rather than housekeeping.** Erasure removes the record
    /// before the bytes, so this sweep is what closes the erasure window;
    /// reclamation joins what this yields against what the structured store
    /// references.
    ///
    /// It streams, and never reads a body, so the cost is one directory walk
    /// and one stat per body no matter how large the bodies are — which is
    /// what "cheap enough to run on a schedule" requires. Order is not
    /// specified, and a caller that depends on one is wrong.
    fn enumerate(&self) -> BodyListing<'_>;
}

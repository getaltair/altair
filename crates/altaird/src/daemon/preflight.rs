//! What must already be true before the daemon serves anything.
//!
//! **Refuse to start rather than start degraded.** Every check here answers a
//! question that would otherwise be answered by a person's capture failing,
//! hours later, with nobody watching. A precondition found at startup costs an
//! operator one clear line; the same precondition found at the first request
//! costs a person their note.
//!
//! Three checks, and one deliberate absence.
//!
//! * **The structured store** is reached and migrated by
//!   [`crate::store::connect`], which fails if any of that fails.
//! * **The extensions** the schema depends on are present —
//!   [`crate::store::preflight`] says why the migration having run is not
//!   evidence of that.
//! * **The object store is writable**, checked by [`object_store`] below.
//! * **Not the identity provider.** Nothing in startup contacts it. A provider
//!   that is briefly away is a wait that clears by the ordinary path
//!   continuing to run (`auth::jwks`), and a household restarting its instance
//!   and its provider together is the ordinary case. An instance that refused
//!   to start until Authentik answered would convert a wait into an outage,
//!   and would do it at exactly the moment recovery was already underway.

use crate::objects::{BodyId, ObjectStore};

/// Prove the object store can take bytes, give them back, and let go of them.
///
/// **Through the four operations and nothing else.** DR-003 makes them the
/// whole boundary; a writability check that stat'ed a directory or opened a
/// file for append would be reaching around the interface to ask a question
/// the interface can already answer, and it would answer a weaker one — a
/// directory can be writable while the disk is full, mounted read-only
/// underneath, or backed by something that accepts a write and loses it.
///
/// A round trip is the real question: put, get back, compare, delete. The
/// probe carries a fresh identity, so nothing it does can collide with a body
/// a person owns, and it is removed before the daemon serves. If the process
/// dies between the put and the delete it leaves one small unreferenced body,
/// which is precisely what reclamation exists to sweep.
///
/// # Errors
///
/// If the store cannot store, cannot produce what it stored, or produces
/// something else.
pub async fn object_store(store: &dyn ObjectStore) -> anyhow::Result<()> {
    /// Small, and not empty: a store that silently discards writes would pass
    /// a zero-length round trip.
    const PROBE: &[u8] = b"altair startup probe";

    let id = BodyId::new();
    let source = Box::pin(futures::stream::once(async { Ok(PROBE.to_vec()) }));

    let written = store
        .put(id, source)
        .await
        .map_err(|e| anyhow::anyhow!("the object store could not be written to: {e}"))?;
    if written != PROBE.len() as u64 {
        anyhow::bail!(
            "the object store accepted {written} bytes of a {}-byte probe",
            PROBE.len()
        );
    }

    let read = read_back(store, id).await;

    // Before the read is checked. The bytes go whatever the answer was, so a
    // store that can write and not read does not also leak the probe.
    store
        .delete(id)
        .await
        .map_err(|e| anyhow::anyhow!("the object store could not remove a body: {e}"))?;

    let read = read?;
    if read != PROBE {
        anyhow::bail!(
            "the object store gave back {} bytes, not the probe",
            read.len()
        );
    }
    Ok(())
}

async fn read_back(store: &dyn ObjectStore, id: BodyId) -> anyhow::Result<Vec<u8>> {
    use futures::StreamExt;

    let body = store
        .get(id)
        .await
        .map_err(|e| anyhow::anyhow!("the object store could not be read back: {e}"))?;

    let mut bytes = Vec::new();
    let mut chunks = body.into_chunks();
    while let Some(chunk) = chunks.next().await {
        bytes.extend_from_slice(
            &chunk.map_err(|e| anyhow::anyhow!("the object store stopped mid-body: {e}"))?,
        );
    }
    Ok(bytes)
}

//! The four operations, exercised against the filesystem implementation.
//!
//! These tests are written against [`ObjectStore`] and never against the
//! filesystem behind it, except where they deliberately break the store to
//! check what it says when it cannot answer, and where they assert that a
//! failed upload left nothing behind. Both need to see the root directory the
//! test itself created, and both are named in the allow list in
//! `object_store_boundary.rs`.

use std::time::{Duration, SystemTime};

use altaird::objects::{
    Body, BodyId, ByteSource, Error, FilesystemObjectStore, ObjectStore, StoredBody,
};
use futures::{StreamExt, stream};
use tempfile::TempDir;

async fn store() -> (TempDir, FilesystemObjectStore) {
    let root = TempDir::new().expect("temp root");
    let store = FilesystemObjectStore::open(root.path())
        .await
        .expect("open the store");
    (root, store)
}

fn source(chunks: Vec<Vec<u8>>) -> ByteSource {
    Box::pin(stream::iter(chunks.into_iter().map(Ok)))
}

/// A source that produces some bytes and then stops, the way a client that
/// hangs up mid-upload does.
fn failing_source(prefix: Vec<u8>) -> ByteSource {
    Box::pin(stream::iter(vec![
        Ok(prefix),
        Err("the client hung up".into()),
    ]))
}

async fn drain(body: Body) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut chunks = body.into_chunks();
    while let Some(chunk) = chunks.next().await {
        bytes.extend_from_slice(&chunk.expect("read a chunk"));
    }
    bytes
}

async fn listing(store: &FilesystemObjectStore) -> Vec<StoredBody> {
    store
        .enumerate()
        .map(|entry| entry.expect("enumerate"))
        .collect()
        .await
}

/// Two hundred kilobytes, so both the write and the read cross the chunk size
/// several times and the test is not silently about a single buffer.
fn large_body() -> Vec<u8> {
    (0..200 * 1024).map(|i| (i % 251) as u8).collect()
}

#[tokio::test]
async fn put_then_get_returns_the_same_bytes() {
    let (_root, store) = store().await;
    let id = BodyId::new();
    let bytes = large_body();

    let written = store
        .put(id, source(vec![bytes.clone()]))
        .await
        .expect("put");
    assert_eq!(written, bytes.len() as u64);

    let body = store.get(id).await.expect("get");
    assert_eq!(body.len, bytes.len() as u64);
    assert_eq!(drain(body).await, bytes);
}

#[tokio::test]
async fn a_body_arriving_in_pieces_is_stored_whole() {
    let (_root, store) = store().await;
    let id = BodyId::new();
    let pieces: Vec<Vec<u8>> = (0..8u8).map(|n| vec![n; 40 * 1024]).collect();
    let whole: Vec<u8> = pieces.concat();

    let written = store.put(id, source(pieces)).await.expect("put");
    assert_eq!(written, whole.len() as u64);
    assert_eq!(drain(store.get(id).await.expect("get")).await, whole);
}

#[tokio::test]
async fn an_empty_body_round_trips() {
    let (_root, store) = store().await;
    let id = BodyId::new();

    assert_eq!(store.put(id, source(vec![])).await.expect("put"), 0);
    let body = store.get(id).await.expect("get");
    assert_eq!(body.len, 0);
    assert!(drain(body).await.is_empty());
}

#[tokio::test]
async fn re_uploading_the_same_identity_is_ordinary_and_repairs_the_first_attempt() {
    let (_root, store) = store().await;
    let id = BodyId::new();

    // A first attempt that was cut short leaves nothing.
    let failed = store.put(id, failing_source(vec![9; 4096])).await;
    assert!(matches!(failed, Err(Error::Source(_))), "{failed:?}");
    assert!(store.get(id).await.is_err());

    // The retry is not refused, and the body is whole afterwards.
    let bytes = vec![7u8; 100 * 1024];
    store
        .put(id, source(vec![bytes.clone()]))
        .await
        .expect("retry");
    assert_eq!(drain(store.get(id).await.expect("get")).await, bytes);

    // And a second complete upload of the same identity is accepted too.
    store
        .put(id, source(vec![bytes.clone()]))
        .await
        .expect("re-upload");
    assert_eq!(drain(store.get(id).await.expect("get")).await, bytes);
    assert_eq!(listing(&store).await.len(), 1, "one identity, one body");
}

#[tokio::test]
async fn an_upload_that_fails_part_way_stores_nothing_and_leaves_nothing() {
    let (root, store) = store().await;
    let id = BodyId::new();

    let failed = store.put(id, failing_source(vec![1; 64 * 1024])).await;
    assert!(matches!(failed, Err(Error::Source(_))), "{failed:?}");

    assert!(
        store.get(id).await.unwrap_err().is_no_such_body(),
        "a partial body must never become visible"
    );
    assert!(listing(&store).await.is_empty());

    let staged: Vec<_> = std::fs::read_dir(root.path().join("staging"))
        .expect("staging")
        .map(|entry| entry.expect("entry").file_name())
        .collect();
    assert!(staged.is_empty(), "incomplete upload left {staged:?}");
}

#[tokio::test]
async fn getting_a_body_that_was_never_put_says_no_such_body() {
    let (_root, store) = store().await;
    let error = store.get(BodyId::new()).await.unwrap_err();
    assert!(error.is_no_such_body(), "{error:?}");
    assert!(!error.is_currently_unavailable());
}

#[tokio::test]
async fn delete_removes_the_bytes_and_repeats_without_complaint() {
    let (_root, store) = store().await;
    let id = BodyId::new();
    store
        .put(id, source(vec![vec![3; 1024]]))
        .await
        .expect("put");

    store.delete(id).await.expect("first delete");
    assert!(store.get(id).await.unwrap_err().is_no_such_body());

    // Reclamation sweeps repeatedly and erasure may already have run.
    store.delete(id).await.expect("second delete");
    store
        .delete(BodyId::new())
        .await
        .expect("delete of something never stored");
}

#[tokio::test]
async fn enumerate_yields_every_body_with_its_size_and_age() {
    let (_root, store) = store().await;
    let before = SystemTime::now() - Duration::from_secs(1);

    let mut expected: Vec<(BodyId, u64)> = Vec::new();
    for n in 0..12u8 {
        let id = BodyId::new();
        let len = u64::from(n) * 1000;
        store
            .put(id, source(vec![vec![n; len as usize]]))
            .await
            .expect("put");
        expected.push((id, len));
    }

    let mut held: Vec<(BodyId, u64)> = listing(&store)
        .await
        .into_iter()
        .inspect(|body| {
            // Reclamation needs an age to tell a genuine orphan from bytes
            // whose record is about to be committed.
            assert!(body.written_at >= before, "{:?}", body.written_at);
            assert!(
                body.written_at <= SystemTime::now() + Duration::from_secs(1),
                "{:?}",
                body.written_at
            );
        })
        .map(|body| (body.id, body.len))
        .collect();

    held.sort_by_key(|(id, _)| *id.as_bytes());
    expected.sort_by_key(|(id, _)| *id.as_bytes());
    assert_eq!(held, expected);
}

#[tokio::test]
async fn enumerate_stops_seeing_what_was_deleted() {
    let (_root, store) = store().await;
    let kept = BodyId::new();
    let swept = BodyId::new();
    store
        .put(kept, source(vec![vec![1; 10]]))
        .await
        .expect("put");
    store
        .put(swept, source(vec![vec![2; 20]]))
        .await
        .expect("put");

    store.delete(swept).await.expect("delete");

    let held = listing(&store).await;
    assert_eq!(held.len(), 1);
    assert_eq!(held[0].id, kept);
}

#[tokio::test]
async fn enumerate_is_lazy_and_never_reads_a_body() {
    let (_root, store) = store().await;
    let bytes = large_body();
    for _ in 0..20 {
        store
            .put(BodyId::new(), source(vec![bytes.clone()]))
            .await
            .expect("put");
    }

    // Taking one item does not walk, stat, or read the rest. The size is
    // reported without the bytes ever being opened, which is what keeps the
    // sweep cheap enough to run on a schedule.
    let first: Vec<_> = store.enumerate().take(1).collect().await;
    assert_eq!(first.len(), 1);
    assert_eq!(
        first[0].as_ref().expect("entry").len,
        bytes.len() as u64,
        "size comes from the walk, not from reading"
    );
}

#[tokio::test]
async fn an_empty_store_enumerates_to_nothing() {
    let (_root, store) = store().await;
    assert!(listing(&store).await.is_empty());
}

#[tokio::test]
async fn a_store_that_cannot_be_reached_says_so_rather_than_saying_the_body_is_missing() {
    let (root, store) = store().await;
    let id = BodyId::new();
    store
        .put(id, source(vec![vec![5; 4096]]))
        .await
        .expect("put");

    std::fs::remove_dir_all(root.path()).expect("take the store away");

    let error = store.get(id).await.unwrap_err();
    assert!(
        error.is_currently_unavailable(),
        "the body is currently unavailable, which is not the same as missing: {error:?}"
    );
    assert!(!error.is_no_such_body());
}

#[tokio::test]
async fn a_store_that_cannot_be_reached_never_reports_bytes_as_already_gone() {
    let (root, store) = store().await;
    let id = BodyId::new();
    store
        .put(id, source(vec![vec![5; 4096]]))
        .await
        .expect("put");

    std::fs::remove_dir_all(root.path()).expect("take the store away");

    // Success here would let a sweep record an erasure that never happened.
    let error = store.delete(id).await.unwrap_err();
    assert!(error.is_currently_unavailable(), "{error:?}");

    // And the sweep must not read an unreachable store as an empty one, which
    // would make every body look unreferenced.
    let listed: Vec<_> = store.enumerate().collect().await;
    assert_eq!(listed.len(), 1);
    assert!(
        listed[0].as_ref().unwrap_err().is_currently_unavailable(),
        "{:?}",
        listed[0]
    );
}

#[tokio::test]
async fn a_put_that_cannot_reach_the_store_stores_nothing_and_says_it_is_unavailable() {
    let (root, store) = store().await;
    std::fs::remove_dir_all(root.path()).expect("take the store away");

    let error = store
        .put(BodyId::new(), source(vec![vec![1; 16]]))
        .await
        .unwrap_err();
    assert!(error.is_currently_unavailable(), "{error:?}");
}

#[tokio::test]
async fn many_bodies_land_at_once() {
    let (_root, store) = store().await;
    let store = std::sync::Arc::new(store);

    let mut tasks = Vec::new();
    for n in 0..32u8 {
        let store = store.clone();
        tasks.push(tokio::spawn(async move {
            let id = BodyId::new();
            store
                .put(id, source(vec![vec![n; 1024 * (usize::from(n) + 1)]]))
                .await
                .expect("put");
            id
        }));
    }

    let mut ids = Vec::new();
    for task in tasks {
        ids.push(task.await.expect("task"));
    }

    let held = listing(&store).await;
    assert_eq!(held.len(), ids.len());
    for id in ids {
        assert!(held.iter().any(|body| body.id == id));
    }
}

/// The interface offers no way to make a body and a record atomic, and that is
/// the point: a caller orders the two sides itself. This is the creation half
/// of the rule, written as a caller would write it.
#[tokio::test]
async fn bytes_can_be_made_durable_before_any_record_points_at_them() {
    let (_root, store) = store().await;
    let id = BodyId::new();
    let bytes = vec![42u8; 8192];

    // Step one: the bytes. When this returns they are durable and readable,
    // and nothing anywhere refers to them yet — which is exactly the orphan
    // the sweep is allowed to collect if step two never happens.
    store
        .put(id, source(vec![bytes.clone()]))
        .await
        .expect("put");
    assert_eq!(drain(store.get(id).await.expect("get")).await, bytes);

    let orphans = listing(&store).await;
    assert_eq!(orphans.len(), 1);
    assert_eq!(orphans[0].id, id);

    // Step two, the record, belongs to the write path and is not this lane's.
    // The erasure half inverts the order, and `delete` tolerating an absent
    // body is what lets it be retried until it takes.
    store.delete(id).await.expect("erase the bytes");
    store.delete(id).await.expect("and again, after a crash");
    assert!(listing(&store).await.is_empty());
}

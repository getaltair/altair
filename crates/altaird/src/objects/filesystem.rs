//! The filesystem, behind the four operations (DR-003).
//!
//! Everything in this file is *beneath* the interface and therefore not a
//! decision at this level: the directory fan-out, the staging area, the chunk
//! size, and the use of rename to publish a body are all replaceable without
//! any caller noticing. What callers may rely on is only what
//! [`super::ObjectStore`] promises.
//!
//! The one path that crosses this boundary is the root directory, named once
//! when the store is opened. Nothing hands a path back out.

use std::io;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use futures::StreamExt;
use futures::stream;
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use uuid::Uuid;

use super::{
    Body, BodyId, BodyListing, BodyStream, ByteSource, Error, ObjectStore, StorageCapacity,
    StoredBody,
};

/// Read and written in 64 KiB pieces. Large enough that syscall overhead
/// disappears against the copy, small enough that a body of any size costs
/// bounded memory at the boundary and fits inside a gRPC message with room to
/// spare.
const CHUNK: usize = 64 * 1024;

/// Bodies are published under two levels of 256, taken from the first four hex
/// digits of the identity. Not meaning — the identity stays opaque — only a
/// way to keep any one directory small enough that the enumerate walk stays
/// cheap on a household's worth of files.
const FAN_OUT: &str = "bodies";

/// Incomplete uploads live here and are renamed into place when whole, so a
/// reader never sees a partial body and a crash never leaves one visible.
const STAGING: &str = "staging";

pub struct FilesystemObjectStore {
    root: PathBuf,
}

impl FilesystemObjectStore {
    /// Open, creating the layout if it is not there.
    ///
    /// This is the only place a filesystem path enters, and it does not leave
    /// again.
    pub async fn open(root: impl AsRef<Path> + Send) -> Result<Self, Error> {
        let root = root.as_ref().to_path_buf();
        fs::create_dir_all(root.join(FAN_OUT))
            .await
            .map_err(Error::Unavailable)?;
        fs::create_dir_all(root.join(STAGING))
            .await
            .map_err(Error::Unavailable)?;
        Ok(Self { root })
    }

    fn bodies(&self) -> PathBuf {
        self.root.join(FAN_OUT)
    }

    fn body_path(&self, id: BodyId) -> PathBuf {
        let name = id.as_uuid().simple().to_string();
        self.bodies()
            .join(&name[0..2])
            .join(&name[2..4])
            .join(&name)
    }

    /// Whether the store itself is reachable, asked only when an operation has
    /// already come back empty-handed. A missing body under a present store is
    /// [`Error::NoSuchBody`]; a missing store is [`Error::Unavailable`], and
    /// conflating the two would let reclamation read "the disk is not mounted"
    /// as "those bytes are already gone".
    async fn absent_or_unavailable(&self, cause: io::Error) -> Error {
        match fs::metadata(self.bodies()).await {
            Ok(meta) if meta.is_dir() => Error::NoSuchBody,
            _ => Error::Unavailable(cause),
        }
    }
}

/// Removes its file unless it was published. Covers every way out of `put` —
/// a source that stopped, a write that failed, a future that was dropped —
/// so a failed upload leaves nothing behind.
struct Staged {
    path: PathBuf,
    published: bool,
}

impl Drop for Staged {
    fn drop(&mut self) {
        if !self.published {
            // Blocking, but it is one unlink on a failure path.
            let _ = std::fs::remove_file(&self.path);
        }
    }
}

/// `fsync` on the directory, so a rename survives a crash rather than only the
/// bytes it published. Without this, "the bytes are durable before the record
/// is committed" is true of the content and not of the name that finds it.
#[cfg(unix)]
async fn sync_dir(dir: &Path) -> io::Result<()> {
    fs::File::open(dir).await?.sync_all().await
}

#[cfg(not(unix))]
async fn sync_dir(_dir: &Path) -> io::Result<()> {
    // Opening a directory as a file is not portable. Platforms without it get
    // the file's own durability and not the rename's.
    Ok(())
}

#[async_trait::async_trait]
impl ObjectStore for FilesystemObjectStore {
    async fn put(&self, id: BodyId, mut source: ByteSource) -> Result<u64, Error> {
        let mut staged = Staged {
            path: self
                .root
                .join(STAGING)
                .join(format!("{}.part", Uuid::new_v4().simple())),
            published: false,
        };

        let mut file = fs::File::create(&staged.path)
            .await
            .map_err(Error::Unavailable)?;

        let mut written: u64 = 0;
        while let Some(chunk) = source.next().await {
            let chunk = chunk.map_err(Error::Source)?;
            file.write_all(&chunk).await.map_err(Error::Unavailable)?;
            written += chunk.len() as u64;
        }

        // Durable before it is visible, and visible only whole. Callers order
        // the record after this returns; that ordering is worth nothing if the
        // bytes are still in a page cache when the record commits.
        file.sync_all().await.map_err(Error::Unavailable)?;
        drop(file);

        let destination = self.body_path(id);
        let parent = destination.parent().expect("a body path has a parent");
        fs::create_dir_all(parent)
            .await
            .map_err(Error::Unavailable)?;

        // Rename is atomic and replaces silently, which is exactly what
        // idempotent re-upload wants: a second attempt at the same identity
        // repairs whatever the first one left, and no reader observes the
        // moment in between.
        fs::rename(&staged.path, &destination)
            .await
            .map_err(Error::Unavailable)?;
        staged.published = true;

        sync_dir(parent).await.map_err(Error::Unavailable)?;
        Ok(written)
    }

    async fn get(&self, id: BodyId) -> Result<Body, Error> {
        let path = self.body_path(id);

        let meta = match fs::metadata(&path).await {
            Ok(meta) => meta,
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                return Err(self.absent_or_unavailable(e).await);
            }
            Err(e) => return Err(Error::Unavailable(e)),
        };

        let file = match fs::File::open(&path).await {
            Ok(file) => file,
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                return Err(self.absent_or_unavailable(e).await);
            }
            Err(e) => return Err(Error::Unavailable(e)),
        };

        // Read lazily. A caller relaying a body to a client never holds more
        // than one chunk of it.
        let chunks: BodyStream = Box::pin(stream::unfold(Some(file), |state| async move {
            let mut file = state?;
            let mut buf = vec![0u8; CHUNK];
            match file.read(&mut buf).await {
                Ok(0) => None,
                Ok(n) => {
                    buf.truncate(n);
                    Some((Ok(buf), Some(file)))
                }
                Err(e) => Some((Err(Error::Unavailable(e)), None)),
            }
        }));

        Ok(Body::new(meta.len(), chunks))
    }

    async fn delete(&self, id: BodyId) -> Result<(), Error> {
        match fs::remove_file(self.body_path(id)).await {
            Ok(()) => Ok(()),
            // Already gone is the ordinary case: erasure and the sweep both
            // remove the same bytes, and they race by design.
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                match self.absent_or_unavailable(e).await {
                    Error::NoSuchBody => Ok(()),
                    other => Err(other),
                }
            }
            Err(e) => Err(Error::Unavailable(e)),
        }
    }

    fn enumerate(&self) -> BodyListing<'_> {
        Box::pin(stream::unfold(
            Walk::Start(self.bodies()),
            |state| async move {
                let mut stack = match state {
                    Walk::Start(root) => match fs::read_dir(&root).await {
                        Ok(dir) => vec![dir],
                        // The store is unreachable. Say so once, then end: a sweep
                        // that saw this must not conclude anything about what is
                        // held.
                        Err(e) => {
                            return Some((Err(Error::Unavailable(e)), Walk::Walking(Vec::new())));
                        }
                    },
                    Walk::Walking(stack) => stack,
                };

                loop {
                    let dir = stack.last_mut()?;
                    let entry = match dir.next_entry().await {
                        Ok(Some(entry)) => entry,
                        Ok(None) => {
                            stack.pop();
                            continue;
                        }
                        Err(e) => {
                            stack.pop();
                            return Some((Err(Error::Unavailable(e)), Walk::Walking(stack)));
                        }
                    };

                    let file_type = match entry.file_type().await {
                        Ok(file_type) => file_type,
                        Err(e) => return Some((Err(Error::Unavailable(e)), Walk::Walking(stack))),
                    };

                    if file_type.is_dir() {
                        match fs::read_dir(entry.path()).await {
                            Ok(dir) => stack.push(dir),
                            Err(e) => {
                                return Some((Err(Error::Unavailable(e)), Walk::Walking(stack)));
                            }
                        }
                        continue;
                    }

                    // Nothing but published bodies should be here; anything else
                    // is not the store's to report on.
                    let Some(id) = entry
                        .file_name()
                        .to_str()
                        .and_then(|name| Uuid::try_parse(name).ok())
                        .map(BodyId::from_uuid)
                    else {
                        continue;
                    };

                    let meta = match entry.metadata().await {
                        Ok(meta) => meta,
                        // It was published a moment ago and erased before the stat.
                        // Nothing is owed about a body that is no longer there.
                        Err(e) if e.kind() == io::ErrorKind::NotFound => continue,
                        Err(e) => return Some((Err(Error::Unavailable(e)), Walk::Walking(stack))),
                    };

                    let body = StoredBody {
                        id,
                        len: meta.len(),
                        written_at: meta.modified().unwrap_or(SystemTime::UNIX_EPOCH),
                    };
                    return Some((Ok(body), Walk::Walking(stack)));
                }
            },
        ))
    }
}

/// The walk holds only the open directories on its current path, so
/// enumerating costs memory in the depth of the layout rather than in the
/// number of bodies.
enum Walk {
    Start(PathBuf),
    Walking(Vec<fs::ReadDir>),
}

#[async_trait::async_trait]
impl StorageCapacity for FilesystemObjectStore {
    /// `statvfs` is a syscall, run on a blocking thread for the same reason
    /// every other operation here keeps the executor off blocking I/O — even
    /// though this one is a single stat rather than a stream of them.
    async fn bytes_free(&self) -> Result<u64, Error> {
        let root = self.root.clone();
        tokio::task::spawn_blocking(move || {
            let stat = rustix::fs::statvfs(&root).map_err(|e| Error::Unavailable(e.into()))?;
            // Available to an unprivileged process, not merely unallocated —
            // many filesystems reserve a slice of free blocks for root, and
            // that slice is never real headroom for a household's files.
            Ok(stat.f_bavail * stat.f_frsize)
        })
        .await
        .map_err(|e| Error::Unavailable(io::Error::other(e)))?
    }
}

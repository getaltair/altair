//! The device store.
//!
//! One SQLite file in the durable directory, holding what the person captured
//! and what has not yet reached the instance. Both live here because they are
//! one question — "what do I show" — and two stores would put the answer in
//! neither.
//!
//! # What is held, and in what shape
//!
//! **Entities are held as encoded `EntityContent`, not as columns.** The
//! contract is explicitly designed to grow fields: numbers are permanent and
//! closing a recorded gap adds one. A schema mirroring the wire would migrate
//! every time that happened. Holding the message whole means the proto grows
//! without a client migration, and nothing about the storage forecloses export.
//! The columns beside it are the orders and the flags a caller navigates by,
//! and they are derived from the messages rather than authoritative over them.
//!
//! **Outbox items are held as encoded `Intent`, keyed by the identity they
//! were given when they were created.** That identity is never regenerated —
//! not on retry, not across a restart — which is the single most likely point
//! of divergence between two implementations of this.
//!
//! # Durability
//!
//! `synchronous = FULL` under a write-ahead log, so a commit has reached the
//! disk before it returns. Acceptance is shown only after one of these
//! commits: a kill immediately after the person is told leaves the capture
//! present, which is a thing the conformance suite kills the process to check.

use std::path::Path;

use altair_proto::v1;
use prost::Message;
use rusqlite::{Connection, OptionalExtension, Transaction, TransactionBehavior, params};

/// The file, inside whatever durable directory the client was given.
pub(crate) const FILE: &str = "altair-device.sqlite3";

/// One outbox item, ready to send.
#[derive(Debug, Clone)]
pub struct Pending {
    /// Position in the outbox. The order intents were accepted in.
    pub seq: i64,
    pub intent_id: Vec<u8>,
    /// The entity this intent is about. Ordering is per entity, so this is
    /// what the sender groups by.
    pub entity_id: Vec<u8>,
    pub intent: v1::Intent,
    /// The counter the instance last acknowledged for this entity. An edit is
    /// sent against this rather than against the counter it was composed
    /// against — see [`crate::sender`].
    pub acknowledged_counter: u64,
}

/// What the person sees when they return to something.
#[derive(Debug, Clone)]
pub struct Held {
    pub entity_id: Vec<u8>,
    pub content: v1::EntityContent,
    /// The bytes of a captured file, where this is one.
    pub bytes: Option<Vec<u8>>,
}

impl Held {
    /// The title, where the person gave one.
    #[must_use]
    pub fn title(&self) -> Option<&str> {
        self.content.title.as_deref()
    }

    /// The body of a note, where this is one.
    #[must_use]
    pub fn body(&self) -> Option<&str> {
        match self.content.specific.as_ref() {
            Some(v1::entity_content::Specific::Note(note)) => note.body.as_deref(),
            _ => None,
        }
    }
}

pub struct Store {
    connection: Connection,
}

impl Store {
    /// Open the store in `state_dir`, creating it if it is not there.
    ///
    /// **A failure here is the one local condition that reaches the guarantee
    /// rather than delaying it**, so it is returned rather than retried: the
    /// caller refuses the capture and says so at the moment of the attempt.
    ///
    /// # Errors
    ///
    /// Fails when the directory cannot be written to, which is what an
    /// unwritable or full device looks like from here.
    pub fn open(state_dir: &Path) -> rusqlite::Result<Self> {
        let connection = Connection::open(state_dir.join(FILE))?;
        // A write-ahead log so a read never waits behind the sender's write,
        // and FULL so a commit has reached the disk before it returns.
        connection.pragma_update(None, "journal_mode", "WAL")?;
        connection.pragma_update(None, "synchronous", "FULL")?;
        connection.busy_timeout(std::time::Duration::from_secs(5))?;
        // WAL mode is a property of the file rather than of the connection, but
        // the first statement is where an unwritable directory actually bites:
        // opening is lazy, and this is the call that has to create something.
        connection.execute_batch(SCHEMA)?;
        Ok(Self { connection })
    }

    /// Begin a write.
    ///
    /// **`IMMEDIATE`, and that word is load bearing.** A deferred transaction
    /// takes its write lock at the first statement that writes, and under a
    /// write-ahead log a transaction that has already read cannot then take
    /// one: if another connection wrote in between, the snapshot it read is
    /// stale and SQLite answers `SQLITE_BUSY` *without* honouring the busy
    /// timeout, because waiting could not help. That is not hypothetical here
    /// — the sender and the person's surface hold a connection each, and it
    /// turned up as a capture refused for "database is locked" while the
    /// sender happened to be recording an acknowledgement. Acceptance is not
    /// allowed to fail for that reason.
    ///
    /// Taking the lock up front makes the wait a wait, which the busy timeout
    /// then covers.
    fn write(&mut self) -> rusqlite::Result<Transaction<'_>> {
        self.connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
    }

    /// A second handle on the same file.
    ///
    /// The sender and the person's surface hold one each, so that a send in
    /// progress and a capture never contend for the same connection. Under a
    /// write-ahead log they contend for nothing a reader can feel.
    ///
    /// # Errors
    ///
    /// Fails for the same reasons [`Store::open`] does.
    pub fn reopen(state_dir: &Path) -> rusqlite::Result<Self> {
        Self::open(state_dir)
    }

    /// Accept a capture: the entity, its bytes where it has any, and the
    /// intent that will carry it, in one transaction.
    ///
    /// **Everything the person was promised is in this commit.** Returning
    /// from it is what makes showing acceptance honest, and a kill on the next
    /// instruction leaves all three present or none of them.
    ///
    /// # Errors
    ///
    /// Fails when the store cannot be written to.
    pub fn accept_creation(
        &mut self,
        entity_id: &[u8],
        content: &v1::EntityContent,
        body: Option<(&[u8], &[u8])>,
        intent: &v1::Intent,
    ) -> rusqlite::Result<()> {
        let transaction = self.write()?;
        if let Some((body_id, bytes)) = body {
            transaction.execute(
                "INSERT OR REPLACE INTO body (body_id, bytes, sent) VALUES (?1, ?2, 0)",
                params![body_id, bytes],
            )?;
        }
        transaction.execute(
            "INSERT INTO entity (entity_id, content, body_id) VALUES (?1, ?2, ?3)",
            params![entity_id, content.encode_to_vec(), body.map(|(id, _)| id)],
        )?;
        transaction.execute(
            "INSERT INTO intent (intent_id, entity_id, message) VALUES (?1, ?2, ?3)",
            params![intent.intent_id, entity_id, intent.encode_to_vec()],
        )?;
        transaction.commit()
    }

    /// Accept an edit: the person's new values over what is held, and the
    /// intent that will carry them.
    ///
    /// # Errors
    ///
    /// Fails when the store cannot be written to, or when nothing is held
    /// under `entity_id`.
    pub fn accept_edit(
        &mut self,
        entity_id: &[u8],
        patch: &v1::EntityContent,
        intent: &v1::Intent,
    ) -> rusqlite::Result<()> {
        let transaction = self.write()?;
        let held: Vec<u8> = transaction.query_row(
            "SELECT content FROM entity WHERE entity_id = ?1",
            params![entity_id],
            |row| row.get(0),
        )?;
        let mut content = decode(&held);
        merge(&mut content, patch);
        transaction.execute(
            "UPDATE entity SET content = ?2 WHERE entity_id = ?1",
            params![entity_id, content.encode_to_vec()],
        )?;
        transaction.execute(
            "INSERT INTO intent (intent_id, entity_id, message) VALUES (?1, ?2, ?3)",
            params![intent.intent_id, entity_id, intent.encode_to_vec()],
        )?;
        transaction.commit()
    }

    /// What is held under `entity_id`, where anything is.
    ///
    /// # Errors
    ///
    /// Fails when the store cannot be read.
    pub fn held(&self, entity_id: &[u8]) -> rusqlite::Result<Option<Held>> {
        let entity_id = &self.current_identity(entity_id)?;
        let row: Option<(Vec<u8>, Option<Vec<u8>>)> = self
            .connection
            .query_row(
                "SELECT content, body_id FROM entity WHERE entity_id = ?1",
                params![entity_id],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .optional()?;
        let Some((content, body_id)) = row else {
            return Ok(None);
        };
        let bytes = match body_id {
            Some(id) => self
                .connection
                .query_row(
                    "SELECT bytes FROM body WHERE body_id = ?1",
                    params![id],
                    |row| row.get(0),
                )
                .optional()?,
            None => None,
        };
        Ok(Some(Held {
            entity_id: entity_id.to_vec(),
            content: decode(&content),
            bytes,
        }))
    }

    /// Where an identity moved to, following a recreation. Answers what it
    /// was given when nothing moved.
    ///
    /// **Anything holding an identity needs this**, which is why it is here
    /// rather than beside whatever noticed the recreation. The substrate makes
    /// an identity stable from the moment something refers to it, and an
    /// erasure at the household is the one thing that breaks that; the client
    /// keeps the trail so nothing it held becomes unreachable.
    ///
    /// # Errors
    ///
    /// Fails when the store cannot be read.
    pub fn current_identity(&self, entity_id: &[u8]) -> rusqlite::Result<Vec<u8>> {
        let mut current = entity_id.to_vec();
        // Bounded: a chain longer than this is a loop, and following one
        // forever would hang the surface rather than answer it.
        for _ in 0..16 {
            let next: Option<Vec<u8>> = self
                .connection
                .query_row(
                    "SELECT now FROM renamed WHERE was = ?1",
                    params![current],
                    |row| row.get(0),
                )
                .optional()?;
            match next {
                Some(moved) => current = moved,
                None => break,
            }
        }
        Ok(current)
    }

    /// The next intents to send: the oldest outstanding one for each entity,
    /// oldest first, skipping entities something has been refused for.
    ///
    /// **At most one per entity, which is what makes the ordering rules
    /// hold.** A later intent for an entity is not offered until the one
    /// before it has been answered, so a create always reaches the instance
    /// before its own edits and an edit always carries the counter the write
    /// before it was acknowledged with.
    ///
    /// # Errors
    ///
    /// Fails when the store cannot be read.
    pub fn next_to_send(&self, limit: usize) -> rusqlite::Result<Vec<Pending>> {
        let mut statement = self.connection.prepare(
            "SELECT i.seq, i.intent_id, i.entity_id, i.message, e.counter
               FROM intent i
               JOIN entity e ON e.entity_id = i.entity_id
              WHERE i.refused = 0
                AND e.blocked = 0
                AND i.seq = (SELECT MIN(seq) FROM intent
                              WHERE entity_id = i.entity_id AND refused = 0)
              ORDER BY i.seq
              LIMIT ?1",
        )?;
        let rows = statement.query_map(params![limit], |row| {
            let message: Vec<u8> = row.get(3)?;
            let counter: i64 = row.get(4)?;
            Ok(Pending {
                seq: row.get(0)?,
                intent_id: row.get(1)?,
                entity_id: row.get(2)?,
                intent: v1::Intent::decode(message.as_slice())
                    .expect("the store holds intents this client encoded"),
                acknowledged_counter: counter.unsigned_abs(),
            })
        })?;
        rows.collect()
    }

    /// The bytes of a body that has not reached the instance yet.
    ///
    /// # Errors
    ///
    /// Fails when the store cannot be read.
    pub fn unsent_body(&self, body_id: &[u8]) -> rusqlite::Result<Option<Vec<u8>>> {
        self.connection
            .query_row(
                "SELECT bytes FROM body WHERE body_id = ?1 AND sent = 0",
                params![body_id],
                |row| row.get(0),
            )
            .optional()
    }

    /// Record that a body reached the instance.
    ///
    /// # Errors
    ///
    /// Fails when the store cannot be written to.
    pub fn body_sent(&mut self, body_id: &[u8]) -> rusqlite::Result<()> {
        self.connection.execute(
            "UPDATE body SET sent = 1 WHERE body_id = ?1",
            params![body_id],
        )?;
        Ok(())
    }

    /// The instance applied an intent: it leaves the outbox, and the counter
    /// it was answered with becomes what later edits are sent against.
    ///
    /// A retained conflict arrives here too and changes nothing. Both values
    /// were kept, the write applied, and there is nothing to retry.
    ///
    /// # Errors
    ///
    /// Fails when the store cannot be written to.
    pub fn applied(
        &mut self,
        intent_id: &[u8],
        entity_id: &[u8],
        counter: u64,
    ) -> rusqlite::Result<()> {
        let transaction = self.write()?;
        transaction.execute(
            "DELETE FROM intent WHERE intent_id = ?1",
            params![intent_id],
        )?;
        transaction.execute(
            "UPDATE entity SET counter = ?2 WHERE entity_id = ?1",
            params![entity_id, i64::try_from(counter).unwrap_or(i64::MAX)],
        )?;
        transaction.commit()
    }

    /// The instance refused an intent: it stays, it is not retried, and
    /// everything queued behind it for the same entity waits.
    ///
    /// Sending the queued ones would break the per entity order and would name
    /// an entity the instance may not hold. Nothing is dropped and nothing is
    /// errored away.
    ///
    /// # Errors
    ///
    /// Fails when the store cannot be written to.
    pub fn refused(&mut self, intent_id: &[u8], entity_id: &[u8]) -> rusqlite::Result<()> {
        let transaction = self.write()?;
        transaction.execute(
            "UPDATE intent SET refused = 1 WHERE intent_id = ?1",
            params![intent_id],
        )?;
        transaction.execute(
            "UPDATE entity SET blocked = 1 WHERE entity_id = ?1",
            params![entity_id],
        )?;
        transaction.commit()
    }

    /// The entity was erased at the household and the person's work came back
    /// under a new identity. The local entity adopts it, and so does
    /// everything still queued for it.
    ///
    /// Nothing was lost, so nothing is signalled.
    ///
    /// # Errors
    ///
    /// Fails when the store cannot be written to.
    pub fn recreated(
        &mut self,
        intent_id: &[u8],
        original: &[u8],
        new: &[u8],
        counter: u64,
    ) -> rusqlite::Result<()> {
        let transaction = self.write()?;
        transaction.execute(
            "DELETE FROM intent WHERE intent_id = ?1",
            params![intent_id],
        )?;
        transaction.execute(
            "UPDATE entity SET entity_id = ?2, counter = ?3 WHERE entity_id = ?1",
            params![original, new, i64::try_from(counter).unwrap_or(i64::MAX)],
        )?;
        transaction.execute(
            "INSERT OR REPLACE INTO renamed (was, now) VALUES (?1, ?2)",
            params![original, new],
        )?;

        // Everything still queued names the old identity, inside the encoded
        // intent as well as in the column. Both move.
        let queued: Vec<(i64, Vec<u8>)> = {
            let mut statement =
                transaction.prepare("SELECT seq, message FROM intent WHERE entity_id = ?1")?;
            let rows =
                statement.query_map(params![original], |row| Ok((row.get(0)?, row.get(1)?)))?;
            rows.collect::<rusqlite::Result<_>>()?
        };
        for (seq, message) in queued {
            let mut intent =
                v1::Intent::decode(message.as_slice()).expect("this client encoded it");
            crate::wire::rename_subject(&mut intent, new);
            transaction.execute(
                "UPDATE intent SET entity_id = ?2, message = ?3 WHERE seq = ?1",
                params![seq, new, intent.encode_to_vec()],
            )?;
        }
        transaction.commit()
    }

    /// How many intents the instance refused. **The only number this client
    /// ever states**, and the reason it is counted here rather than kept in
    /// memory: it is a fact about what is held, so it survives a restart and
    /// it cannot drift from what is actually in the outbox.
    ///
    /// # Errors
    ///
    /// Fails when the store cannot be read.
    pub fn refused_count(&self) -> rusqlite::Result<u64> {
        let count: i64 = self.connection.query_row(
            "SELECT COUNT(*) FROM intent WHERE refused = 1",
            [],
            |row| row.get(0),
        )?;
        Ok(count.unsigned_abs())
    }

    /// Bind this device to a household as `member_id`.
    ///
    /// # Errors
    ///
    /// Fails when the store cannot be written to.
    pub fn bind(&mut self, member_id: &[u8]) -> rusqlite::Result<()> {
        self.connection.execute(
            "INSERT OR REPLACE INTO binding (only, member_id) VALUES (1, ?1)",
            params![member_id],
        )?;
        Ok(())
    }
}

/// The client's own schema. One statement per thing it holds.
const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS entity (
  entity_id BLOB PRIMARY KEY,
  content   BLOB NOT NULL,
  body_id   BLOB,
  counter   INTEGER NOT NULL DEFAULT 0,
  blocked   INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS intent (
  seq       INTEGER PRIMARY KEY AUTOINCREMENT,
  intent_id BLOB NOT NULL UNIQUE,
  entity_id BLOB NOT NULL,
  message   BLOB NOT NULL,
  refused   INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX IF NOT EXISTS intent_by_entity ON intent (entity_id, seq);

CREATE TABLE IF NOT EXISTS renamed (
  was BLOB PRIMARY KEY,
  now BLOB NOT NULL
);

CREATE TABLE IF NOT EXISTS body (
  body_id BLOB PRIMARY KEY,
  bytes   BLOB NOT NULL,
  sent    INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS binding (
  only      INTEGER PRIMARY KEY CHECK (only = 1),
  member_id BLOB NOT NULL
);
";

fn decode(bytes: &[u8]) -> v1::EntityContent {
    v1::EntityContent::decode(bytes).expect("the store holds content this client encoded")
}

/// Lay a person's edit over what is held.
///
/// **Covers what this client can currently compose, and no more.** The wire's
/// rule is that a present field is set, a listed field is cleared and an absent
/// field is untouched; the client only ever composes a title or a note's body,
/// so those are what this reads. A field this client learns to edit is a field
/// this function learns to merge, and the compiler will not say so.
fn merge(held: &mut v1::EntityContent, patch: &v1::EntityContent) {
    if patch.title.is_some() {
        held.title.clone_from(&patch.title);
    }
    if let Some(v1::entity_content::Specific::Note(edited)) = patch.specific.as_ref()
        && edited.body.is_some()
        && let Some(v1::entity_content::Specific::Note(note)) = held.specific.as_mut()
    {
        note.body.clone_from(&edited.body);
    }
}

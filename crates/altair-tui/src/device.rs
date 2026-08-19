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
    /// Where the instance says this entity stands. Active for something the
    /// instance has not answered about yet, because a capture is the person's
    /// from the moment it is accepted.
    pub lifecycle: v1::LifecycleState,
    /// Whether anything about this entity is still on its way to the instance.
    /// **A pending write is what makes the local view outrank the instance's**,
    /// and it is the only thing that does.
    pub pending: bool,
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
        let entity_id = self.current_identity(entity_id)?;
        let local: Option<(Vec<u8>, Option<Vec<u8>>)> = self
            .connection
            .query_row(
                "SELECT content, body_id FROM entity WHERE entity_id = ?1",
                params![entity_id],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .optional()?;
        let truth: Option<Vec<u8>> = self
            .connection
            .query_row(
                "SELECT message FROM replica WHERE entity_id = ?1",
                params![entity_id],
                |row| row.get(0),
            )
            .optional()?;
        if local.is_none() && truth.is_none() {
            return Ok(None);
        }
        let pending = self.pending_for(&entity_id)?;
        let entity = truth.map(|bytes| {
            v1::Entity::decode(bytes.as_slice()).expect("the replica holds what the instance sent")
        });

        // **The overlay, and the whole of it.** Instance truth is what the
        // household agreed; the local view outranks it only while something
        // local has not got there yet. Once nothing is pending the two agree,
        // except where somebody else moved the entity — and then the
        // instance's is the newer one and should win.
        let content = match (&local, &entity) {
            (Some((held, _)), _) if pending => decode(held),
            (_, Some(entity)) => entity.content.clone().unwrap_or_default(),
            (Some((held, _)), None) => decode(held),
            (None, None) => unreachable!("one of the two is present"),
        };
        let bytes = match local.as_ref().and_then(|(_, body_id)| body_id.clone()) {
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
            entity_id,
            content,
            bytes,
            lifecycle: entity
                .as_ref()
                .and_then(|entity| v1::LifecycleState::try_from(entity.lifecycle).ok())
                .unwrap_or(v1::LifecycleState::Active),
            pending,
        }))
    }

    /// Whether anything for this entity is still outstanding.
    ///
    /// A refused intent counts: it is still held, the person's work is still
    /// theirs, and the instance has not taken it.
    ///
    /// # Errors
    ///
    /// Fails when the store cannot be read.
    pub fn pending_for(&self, entity_id: &[u8]) -> rusqlite::Result<bool> {
        let count: i64 = self.connection.query_row(
            "SELECT COUNT(*) FROM intent WHERE entity_id = ?1",
            params![entity_id],
            |row| row.get(0),
        )?;
        Ok(count > 0)
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

    /// Where in the instance's change sequence this device has read to.
    ///
    /// Zero is the beginning, and the beginning is always answerable while
    /// nothing trims the sequence. A client may always rebuild.
    ///
    /// # Errors
    ///
    /// Fails when the store cannot be read.
    pub fn position(&self) -> rusqlite::Result<u64> {
        let value: Option<i64> = self
            .connection
            .query_row(
                "SELECT value FROM replica_position WHERE only = 1",
                [],
                |row| row.get(0),
            )
            .optional()?;
        Ok(value.unwrap_or(0).unsigned_abs())
    }

    /// Take a page of the member's change stream.
    ///
    /// **One transaction, position included.** The position is what says which
    /// changes have been seen, so recording it apart from them would let a kill
    /// in between either lose changes or replay them. Replaying is harmless —
    /// every change here is a statement of what is, not a delta — but losing
    /// them is not, and one commit costs nothing.
    ///
    /// A change carries what the *member* sees. An audience broadening arrives
    /// as a creation and a narrowing as a disappearance, and neither has a
    /// variant of its own, so there is nothing here to distinguish them with
    /// and nothing that tries.
    ///
    /// # Errors
    ///
    /// Fails when the store cannot be written to.
    pub fn take_changes(&mut self, changes: &[v1::Change], position: u64) -> rusqlite::Result<()> {
        let transaction = self.write()?;
        for change in changes {
            match change.what.as_ref() {
                Some(v1::change::What::Entity(entity)) => {
                    transaction.execute(
                        "INSERT OR REPLACE INTO replica (entity_id, message) VALUES (?1, ?2)",
                        params![entity.entity_id, entity.encode_to_vec()],
                    )?;
                    let placed = Placement::of(entity);
                    transaction.execute(
                        "INSERT OR REPLACE INTO placement
                           (entity_id, kind, container_id, container_position,
                            category_id, category_position)
                         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                        params![
                            entity.entity_id,
                            placed.kind,
                            placed.container_id,
                            placed.container_position,
                            placed.category_id,
                            placed.category_position,
                        ],
                    )?;
                }
                Some(v1::change::What::GoneEntityId(id)) => {
                    transaction.execute("DELETE FROM replica WHERE entity_id = ?1", params![id])?;
                    transaction
                        .execute("DELETE FROM placement WHERE entity_id = ?1", params![id])?;
                    // A relation reaches a member only when they can see both
                    // of its endpoints, so an endpoint going takes the
                    // relations touching it with it. The instance says so too;
                    // not waiting to be told is what keeps a relation from
                    // being briefly readable with one end missing.
                    transaction.execute(
                        "DELETE FROM replica_relation WHERE relation_id IN
                           (SELECT relation_id FROM relation_end WHERE entity_id = ?1)",
                        params![id],
                    )?;
                    transaction
                        .execute("DELETE FROM relation_end WHERE entity_id = ?1", params![id])?;
                }
                Some(v1::change::What::Relation(relation)) => {
                    transaction.execute(
                        "INSERT OR REPLACE INTO replica_relation (relation_id, message) \
                         VALUES (?1, ?2)",
                        params![relation.relation_id, relation.encode_to_vec()],
                    )?;
                    // An edit can move an endpoint, so the ends are rewritten
                    // rather than added to.
                    transaction.execute(
                        "DELETE FROM relation_end WHERE relation_id = ?1",
                        params![relation.relation_id],
                    )?;
                    for end in endpoints(relation) {
                        transaction.execute(
                            "INSERT OR REPLACE INTO relation_end (relation_id, entity_id) \
                             VALUES (?1, ?2)",
                            params![relation.relation_id, end],
                        )?;
                    }
                }
                Some(v1::change::What::GoneRelationId(id)) => {
                    transaction.execute(
                        "DELETE FROM replica_relation WHERE relation_id = ?1",
                        params![id],
                    )?;
                    transaction.execute(
                        "DELETE FROM relation_end WHERE relation_id = ?1",
                        params![id],
                    )?;
                }
                None => {}
            }
        }
        transaction.execute(
            "INSERT OR REPLACE INTO replica_position (only, value) VALUES (1, ?1)",
            params![i64::try_from(position).unwrap_or(i64::MAX)],
        )?;
        transaction.commit()
    }

    /// Throw the replica away and start again from the beginning.
    ///
    /// **What the instance holds is authoritative and what is here is a copy**,
    /// so this costs time and never data. It is the whole of the client's
    /// response to being told its position is past the horizon: rebuilding is
    /// always available and always correct. Nothing local is touched — an
    /// unsent capture is not the instance's to have an opinion about.
    ///
    /// # Errors
    ///
    /// Fails when the store cannot be written to.
    pub fn rebuild(&mut self) -> rusqlite::Result<()> {
        let transaction = self.write()?;
        transaction.execute("DELETE FROM replica", [])?;
        transaction.execute("DELETE FROM replica_relation", [])?;
        transaction.execute("DELETE FROM placement", [])?;
        transaction.execute("DELETE FROM relation_end", [])?;
        transaction.execute("DELETE FROM replica_position", [])?;
        transaction.commit()
    }

    /// The instance's own copy of an entity, where the change stream has
    /// delivered one.
    ///
    /// **Everything the client does not write is here rather than in the
    /// content**: who authored it, when it was last touched, the counter, the
    /// lifecycle, the body's blocks and both sides of any retained conflict. A
    /// detail screen wants all of it, and none of it can be inferred from what
    /// this device sent.
    ///
    /// # Errors
    ///
    /// Fails when the store cannot be read.
    pub fn instance_copy(&self, entity_id: &[u8]) -> rusqlite::Result<Option<v1::Entity>> {
        let entity_id = self.current_identity(entity_id)?;
        let message: Option<Vec<u8>> = self
            .connection
            .query_row(
                "SELECT message FROM replica WHERE entity_id = ?1",
                params![entity_id],
                |row| row.get(0),
            )
            .optional()?;
        Ok(message.map(|bytes| {
            v1::Entity::decode(bytes.as_slice()).expect("the replica holds what the instance sent")
        }))
    }

    /// What the person has captured, most recent first.
    ///
    /// **Assembled here because there is nowhere else it could be.** The
    /// instance serves no way to read an entity by identity and no way to list
    /// what a container holds; `Query` is the literal arm and cannot enumerate.
    /// Every screen that is not search is built from what the change stream
    /// delivered, laid under what this device has not sent yet.
    ///
    /// # Errors
    ///
    /// Fails when the store cannot be read.
    pub fn captures(&self) -> rusqlite::Result<Vec<Held>> {
        let mut statement = self.connection.prepare(
            "SELECT entity_id FROM replica
             UNION
             SELECT entity_id FROM entity",
        )?;
        let ids: Vec<Vec<u8>> = statement
            .query_map([], |row| row.get(0))?
            .collect::<rusqlite::Result<_>>()?;
        let mut out = Vec::new();
        for id in ids {
            if let Some(held) = self.held(&id)? {
                // Removal is reversible and the removed list is its own screen.
                // The captures list is what is here now.
                if held.lifecycle != v1::LifecycleState::Deleted
                    && held.lifecycle != v1::LifecycleState::Erased
                {
                    out.push(held);
                }
            }
        }
        Ok(out)
    }

    /// What sits directly beneath `container`, in the order it sits there.
    ///
    /// **One question, three screens.** A campaign's arcs and quests, a quest's
    /// place beneath its arc, and a location's contents are the same shape —
    /// something beneath something else, in an order that belongs to the
    /// container rather than to the thing. The wire spells the parent field
    /// differently for each type, and [`Placement`] is where that ends.
    ///
    /// # Errors
    ///
    /// Fails when the store cannot be read.
    pub fn beneath(&self, container: &[u8]) -> rusqlite::Result<Vec<Held>> {
        let mut statement = self.connection.prepare(
            "SELECT entity_id FROM placement
              WHERE container_id = ?1
              ORDER BY container_position IS NULL, container_position, entity_id",
        )?;
        let ids: Vec<Vec<u8>> = statement
            .query_map(params![container], |row| row.get(0))?
            .collect::<rusqlite::Result<_>>()?;
        self.all(&ids)
    }

    /// What sits directly in `category`, in the order it sits there.
    ///
    /// # Errors
    ///
    /// Fails when the store cannot be read.
    pub fn within(&self, category: &[u8]) -> rusqlite::Result<Vec<Held>> {
        let mut statement = self.connection.prepare(
            "SELECT entity_id FROM placement
              WHERE category_id = ?1
              ORDER BY category_position IS NULL, category_position, entity_id",
        )?;
        let ids: Vec<Vec<u8>> = statement
            .query_map(params![category], |row| row.get(0))?
            .collect::<rusqlite::Result<_>>()?;
        self.all(&ids)
    }

    /// Every relation touching `entity_id`, from either end.
    ///
    /// **There is no reverse row and there is not supposed to be.** One record
    /// joins two entities and direction is a property of that record, so
    /// "what points at this" and "what this points at" are the same lookup
    /// asked from different sides.
    ///
    /// # Errors
    ///
    /// Fails when the store cannot be read.
    pub fn relations_touching(&self, entity_id: &[u8]) -> rusqlite::Result<Vec<v1::Relation>> {
        let entity_id = self.current_identity(entity_id)?;
        let mut statement = self.connection.prepare(
            "SELECT r.message FROM replica_relation r
               JOIN relation_end e ON e.relation_id = r.relation_id
              WHERE e.entity_id = ?1
              ORDER BY r.relation_id",
        )?;
        let rows: Vec<Vec<u8>> = statement
            .query_map(params![entity_id], |row| row.get(0))?
            .collect::<rusqlite::Result<_>>()?;
        Ok(rows
            .iter()
            .map(|bytes| {
                v1::Relation::decode(bytes.as_slice())
                    .expect("the replica holds what the instance sent")
            })
            .collect())
    }

    fn all(&self, ids: &[Vec<u8>]) -> rusqlite::Result<Vec<Held>> {
        let mut out = Vec::new();
        for id in ids {
            if let Some(held) = self.held(id)? {
                out.push(held);
            }
        }
        Ok(out)
    }

    /// The credential this device presents.
    ///
    /// # Errors
    ///
    /// Fails when the store cannot be read.
    pub fn credential(&self) -> rusqlite::Result<Option<String>> {
        self.connection
            .query_row("SELECT token FROM credential WHERE only = 1", [], |row| {
                row.get(0)
            })
            .optional()
    }

    /// Put a credential where this device will find it again.
    ///
    /// # Errors
    ///
    /// Fails when the store cannot be written to.
    pub fn set_credential(&mut self, token: &str) -> rusqlite::Result<()> {
        self.connection.execute(
            "INSERT OR REPLACE INTO credential (only, token) VALUES (1, ?1)",
            params![token],
        )?;
        Ok(())
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

CREATE TABLE IF NOT EXISTS replica (
  entity_id BLOB PRIMARY KEY,
  message   BLOB NOT NULL
);

-- Where each entity sits, and where in it. Derived from the messages beside
-- it and rebuildable from them; nothing here is authoritative.
CREATE TABLE IF NOT EXISTS placement (
  entity_id          BLOB PRIMARY KEY,
  kind               TEXT NOT NULL,
  container_id       BLOB,
  container_position INTEGER,
  category_id        BLOB,
  category_position  INTEGER
);
CREATE INDEX IF NOT EXISTS placement_by_container
  ON placement (container_id, container_position);
CREATE INDEX IF NOT EXISTS placement_by_category
  ON placement (category_id, category_position);

-- Both ends of every relation. The wire has one record per connection and no
-- reverse row, because direction is a property of the record and is read from
-- either end. Reading from either end cheaply is what this is for.
CREATE TABLE IF NOT EXISTS relation_end (
  relation_id BLOB NOT NULL,
  entity_id   BLOB NOT NULL,
  PRIMARY KEY (relation_id, entity_id)
);
CREATE INDEX IF NOT EXISTS relation_end_by_entity ON relation_end (entity_id);

CREATE TABLE IF NOT EXISTS replica_relation (
  relation_id BLOB PRIMARY KEY,
  message     BLOB NOT NULL
);

CREATE TABLE IF NOT EXISTS replica_position (
  only  INTEGER PRIMARY KEY CHECK (only = 1),
  value INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS renamed (
  was BLOB PRIMARY KEY,
  now BLOB NOT NULL
);

CREATE TABLE IF NOT EXISTS body (
  body_id BLOB PRIMARY KEY,
  bytes   BLOB NOT NULL,
  sent    INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS credential (
  only  INTEGER PRIMARY KEY CHECK (only = 1),
  token TEXT NOT NULL
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

/// Where an entity sits, read out of the message once so the screens do not
/// each have to know which field their type spells it in.
///
/// **Derived, and therefore never authoritative.** Everything here can be
/// recomputed from the messages beside it, which is what makes a rebuild a
/// truncate and a replay rather than a reconciliation.
struct Placement {
    kind: &'static str,
    container_id: Option<Vec<u8>>,
    container_position: Option<i64>,
    category_id: Option<Vec<u8>>,
    category_position: Option<i64>,
}

impl Placement {
    fn of(entity: &v1::Entity) -> Self {
        let content = entity.content.clone().unwrap_or_default();
        let position = |value: Option<u32>| value.map(i64::from);
        let (kind, container_id, container_position) = match content.specific.as_ref() {
            Some(v1::entity_content::Specific::Campaign(_)) => ("campaign", None, None),
            Some(v1::entity_content::Specific::Arc(arc)) => (
                "arc",
                arc.campaign_id.clone(),
                position(arc.ladder_position),
            ),
            // At most one ladder parent in total, an arc or a campaign, never
            // both — so this reads as one field however it was spelled.
            Some(v1::entity_content::Specific::Quest(quest)) => (
                "quest",
                match quest.parent.as_ref() {
                    Some(v1::quest_content::Parent::ArcId(id)) => Some(id.clone()),
                    Some(v1::quest_content::Parent::CampaignId(id)) => Some(id.clone()),
                    None => None,
                },
                position(quest.ladder_position),
            ),
            Some(v1::entity_content::Specific::Location(location)) => {
                ("location", location.parent_location_id.clone(), None)
            }
            Some(v1::entity_content::Specific::Routine(_)) => ("routine", None, None),
            Some(v1::entity_content::Specific::FocusSession(_)) => ("focus_session", None, None),
            Some(v1::entity_content::Specific::CheckIn(_)) => ("check_in", None, None),
            Some(v1::entity_content::Specific::Note(_)) => ("note", None, None),
            Some(v1::entity_content::Specific::File(_)) => ("file", None, None),
            Some(v1::entity_content::Specific::Item(_)) => ("item", None, None),
            Some(v1::entity_content::Specific::ShoppingList(_)) => ("shopping_list", None, None),
            Some(v1::entity_content::Specific::Category(_)) => ("category", None, None),
            // The type is the tag of content.specific, so an entity without one
            // has no type. It is still the person's and still listed.
            None => ("untyped", None, None),
        };
        Self {
            kind,
            container_id,
            container_position,
            category_id: content.category_id.clone(),
            category_position: position(content.category_position),
        }
    }
}

/// Both ends of a relation, where it has them.
fn endpoints(relation: &v1::Relation) -> Vec<Vec<u8>> {
    let Some(content) = relation.content.as_ref() else {
        return Vec::new();
    };
    content
        .from_entity_id
        .iter()
        .chain(content.to_entity_id.iter())
        .cloned()
        .collect()
}

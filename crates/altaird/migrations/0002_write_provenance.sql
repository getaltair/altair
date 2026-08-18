-- Altair structured store, migration two: per-part write provenance, and a
-- lifecycle on a relation.
--
-- Status: Draft.
-- Governed by: DR-002, Altair Substrate Specification, Altair Component Model,
--              Altair v0 Implementation Plan (Wave 2.1).
--
-- Taken once, at the start of the intent spine, because everything in that
-- item reads one or the other. The two changes are unrelated to each other and
-- are here together only because they were both found by reading Wave 2.1
-- against the documents before writing any of it.
--
-- The rules migration one states about what is a constraint and what is a
-- write-path check are unchanged and are not restated.

-- ---------------------------------------------------------------------------
-- Per-part write provenance
-- ---------------------------------------------------------------------------
--
-- Conflict detection asks a question nothing in migration one could answer.
-- The rule from the substrate is that a stale base touching parts disjoint
-- from what moved since applies without conflict, and evaluating it needs to
-- know *which parts moved between two counter values*.
--
-- Nothing held that. The change sequence carries block identities but no field
-- list and no counter; an intent row carries the counter after a write but not
-- what it wrote; and versions are Knowledge-only and a household may decline
-- them entirely.
--
-- So: one row per part of an entity that has ever been written, holding the
-- counter the part last moved at and the member who moved it. The member is
-- here because a conflict names whose the other value was, and an entity's
-- author is its creator and never changes, so the author cannot answer it.
--
-- A part is a field, except in a body, where it is the block. That is the same
-- shape the conflict table already uses, and the two are deliberately spelled
-- the same way so a reader meeting one recognises the other.
--
-- Rejected: a map of parts to counters as a document column on the entity row.
-- Fewer moving parts, and it would die with the row rather than needing its own
-- removal on erasure. Against it: the shape would be unenforced by the store,
-- and a document column in a schema that rejected one for content invites the
-- argument every time somebody reads it.
--
-- Rejected: the changed parts and the counter on the change row. It reuses a
-- write the transaction already makes and adds no table. Against it: the change
-- sequence is trimmed below the horizon, so conflict correctness would decay as
-- history is trimmed, and making the write path depend on a trimmer having not
-- yet run is the wrong direction.

CREATE TABLE entity_part_counter (
  entity_id  uuid NOT NULL REFERENCES entity (id) ON DELETE CASCADE,

  -- The part is the field, except in a body, where it is the block. Exactly
  -- one of these is set, as on the conflict table.
  --
  -- The name is text here and a field number on the wire. Something has to
  -- hold that mapping; migration one recorded that as gap (d) and it is still
  -- true. What is new is that there are now two tables keyed by it, so the
  -- mapping has to be written once rather than once per table.
  field_name text,
  block_id   uuid REFERENCES block (id) ON DELETE CASCADE,

  -- The entity counter this part last moved at. Compared against an edit's
  -- base counter, and never shown to anyone.
  counter    bigint NOT NULL,

  -- Who moved it. A conflict names whose the other value was, and this is
  -- where that comes from.
  member_id  uuid NOT NULL REFERENCES membership (id),

  CHECK (num_nonnulls(field_name, block_id) = 1),

  -- NULLS NOT DISTINCT because exactly one of the two key columns is set, so
  -- the default null-is-distinct behaviour would let the same part be recorded
  -- any number of times.
  UNIQUE NULLS NOT DISTINCT (entity_id, field_name, block_id)
);

-- ---------------------------------------------------------------------------
-- A relation joins the holding state
-- ---------------------------------------------------------------------------
--
-- The wire can name a relation in a removal and migration one had nowhere to
-- hold the result. The substrate is explicit that removing a relation is
-- reversible on the same terms as removing an entity: it leaves traversal, it
-- comes back with the act that removed it, and there is no restoring one on
-- its own.
--
-- This needs no change to the wire. A removal is already the relation-gone
-- signal, a restoration is an ordinary relation write, and restoring an entity
-- with its group reaches the relations in that group.
--
-- Rejected: a hard delete, documented as such. It is what migration one and
-- the wire already implied and it would have cost one sentence. Against it: it
-- would be the only permanent destructive act in the design with no holding
-- state and no announcement, in a document that says anything becoming
-- permanent does so visibly and never as a side effect.

ALTER TABLE relation
  ADD COLUMN lifecycle         lifecycle_state NOT NULL DEFAULT 'active',
  ADD COLUMN deleted_at        timestamptz,
  ADD COLUMN deletion_group_id uuid;

-- The same two invariants the entity row carries, spelled the same way.
ALTER TABLE relation
  ADD CONSTRAINT relation_holding_state
    CHECK ((lifecycle = 'deleted') = (deleted_at IS NOT NULL)),
  ADD CONSTRAINT relation_group_only_when_deleted
    CHECK (deletion_group_id IS NULL OR lifecycle = 'deleted');

-- A relation is never a tombstone. Erasing either endpoint removes the
-- relation outright, which is what erasure means everywhere, and there is
-- nothing for a tombstone to carry: no edit can arrive for a relation that
-- needs recognising as a recreation, because a relation has no content a
-- person would retype.
--
-- The enum is shared with the entity row rather than a second one being
-- declared, because the two states a relation does have mean exactly what they
-- mean there.
ALTER TABLE relation
  ADD CONSTRAINT relation_is_never_a_tombstone
    CHECK (lifecycle <> 'erased');

-- Restoring an act reaches the relations it removed, which is a read by group.
CREATE INDEX relation_group_idx ON relation (deletion_group_id)
  WHERE deletion_group_id IS NOT NULL;

-- The holding window expires by comparison against deleted_at, and it now
-- reaches relations as well as entities.
CREATE INDEX relation_holding_idx ON relation (deleted_at)
  WHERE lifecycle = 'deleted';

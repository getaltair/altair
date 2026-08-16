-- Altair structured store, version 1.
--
-- Status: Draft.
-- Governed by: DR-002, DR-001, Altair Data Model, Altair Substrate Specification,
--              Altair Architecture Foundations, Altair Component Model.
--
-- PostgreSQL 18 is the baseline per DR-002. Applied and exercised on 18.6 with
-- pgvector 0.8.6.
--
-- search_vector is declared STORED explicitly rather than relying on a default.
-- On 18 a generated column with no keyword is virtual, which would recompute
-- the tsvector per row read and could not be indexed, so the keyword is doing
-- work rather than restating a default.
--
-- Two rules decide what is a constraint here and what is a check in the write
-- path, and they are stated once rather than argued at each table.
--
--   In the store: anything answerable from the row being written or from a
--   key it points at. Referential integrity, the type of the thing a reference
--   points at, cardinality, closed value sets, uniqueness.
--
--   In the write path: anything needing the requesting member, the prior state
--   of the row, or a declaration held in another table. Audience, the counter
--   and conflict detection, type immutability, canonical ordering of symmetric
--   relations, and whether a relation property is permitted by its type.
--
-- No triggers. A trigger is a third enforcement site that reads like the
-- store's and behaves like the write path's, and the design elsewhere prefers
-- one rule kept true in several places to two rules that can disagree.
--
-- What is not created here, and why, is at the foot of the file, along with
-- the recommendations and the gaps this pass found.

BEGIN;

CREATE EXTENSION IF NOT EXISTS pg_trgm;
CREATE EXTENSION IF NOT EXISTS vector;

-- ---------------------------------------------------------------------------
-- Closed value sets
-- ---------------------------------------------------------------------------
--
-- capture_method is deliberately absent from this list. The substrate states
-- the set of methods is open and grows as clients add ways to create things,
-- so it is text and carries no constraint.

CREATE TYPE entity_type AS ENUM (
  'campaign', 'arc', 'quest', 'routine', 'focus_session', 'check_in',
  'note', 'file', 'item', 'location', 'shopping_list', 'category'
);

CREATE TYPE lifecycle_state AS ENUM ('active', 'deleted', 'erased');

-- Names are an open question in the Guidance PRD.
CREATE TYPE guidance_state AS ENUM ('waiting', 'working', 'worked');

CREATE TYPE uses_resolution AS ENUM ('unresolved', 'returned', 'consumed');

CREATE TYPE property_kind AS ENUM ('text', 'date', 'number', 'member', 'boolean');

CREATE TYPE change_kind AS ENUM (
  'entity_written', 'entity_gone', 'relation_written', 'relation_gone'
);

CREATE TYPE derivation_kind AS ENUM ('embedding', 'text_extraction');

CREATE TYPE event_kind AS ENUM ('consumption', 'purchase');

CREATE TYPE intent_outcome AS ENUM ('applied', 'refused', 'recreated');

-- Nothing distinguishes an entity that does not exist from one the submitting
-- member cannot see, here or on the wire.
CREATE TYPE refusal_reason AS ENUM ('not_available', 'malformed');

-- ---------------------------------------------------------------------------
-- Household and membership
-- ---------------------------------------------------------------------------
--
-- One household per instance, so nothing carries a household reference and no
-- query is scoped by one. The table exists because a membership belongs to a
-- household and because the model should not have to change when that stops
-- being true, not because anything here arbitrates between two.

CREATE TABLE household (
  id          uuid PRIMARY KEY,
  name        text,
  created_at  timestamptz NOT NULL
);

CREATE TABLE membership (
  id            uuid PRIMARY KEY,
  household_id  uuid NOT NULL REFERENCES household (id),

  -- The subject claim from the token, per DR-005. The membership is the
  -- internal thing; this is the external key onto it.
  subject       text NOT NULL,

  display_name  text,

  -- Any number of members may hold it, more than one is ordinary, and nothing
  -- assigns it. A person takes it.
  administrator boolean NOT NULL DEFAULT false,

  joined_at     timestamptz NOT NULL,

  -- Set on departure. References to this membership survive it: authorship
  -- never changes and an audience entry stays where it is.
  departed_at   timestamptz,

  UNIQUE (household_id, subject)
);

-- ---------------------------------------------------------------------------
-- Templates
-- ---------------------------------------------------------------------------
--
-- One shared set reaching items and locations. Contributes property names,
-- never values. Following is live, so renaming a property reaches everything
-- following it, which is why a value is keyed by name above.

CREATE TABLE template (
  id         uuid PRIMARY KEY,
  name       text NOT NULL,
  created_at timestamptz NOT NULL
);

CREATE TABLE template_property (
  template_id     uuid NOT NULL REFERENCES template (id) ON DELETE CASCADE,
  name            text NOT NULL,

  -- A kind is not a constraint. It says how a surface should treat a value,
  -- and the value crosses and is stored as text either way.
  kind            property_kind NOT NULL DEFAULT 'text',

  ordinal         integer NOT NULL,

  -- A date property may seed from another date and an offset, once. It never
  -- recomputes, never overwrites, and never reaches an entity retrospectively.
  seed_from_label text,
  seed_offset     interval,

  CHECK ((seed_from_label IS NULL) = (seed_offset IS NULL)),
  CHECK (seed_from_label IS NULL OR kind = 'date'),

  PRIMARY KEY (template_id, name)
);

-- ---------------------------------------------------------------------------
-- Entities
-- ---------------------------------------------------------------------------
--
-- One table for the shared model, with a side table per type for what a type
-- holds beyond it. Not a table per type, because a cross domain query has to
-- reach every type without a per type barrier and both search indexes have to
-- sit beside the audience predicate in the query that produces candidates. Not
-- a document column either, because DR-002 validates shape on write and a
-- document column moves shape interpretation into the read path, which is the
-- component expected to churn.

CREATE TABLE entity (
  id                  uuid PRIMARY KEY,

  -- Fixed at creation. The store cannot see that it is fixed; the write path
  -- refuses an update that changes it.
  type                entity_type NOT NULL,

  title               text,

  -- Absent on an entity created before its device was bound to a household,
  -- and acquired at binding.
  author_member_id    uuid REFERENCES membership (id),

  -- Set once by the creating client. Settles the order of nothing.
  created_at          timestamptz NOT NULL,
  updated_at          timestamptz NOT NULL,

  -- An open set. See the note above the value sets.
  capture_method      text NOT NULL,

  -- Whether the entity is still filterable as bulk captured. Derived from
  -- capture_method at creation and mutable from there.
  bulk                boolean NOT NULL DEFAULT false,

  -- At most one, required by none. Containment, not a relation.
  category_id         uuid,
  category_type       entity_type NOT NULL DEFAULT 'category'
                        CHECK (category_type = 'category'),

  -- Position within that category. An order belongs to a container rather
  -- than to an entity, so it exists exactly where containment does, and
  -- leaving the category discards it. Assigned by the write path, which
  -- appends on entry.
  category_position   integer,

  -- Who else can see it. Empty is private to the author, which is the default.
  -- Enumerated rather than standing, so a shared thing does not narrow when
  -- somebody leaves.
  audience_member_ids uuid[] NOT NULL DEFAULT '{}',

  lifecycle           lifecycle_state NOT NULL DEFAULT 'active',

  -- When the entity entered the holding state. The holding window expires by
  -- comparison against this rather than by anything writing on a schedule.
  deleted_at          timestamptz,

  -- Where one act removed several entities, that act. Restoring any of them
  -- can bring back the rest.
  deletion_group_id   uuid,

  -- Advances on every accepted write. Detects concurrency, never gates
  -- admission, and is never shown to anyone.
  counter             bigint NOT NULL DEFAULT 1,

  -- Maintained by the write path from the title and whatever the type
  -- contributes, because those live in side tables and a generated column
  -- cannot reach them. The tsvector is generated from it, so the two cannot
  -- drift, and the plain text stays for trigram matching.
  search_text         text NOT NULL DEFAULT '',
  search_vector       tsvector GENERATED ALWAYS AS
                        (to_tsvector('english', search_text)) STORED,

  -- The target of every typed reference below. Redundant for uniqueness and
  -- present so a reference can be constrained to point at the right type.
  UNIQUE (id, type),

  FOREIGN KEY (category_id, category_type) REFERENCES entity (id, type),

  CHECK ((lifecycle = 'deleted') = (deleted_at IS NOT NULL)),
  CHECK (deletion_group_id IS NULL OR lifecycle = 'deleted'),
  CHECK ((category_id IS NULL) = (category_position IS NULL))
);

-- Erasure strips content and leaves the row as a tombstone rather than
-- deleting it, for two reasons: an edit arriving from a device that was away
-- has to be recognisable as an edit to something erased, which is what makes
-- it a recreation rather than a create, and the change sequence has to be able
-- to say the entity is gone rather than leaving absence to be inferred.

-- ---------------------------------------------------------------------------
-- Type specific content
-- ---------------------------------------------------------------------------
--
-- Each side table carries a constant type column so the composite foreign key
-- can refuse an entity of the wrong type. That is one extra column per typed
-- reference, which is the price of the store refusing a quest whose ladder
-- parent is a note rather than the write path remembering to.
--
-- There is no note table. A note holds a body beyond the shared set, a body is
-- its blocks in order, and there is no second representation of the same
-- content for them to disagree with. The same is true of a shopping list,
-- whose entries are blocks.

CREATE TABLE campaign (
  entity_id   uuid PRIMARY KEY,
  entity_type entity_type NOT NULL DEFAULT 'campaign'
                CHECK (entity_type = 'campaign'),

  -- The person's own state, never a summary of what is beneath it.
  state       guidance_state NOT NULL DEFAULT 'waiting',

  FOREIGN KEY (entity_id, entity_type) REFERENCES entity (id, type)
    ON DELETE CASCADE
);

CREATE TABLE arc (
  entity_id     uuid PRIMARY KEY,
  entity_type   entity_type NOT NULL DEFAULT 'arc'
                  CHECK (entity_type = 'arc'),
  state         guidance_state NOT NULL DEFAULT 'waiting',

  campaign_id   uuid,
  campaign_type entity_type NOT NULL DEFAULT 'campaign'
                  CHECK (campaign_type = 'campaign'),

  -- Position beneath that campaign, which orders arcs and quests together
  -- because a campaign is one container whatever kind its children are.
  ladder_position integer,

  FOREIGN KEY (entity_id, entity_type) REFERENCES entity (id, type)
    ON DELETE CASCADE,
  FOREIGN KEY (campaign_id, campaign_type) REFERENCES entity (id, type),

  CHECK ((campaign_id IS NULL) = (ladder_position IS NULL))
);

CREATE TABLE quest (
  entity_id     uuid PRIMARY KEY,
  entity_type   entity_type NOT NULL DEFAULT 'quest'
                  CHECK (entity_type = 'quest'),
  state         guidance_state NOT NULL DEFAULT 'waiting',

  -- At most one ladder parent in total, an arc or a campaign, never both.
  arc_id        uuid,
  arc_type      entity_type NOT NULL DEFAULT 'arc'
                  CHECK (arc_type = 'arc'),
  campaign_id   uuid,
  campaign_type entity_type NOT NULL DEFAULT 'campaign'
                  CHECK (campaign_type = 'campaign'),

  -- Held independently of the ladder parent. A quest that loses its routine is
  -- an ordinary quest. Untyped for now because the routine table is not
  -- created; see the foot of the file.
  routine_id    uuid,

  -- Position beneath whichever ladder parent it has.
  ladder_position integer,

  CHECK (num_nonnulls(arc_id, campaign_id) <= 1),
  CHECK ((num_nonnulls(arc_id, campaign_id) = 0) = (ladder_position IS NULL)),

  FOREIGN KEY (entity_id, entity_type) REFERENCES entity (id, type)
    ON DELETE CASCADE,
  FOREIGN KEY (arc_id, arc_type) REFERENCES entity (id, type),
  FOREIGN KEY (campaign_id, campaign_type) REFERENCES entity (id, type)
);

CREATE TABLE file (
  entity_id   uuid PRIMARY KEY,
  entity_type entity_type NOT NULL DEFAULT 'file'
                CHECK (entity_type = 'file'),

  -- The key into the object store. Bytes are written before this row exists.
  body_id     uuid NOT NULL,

  media_type  text,
  byte_size   bigint,

  FOREIGN KEY (entity_id, entity_type) REFERENCES entity (id, type)
    ON DELETE CASCADE
);

CREATE TABLE item (
  entity_id        uuid PRIMARY KEY,
  entity_type      entity_type NOT NULL DEFAULT 'item'
                     CHECK (entity_type = 'item'),

  -- What the person last said is there. Changed by nothing except an explicit
  -- act, and never computed from history. Availability is this minus what live
  -- quantified relations commit, and is derived rather than stored.
  asserted_amount  numeric,

  -- The person's word for it, interpreted by nothing.
  unit             text,
  asserted_at      timestamptz,

  location_id      uuid,
  location_type    entity_type NOT NULL DEFAULT 'location'
                     CHECK (location_type = 'location'),

  template_id      uuid REFERENCES template (id),

  CHECK ((asserted_amount IS NULL) = (asserted_at IS NULL)),

  FOREIGN KEY (entity_id, entity_type) REFERENCES entity (id, type)
    ON DELETE CASCADE,
  FOREIGN KEY (location_id, location_type) REFERENCES entity (id, type)
);

CREATE TABLE location (
  entity_id          uuid PRIMARY KEY,
  entity_type        entity_type NOT NULL DEFAULT 'location'
                       CHECK (entity_type = 'location'),

  parent_location_id uuid,
  parent_type        entity_type NOT NULL DEFAULT 'location'
                       CHECK (parent_type = 'location'),

  template_id        uuid REFERENCES template (id),

  CHECK (parent_location_id IS NULL OR parent_location_id <> entity_id),

  FOREIGN KEY (entity_id, entity_type) REFERENCES entity (id, type)
    ON DELETE CASCADE,
  FOREIGN KEY (parent_location_id, parent_type) REFERENCES entity (id, type)
);

CREATE TABLE category (
  entity_id           uuid PRIMARY KEY,
  entity_type         entity_type NOT NULL DEFAULT 'category'
                        CHECK (entity_type = 'category'),

  parent_category_id  uuid,
  parent_type         entity_type NOT NULL DEFAULT 'category'
                        CHECK (parent_type = 'category'),

  -- Acts once, at the creation of an entity placed in this category. Not a
  -- standing rule, and never inherited.
  creation_default_audience uuid[] NOT NULL DEFAULT '{}',

  CHECK (parent_category_id IS NULL OR parent_category_id <> entity_id),

  FOREIGN KEY (entity_id, entity_type) REFERENCES entity (id, type)
    ON DELETE CASCADE,
  FOREIGN KEY (parent_category_id, parent_type) REFERENCES entity (id, type)
);

-- ---------------------------------------------------------------------------
-- Bodies
-- ---------------------------------------------------------------------------

CREATE TABLE block (
  id         uuid PRIMARY KEY,

  entity_id  uuid NOT NULL,

  -- Only two types carry a body, and the store refuses a block on any other.
  body_type  entity_type NOT NULL
               CHECK (body_type IN ('note', 'shopping_list')),

  position   integer NOT NULL,
  text       text NOT NULL,

  FOREIGN KEY (entity_id, body_type) REFERENCES entity (id, type)
    ON DELETE CASCADE,

  -- Deferrable because rewriting a body renumbers what survives, and the
  -- intermediate states of that renumbering are not violations.
  CONSTRAINT block_position_unique UNIQUE (entity_id, position)
    DEFERRABLE INITIALLY DEFERRED
);

-- Boundaries are computed from the text and identity is not. A body write
-- recomputes boundaries, matches them against the rows already here, keeps the
-- identity of what survived, and writes only what changed.

-- ---------------------------------------------------------------------------
-- Relations
-- ---------------------------------------------------------------------------

-- The declared thing a typed relation points at. A table rather than a set of
-- branches, so that exposing user declared types later is exposing what
-- already exists rather than a rewrite.
CREATE TABLE relation_type (
  id         uuid PRIMARY KEY,

  -- Renameable. Renaming changes the label and nothing the system does.
  label      text NOT NULL,

  -- Read from either end as two phrasings of one record. Named with a prefix
  -- because ASYMMETRIC is reserved in SQL.
  is_asymmetric boolean NOT NULL,

  -- Carries a quantity on the relation, which belongs to neither end.
  is_quantified boolean NOT NULL,

  shipped    boolean NOT NULL DEFAULT true
);

CREATE TABLE relation (
  id                uuid PRIMARY KEY,

  -- One record, traversable from either end. Direction is a property of it and
  -- there is no reverse row.
  from_entity_id    uuid NOT NULL REFERENCES entity (id) ON DELETE CASCADE,
  to_entity_id      uuid NOT NULL REFERENCES entity (id) ON DELETE CASCADE,

  -- Null is untyped, which is the common case. Untyped is not possibly typed.
  relation_type_id  uuid REFERENCES relation_type (id),

  -- Where the relation was formed. Belongs to the relation, not to the body,
  -- so the body stays plain text.
  anchor_entity_id  uuid,
  anchor_block_id   uuid REFERENCES block (id) ON DELETE SET NULL,
  anchor_phrase     text,

  -- Declared by the type and by no other means. Whether the type declares them
  -- is a fact in another table, so the write path refuses the mismatch.
  quantity          numeric,
  resolution        uses_resolution,

  author_member_id  uuid REFERENCES membership (id),
  created_at        timestamptz NOT NULL,

  CHECK (from_entity_id <> to_entity_id),

  -- An anchor locates into one of the two things this joins, and nothing else.
  CHECK (anchor_entity_id IS NULL
         OR anchor_entity_id IN (from_entity_id, to_entity_id)),
  CHECK (anchor_block_id IS NULL OR anchor_entity_id IS NOT NULL),
  CHECK (anchor_phrase IS NULL OR anchor_block_id IS NOT NULL),

  CHECK ((quantity IS NULL) = (resolution IS NULL))
);

-- Deduplicating untyped unanchored relations belongs to the write path, not
-- here. A unique index over the unanchored pair looks right and is not: a
-- relation that loses its anchor when its block is removed becomes unanchored,
-- collides with an existing relation between the same pair, and the block
-- removal fails. A body edit must never fail because a relation lost an
-- anchor, and that outranks refusing a duplicate structurally.

-- ---------------------------------------------------------------------------
-- Shared model side tables
-- ---------------------------------------------------------------------------

CREATE TABLE entity_date (
  entity_id     uuid NOT NULL REFERENCES entity (id) ON DELETE CASCADE,
  ordinal       integer NOT NULL,

  -- The person's word for it, read by them and interpreted by nothing. May
  -- come from a template, and is an ordinary date if it does.
  label         text NOT NULL,

  occurs_at     timestamptz NOT NULL,

  -- The person's mark, never inferred. Surfaces showing what is coming take
  -- only dates carrying it.
  bring_forward boolean NOT NULL DEFAULT false,

  PRIMARY KEY (entity_id, ordinal)
);

CREATE TABLE entity_assignment (
  entity_id  uuid NOT NULL REFERENCES entity (id) ON DELETE CASCADE,
  member_id  uuid NOT NULL REFERENCES membership (id),
  PRIMARY KEY (entity_id, member_id)
);

-- Values live on the entity, keyed by the name the property had at the moment
-- it was set. Not by a reference to the template property, because detaching a
-- template keeps every value with the names it had then.
CREATE TABLE entity_property_value (
  entity_id uuid NOT NULL REFERENCES entity (id) ON DELETE CASCADE,
  name      text NOT NULL,
  value     text,
  PRIMARY KEY (entity_id, name)
);

-- ---------------------------------------------------------------------------
-- Intents
-- ---------------------------------------------------------------------------

-- What the instance already answered. Replay is idempotent, so an intent
-- carries its own identity and resubmitting one returns the acknowledgement
-- already issued rather than producing a second effect. That promise cannot be
-- kept without the acknowledgement being held, which is what this table is.
--
-- The row is written in the same transaction as the write it acknowledges. A
-- row here and no write, or a write and no row here, would each break the
-- promise in a different direction.
--
-- Every column beyond the outcome belongs to one outcome and is null for the
-- others, which is the price of one table over three.
CREATE TABLE intent (
  id                uuid PRIMARY KEY,

  -- Who submitted it. An intent identity comes from a client, so the same
  -- value arriving from a different member is not the same intent and the
  -- stored answer is not theirs to receive.
  member_id         uuid NOT NULL REFERENCES membership (id),

  received_at       timestamptz NOT NULL,
  outcome           intent_outcome NOT NULL,

  -- Applied. Identities the intent affected, and the counter after the write
  -- where a single entity was written.
  entity_ids        uuid[],
  relation_ids      uuid[],
  counter           bigint,

  -- Applied, where a stale base overlapped what moved since. Both values were
  -- retained and the write still applied, so this is part of a success.
  conflict_content_fields  integer[],
  conflict_specific_fields integer[],
  conflict_block_ids       uuid[],

  -- Refused. The detail is for a person reading a log and is never parsed.
  refusal           refusal_reason,
  refusal_detail    text,

  -- Recreated. An edit arrived for an entity that was erased, the erasure
  -- stands, and the work came back under a new identity.
  original_entity_id uuid,
  new_entity_id      uuid,

  CHECK ((outcome = 'refused') = (refusal IS NOT NULL)),
  CHECK ((outcome = 'recreated') = (new_entity_id IS NOT NULL)),
  CHECK (outcome <> 'recreated' OR original_entity_id IS NOT NULL)
);

-- Replay looks an intent up by identity and by member together, because an
-- identity alone is a client's assertion.
CREATE INDEX intent_member_idx ON intent (member_id, received_at);

-- ---------------------------------------------------------------------------
-- The change sequence
-- ---------------------------------------------------------------------------

-- Position is allocated from this row inside the writing transaction, which
-- makes commit order and position order the same. A sequence would be faster
-- and would leave gaps: a client polling could read past a position whose
-- transaction had not yet committed and never see it. That is a partial answer
-- presented as a complete one, which is the failure the change stream exists to
-- prevent. The cost is that writes serialise on one row.
CREATE TABLE change_position (
  singleton     boolean PRIMARY KEY DEFAULT true CHECK (singleton),
  next_position bigint NOT NULL DEFAULT 1
);

INSERT INTO change_position (singleton, next_position) VALUES (true, 1);

CREATE TABLE change (
  position          bigint PRIMARY KEY,
  occurred_at       timestamptz NOT NULL,
  kind              change_kind NOT NULL,

  -- No foreign key. The sequence has to be able to speak about something that
  -- no longer exists, which is the whole reason erasure needs its own signal.
  entity_id         uuid,
  relation_id       uuid,

  -- Broadening an audience makes something appear to a member for the first
  -- time and narrowing makes it vanish, and to that member those are creation
  -- and disappearance. Assembling a member's stream needs to know who could
  -- see the entity before the write as well as after, or a narrowing produces
  -- nothing for the member who lost sight of it.
  audience_before   uuid[],
  audience_after    uuid[],
  author_before     uuid,
  author_after      uuid,

  -- Where a body moved, which blocks moved. The instance has already divided,
  -- so this is a fact it holds.
  changed_block_ids uuid[],

  CHECK (num_nonnulls(entity_id, relation_id) = 1)
);

-- ---------------------------------------------------------------------------
-- Conflict state
-- ---------------------------------------------------------------------------
--
-- Recorded on the entity. Entity local, non blocking, and uncounted: nothing
-- aggregates these into a number and there is no queue over this table.

CREATE TABLE conflict (
  id                uuid PRIMARY KEY,
  entity_id         uuid NOT NULL REFERENCES entity (id) ON DELETE CASCADE,

  -- The part is the field, except in a body, where it is the block.
  field_name        text,
  block_id          uuid REFERENCES block (id) ON DELETE CASCADE,

  mine_member_id    uuid REFERENCES membership (id),
  mine_value        text,
  theirs_member_id  uuid REFERENCES membership (id),
  theirs_value      text,

  detected_at       timestamptz NOT NULL,

  CHECK (num_nonnulls(field_name, block_id) = 1)
);

-- ---------------------------------------------------------------------------
-- Versions
-- ---------------------------------------------------------------------------
--
-- A version holds the body and only what tells one version from another in a
-- list. It holds the body flattened rather than its blocks: restore replaces
-- the body and nothing else, and the replacement is divided again on the way
-- in, so versioning block identity would retain something restore does not use.
--
-- Retention is bounded on an operator window, and a household may decline
-- version history entirely with content never at risk.

CREATE TABLE entity_version (
  id               uuid PRIMARY KEY,
  entity_id        uuid NOT NULL REFERENCES entity (id) ON DELETE CASCADE,
  captured_at      timestamptz NOT NULL,
  author_member_id uuid REFERENCES membership (id),
  body             text NOT NULL
);

-- ---------------------------------------------------------------------------
-- Event records
-- ---------------------------------------------------------------------------
--
-- Immutable facts. Appended, never updated, corrected by appending. A log
-- entry belongs to the item it is about and is not an entity of any type.
--
-- The cascade below is erasure, not deletion. Deleting an item leaves its
-- event records untouched, because the item's row stays and only its lifecycle
-- moves.

CREATE TABLE event_record (
  id               uuid PRIMARY KEY,

  item_id          uuid NOT NULL,
  item_type        entity_type NOT NULL DEFAULT 'item'
                     CHECK (item_type = 'item'),

  kind             event_kind NOT NULL,
  amount           numeric,
  occurred_at      timestamptz NOT NULL,
  recorded_at      timestamptz NOT NULL,
  author_member_id uuid REFERENCES membership (id),

  -- A correction is an appended record naming what it corrects.
  corrects_id      uuid REFERENCES event_record (id),

  FOREIGN KEY (item_id, item_type) REFERENCES entity (id, type)
    ON DELETE CASCADE
);

-- ---------------------------------------------------------------------------
-- Derived data
-- ---------------------------------------------------------------------------
--
-- Never canonical, always discardable, and rebuildable without loss. Each row
-- records what produced it, which is what makes the outstanding work set a
-- fact about the store rather than about a queue.

-- The queue is an optimisation over computing outstanding work from provenance,
-- and may be lost without loss. Claimed with SELECT ... FOR UPDATE SKIP LOCKED
-- per DR-002.
CREATE TABLE derivation_queue (
  id          bigserial PRIMARY KEY,
  entity_id   uuid NOT NULL REFERENCES entity (id) ON DELETE CASCADE,
  kind        derivation_kind NOT NULL,
  enqueued_at timestamptz NOT NULL,
  attempts    integer NOT NULL DEFAULT 0
);

-- !! The dimension below is the one value in this file that is not decided. It
-- !! is fixed by the embedding model, which is open, and changing it means re
-- !! embedding the corpus. Provenance makes that recoverable rather than fatal.
-- !! 1024 is a placeholder and is not a recommendation.
CREATE TABLE embedding (
  id           uuid PRIMARY KEY,
  entity_id    uuid NOT NULL REFERENCES entity (id) ON DELETE CASCADE,

  -- Null where the whole entity was embedded. Set where one block was. Both
  -- are expressible because which is used is retrieval design's call, and a
  -- block has no presence in retrieval apart from the body it belongs to, so
  -- either way a result is the entity.
  block_id     uuid REFERENCES block (id) ON DELETE CASCADE,

  model        text NOT NULL,
  produced_at  timestamptz NOT NULL,
  vector       vector(1024) NOT NULL
);

CREATE TABLE derived_text (
  entity_id          uuid PRIMARY KEY REFERENCES entity (id) ON DELETE CASCADE,
  content            text NOT NULL,
  producer           text NOT NULL,
  produced_at        timestamptz NOT NULL,

  -- A person's edit outranks recomputation, and a later pass must not
  -- overwrite it. Nothing turns on where the words originally came from, so
  -- an edited extraction is not a third category of content.
  edited             boolean NOT NULL DEFAULT false,
  edited_by_member_id uuid REFERENCES membership (id),
  edited_at          timestamptz
);

-- ---------------------------------------------------------------------------
-- Configuration
-- ---------------------------------------------------------------------------
--
-- Here rather than in a file beside the instance, so it is backed up with
-- everything else and reachable by the same queries. v0 runs on the values set
-- with the build, which is a supported state rather than a degraded one.

CREATE TABLE instance_config (
  singleton              boolean PRIMARY KEY DEFAULT true CHECK (singleton),

  holding_retention      interval NOT NULL,

  -- Null declines version history entirely, which never puts content at risk.
  version_retention      interval,

  -- Not a duration, because time is a proxy for volume and is wrong in both
  -- directions. Null is no horizon, and every client rebuilds, which is always
  -- available and always correct.
  --
  -- This is either null or longer than every other window above. A value in
  -- between is a correctness problem wearing the costume of a tuning choice,
  -- and the store cannot check it because the two are not in the same unit.
  change_horizon_entries bigint,

  CHECK (change_horizon_entries IS NULL OR change_horizon_entries > 0)
);

-- ---------------------------------------------------------------------------
-- Indexes
-- ---------------------------------------------------------------------------

-- Both retrieval arms and the audience predicate sit on the same table, which
-- is what makes one candidate set in one query possible rather than merely
-- convenient.
CREATE INDEX entity_search_vector_idx ON entity USING gin (search_vector);
CREATE INDEX entity_search_trgm_idx ON entity USING gin (search_text gin_trgm_ops);
CREATE INDEX entity_audience_idx ON entity USING gin (audience_member_ids);
CREATE INDEX entity_author_idx ON entity (author_member_id);

CREATE INDEX embedding_vector_idx ON embedding
  USING hnsw (vector vector_cosine_ops);

-- An order is read by container. Position is unique within one, so nothing
-- breaks a tie.
CREATE INDEX entity_category_order_idx ON entity (category_id, category_position);
CREATE INDEX arc_ladder_order_idx ON arc (campaign_id, ladder_position);
CREATE INDEX quest_arc_order_idx ON quest (arc_id, ladder_position);
CREATE INDEX quest_campaign_order_idx ON quest (campaign_id, ladder_position);

CREATE INDEX entity_type_idx ON entity (type);
CREATE INDEX entity_active_idx ON entity (id) WHERE lifecycle = 'active';
CREATE INDEX entity_holding_idx ON entity (deleted_at) WHERE lifecycle = 'deleted';
CREATE INDEX entity_category_idx ON entity (category_id);
CREATE INDEX entity_group_idx ON entity (deletion_group_id)
  WHERE deletion_group_id IS NOT NULL;

-- Traversal from either end, which is what makes a backlink a query rather
-- than a second record to maintain.
CREATE INDEX relation_from_idx ON relation (from_entity_id);
CREATE INDEX relation_to_idx ON relation (to_entity_id);
CREATE INDEX relation_anchor_block_idx ON relation (anchor_block_id);

CREATE INDEX block_entity_idx ON block (entity_id, position);

CREATE INDEX change_entity_idx ON change (entity_id);
CREATE INDEX change_relation_idx ON change (relation_id);

CREATE INDEX entity_date_forward_idx ON entity_date (occurs_at)
  WHERE bring_forward;

CREATE INDEX event_record_item_idx ON event_record (item_id, occurred_at);

CREATE INDEX derivation_queue_idx ON derivation_queue (kind, id);

CREATE INDEX embedding_entity_idx ON embedding (entity_id);
CREATE INDEX embedding_model_idx ON embedding (model);

COMMIT;

-- ---------------------------------------------------------------------------
-- What is not created here
-- ---------------------------------------------------------------------------
--
-- The rule applied: a table exists where the documents decide its shape,
-- whether or not v0 writes to it. A table whose shape is still open is not
-- guessed at.
--
--   routine        The schedule's expression is an open Guidance concern.
--                  quest.routine_id is therefore untyped and unconstrained,
--                  and gains its composite foreign key when this arrives.
--   focus_session  What it holds beyond the shared set awaits its domain.
--   check_in       The same.
--
-- Three things are decided and are not tables, which is a result rather than
-- an omission:
--
--   note           A body is its blocks in order. There is nothing else.
--   shopping_list  Entries are blocks. An entry pointing at an item does so by
--                  a relation from the list anchored at the entry's block.
--   backlinks      Derived by querying the far endpoint. Never stored.
--
-- v0 does not write to entity_version, event_record, derived_text, or
-- template, per the v0 scope. They are created because their shape is settled.
--
-- ---------------------------------------------------------------------------
-- Decisions this file embodies
-- ---------------------------------------------------------------------------
--
-- 1. One entity table plus a side table per type, rather than a table per type
--    or a document column.
-- 2. A constant type column on each side table, so a composite foreign key
--    refuses a reference to the wrong type. One extra column per typed
--    reference is the price.
-- 3. search_text maintained by the write path, with the tsvector generated
--    from it so the two cannot drift, and trigram matching over the same text.
-- 4. Audience as a uuid array on the entity row with a GIN index, rather than
--    a join table. Denormalised, and it keeps the predicate inside the query
--    that produces candidates rather than behind a join.
-- 5. Change positions allocated from a single row inside the writing
--    transaction rather than from a sequence, so a poller cannot skip a change
--    that had not yet committed. Writes serialise on that row.
-- 6. change rows carry the audience before and after the write, because a
--    narrowing has to reach the member who lost sight of the entity and the
--    current audience no longer names them.
-- 7. Erasure strips content and leaves a tombstone rather than deleting the row.
-- 8. A version holds the body flattened rather than its blocks.
-- 9. No triggers anywhere.
-- 10. One intent table holding the acknowledgement already issued, with the
--     columns of each outcome null for the others, rather than a table per
--     outcome.
--
-- ---------------------------------------------------------------------------
-- Checks this file cannot make, which the write path owes
-- ---------------------------------------------------------------------------
--
--   Audience, on both paths, on the same predicate.
--   Every member id in an audience array naming a real membership, since
--     a foreign key cannot reach inside an array.
--   Position unique within a container, and appended on entry to one.
--   Returning a held acknowledgement on replay rather than writing again, and
--     writing the intent row in the transaction it acknowledges.
--   Type immutability on an edit.
--   The counter, conflict detection, and retaining both values.
--   Canonical ordering of symmetric and untyped relations, since symmetry is a
--     property of the type declaration and a constraint cannot see it.
--   Refusing a duplicate untyped unanchored relation, for the reason recorded
--     above the relation table.
--   A relation property permitted only where its type declares it.
--   Block boundary recomputation and identity matching.
--   Bulk state transitions.
--   The horizon being null or longer than every other retention window.
--   Everything the wire shape implies: identifier length, decimal range, a
--     field both present and cleared, a cleared number naming no field.
--
-- ---------------------------------------------------------------------------
-- Gaps this pass found
-- ---------------------------------------------------------------------------
--
-- a. The embedding dimension, above.
-- b. Intent rows are retained with no stated trimming. Keeping them forever
--    grows a table nothing reads except on replay, and trimming them means a
--    replay from far enough back is answered by writing again. A create is
--    protected by the entity primary key; an edit is not.
-- c. Conflict values are stored as text whatever the part's kind is, which is
--    the proto's gap (g) landing here too.
-- d. A conflicted part is named by text here and by a field number on the wire.
--    Something has to hold that mapping, and wherever it lives it can drift.
-- e. A tombstone is retained indefinitely. Trimming it is bounded by the same
--    horizon reasoning as the change sequence, and an edit arriving from a
--    device away longer than that would read as a create rather than a
--    recreation, and would reuse an erased identity.
-- f. entity_date uses timestamptz. Whether a date is a calendar date or an
--    instant is still not stated, and expiry and renewal read as calendar
--    dates. Carried forward from the proto's gap (b).
-- g. Only categories and the ladder carry a position. Nested locations,
--    nested categories, and the schedule surface's day are containers by
--    the substrate's definition and have none, because nothing yet states
--    that their contents are hand ordered.
-- h. Nothing prevents a cycle in nested locations or categories. Self
--    reference is refused; a longer loop is not, and a recursive query would
--    not terminate on one.
-- i. v0's treatment of templates is ambiguous. The scope names Tracking's core
--    as items, nested locations and quantity, which reads as an enumeration,
--    while also saying anything not named as deferred is in force, and
--    templates are not named as deferred. Created here on the second reading.

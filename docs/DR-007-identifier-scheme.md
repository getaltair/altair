# DR-007: Identifiers are random UUIDs

**Status:** Accepted
**Date:** 2026-08-16
**Supersedes:** nothing
**Related:** Altair Architecture Foundations, Altair Component Model, DR-002, DR-004

---

## Context

The foundations set three constraints on identity and name no scheme that satisfies them. An identifier exists from the moment a thing is created, before anything else has seen it, and that is load-bearing. Identifiers do not collide across devices, because two clients may create entities offline at the same moment with no way to coordinate, and that is load-bearing too. Identity is stable and opaque, and does not encode where or when something was made.

Two later decisions fixed the shape without fixing the scheme. DR-004 makes every identifier sixteen bytes on the wire. DR-002 types them `uuid` in the store. What generates the value is a property of the whole system rather than of either boundary, which is why it is recorded here rather than inside one of them.

---

## Decision

**Every identifier is a random UUID, version 4 under RFC 9562.**

1. **The scheme is uniform.** Households, memberships, entities, blocks, relations, relation types, templates, conflicts, versions, event records, embeddings, and file bodies all carry a version 4 UUID. The derivation queue's row number is the one integer key in the store, and it identifies a unit of pending work rather than a thing, which is why it is not covered here.
2. **Generation belongs to whatever brings the thing into existence**, per the foundations. A client generates for entities and relations. The instance generates for blocks.
3. **Uniqueness rests on entropy and is enforced by the primary key.** Nothing coordinates, allocates, or reserves. This is what makes creation with nothing reachable possible, and it is the reason the scheme has to be random rather than derived from anything a device holds.
4. **Nothing reads an identifier.** No value is inferred from it, no query groups by it, and no order taken over it means anything. Ordering within a container is a position held by the containment, and nowhere in the system does the order of two identifiers decide anything.
5. **Sixteen bytes on the wire, `uuid` in the store.** Neither is new. Both are named here so that the scheme, the width, and the column type are written down in one place.

---

## Alternatives considered

### UUIDv7

**Rejected, and the only alternative with a real argument behind it.** Its leading forty-eight bits are a millisecond timestamp, and the foundations state that identity does not encode when something was made. That alone decides it, and the rest is worth recording because the benefit is quoted often enough to be raised again.

The benefit is index locality: recent inserts concentrate in a narrow region of the index rather than scattering across it. This system is close to the worst case for collecting it. Identifiers are generated on devices that are offline, whose clocks skew, and the ordering section already states that no device's clock settles anything. Arrival order is not creation order, so a batch replayed from an outbox after a week away lands in the middle of the index rather than at its end. The locality that v7 buys is proportional to how closely insert order tracks generation order, and here they are decoupled by design.

RFC 9562 recommends v7 in place of v1 and v6. It does not recommend it in place of v4, and the choice between random and time-ordered is left to the application.

The remaining objection is that reopening this on index locality grounds is a storage engine's concern deciding a system-wide property of identity, which inverts the order the foundations set. The store-local remedy, and the observable condition that would justify reaching for it, are recorded in DR-002.

### ULID and other lexicographically sortable identifiers

**Rejected on the same encoding objection**, since sortability here means a timestamp prefix. They are also not UUIDs, so the store's `uuid` column and the wire's sixteen bytes would become a bespoke type carrying its own parsing and its own validation on every boundary.

### Instance-assigned sequential identifiers

**Rejected against the load-bearing requirement** that an identifier exists from the moment a thing is created. A client capturing with nothing reachable cannot ask for one, and capture never fails.

### A device prefix with a local counter

**Rejected.** It satisfies non-collision without coordination, which is why it is worth naming. It encodes where something was made, which the foundations refuse, and it leaves every device that ever captured permanently legible in the identifiers it produced, including after that device is gone.

### Content-derived identifiers, whether UUIDv3, UUIDv5, or a content hash

**Rejected on two counts.** Identity is stable and content is not, so an edit would change identity. And two people capturing the same words on two devices are two things, while a content-derived scheme would collapse them into one.

### UUIDv1 and UUIDv6

**Rejected.** Both embed a node field, historically a MAC address, which encodes where. v6 is v1 reordered for locality and keeps that field, and the standard describes it as a path for systems already holding v1 data rather than as a choice for new ones.

---

## Consequences

**Gained**

- Creation with nothing reachable needs no allocation, no reservation, and no round trip
- An identifier that escapes into an export, a copied link, or a log reveals nothing about when or where it was made
- The wire representation and the stored representation are the same value

**Given up**

- Insert locality on every table keyed by identity. Recorded, with its reopening condition, in DR-002
- Identifiers carry no order, so anything needing one uses the change position or the arrangement key, both of which already exist for that purpose

**Obligations this creates**

- **Generation uses a cryptographically strong random source.** Non-collision across devices is the whole load-bearing requirement, and two devices of the same model seeding a weak generator the same way is precisely how it fails.
- **No code infers structure from an identifier.** Sorting by identifier produces an arbitrary order, and an arbitrary order must never be presented to a person as a stable one.

---

## Deliberately not decided here

- How an identifier is presented on a surface a person reads, which is a client concern
- The arrangement key's generation rule, which is open. It is opaque bytes of no fixed width rather than an identity, so it is not a UUID and nothing here constrains it
- Whether the identifier a person types or pastes to reach an entity is the identifier itself

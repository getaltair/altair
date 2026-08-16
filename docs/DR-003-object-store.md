# DR-003: The object store is the filesystem behind a four-operation interface

**Status:** Accepted
**Date:** 2026-08-15
**Supersedes:** nothing
**Related:** Altair Component Model, Altair Architecture Foundations, Altair v0 Scope, DR-002

---

## Context

The object store holds file bodies and nothing else. It is forbidden from holding anything the structured store holds, and it cannot participate in a transaction with it, which is why the ordering rules exist: bytes before the record that points at them on creation, the record before the bytes on erasure.

**Nothing in the requirements asks for object storage semantics.** No content addressing, no multipart uploads, no lifecycle policies, no replication. What the component model asks of this boundary is four things: write bytes, read a body on request, remove bytes belonging to an erased entity, and enumerate so reclamation can find bytes no record points at.

**The absence behaviour is already mild.** When the object store is gone, no file entity is committed and every other kind of capture is unaffected on the write path; on the read path the entity, its title, its relations, and its derived text remain available and only the body is currently unavailable, which is a different statement from missing.

---

## Decision

**The filesystem, reached through an interface of exactly four operations: put, get, delete, and enumerate.**

1. **The interface is the decision.** It is small enough that replacing what sits behind it is a contained piece of work rather than a migration, which is what makes deferring the product choice safe rather than merely postponed.
2. **No S3-compatible server is deployed for v0.** What one buys is the ability to move bytes onto separate hardware, and nothing currently needs that. The instance is one authority and file bodies sit beside it.
3. **Reclamation enumerates.** This is why enumerate is in the interface rather than an afterthought: erasure removes the record before the bytes, so the sweep for unreferenced bytes is load-bearing rather than housekeeping, and any later replacement must support it.

---

## Alternatives considered

### MinIO

**Rejected, and no longer a candidate on any terms.** The community edition entered maintenance mode in December 2025 and the repository was archived in April 2026. Whatever its technical merits, a frozen upstream is the wrong foundation for the one component that holds bytes nothing else can reconstruct.

### An S3-compatible server for v0

**Rejected as premature rather than wrong.** Garage is the strongest candidate when this is needed: a single binary with a conservative release pace, and its AGPL licence is Altair's own, so it raises nothing. RustFS is the closest replacement for MinIO's shape and ships under Apache 2.0, but is alpha with an active stream of advisories, which is a poor place to put file bodies. Neither is chosen now, because deploying either in v0 buys a capability nothing uses and costs a moving part.

### File bodies in the structured store

**Rejected.** The component model states that the object store holds bodies and nothing else, and that the structured store holds everything else, which leaves no second candidate for either. Folding bodies into the database would remove the ordering rules by removing the boundary, and it would put large immutable blobs in the one component whose absence means the instance is down.

---

## Consequences

**Gained**

- One fewer service to run, and file bodies land on ordinary storage that is backed up the way everything else on the host is
- The ordering rules are exercised from the first release, so the boundary is real rather than theoretical

**Given up**

- Bytes cannot be moved onto separate hardware without implementing the interface against something else first

**Obligations this creates**

- **Nothing outside the interface may touch the bytes.** The moment something reads a path directly, the four operations stop being the whole boundary and the replacement stops being contained.
- **Enumerate must stay cheap enough to run on a schedule**, since reclamation depends on it and reclamation is what closes the erasure window.

---

## Deliberately not decided here

- Which S3-compatible product replaces the filesystem when bytes need to move, and when that becomes necessary
- Layout beneath the interface, which is behind it and therefore not a decision at this level

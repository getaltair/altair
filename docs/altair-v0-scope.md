# Altair v0 Scope

**Status:** Draft
**Date:** 2026-08-13
**Governed by:** Altair Vision & Scope
**Related:** Altair Substrate Specification, Altair Guidance PRD, Altair Knowledge PRD, Altair Tracking PRD, Altair Architecture Foundations, Altair Component Model

---

## What this document is

The scope of the first running build. It is a sequencing document of the kind the vision deliberately leaves out of itself, so it sits below the vision and every normative document: it defers commitments and amends none. Where this document narrows a Must, the Must stands and the narrowing is temporary. Anything not named as deferred here is in force.

**The entity model completes before this scope hardens.** Even a thin build needs to know what an entity is, and design before architecture before tools still holds. v0 is a smaller release, not a licence to skip the order.

---

## What v0 is for

Three things, in order.

**It supports its author daily.** The need the product exists for is present tense, and a system that supports one person now is worth more than a complete one later. This is progress over perfection applied to the project itself.

**It validates the bet the vision names.** One place where anything connects to anything, searchable in a single pass, is worth more than depth in any single domain. A v0 that cannot test that claim is not a v0 of Altair.

**It produces a real corpus under real use.** Retrieval quality and surfacing aggressiveness are settled by tuning, and tuning needs a running system holding real material. v0 is what creates that material.

---

## The commitment that does not narrow

**All three domains, connected.** The domains exist to support and reinforce one another, so the connection is what gets validated and depth within each domain is what gets sacrificed. Cross-domain relations and cross-domain search are in scope from the first running build, because they are the load-bearing claim.

---

## Domain cores

**Guidance** is the ladder: campaigns, arcs, and quests, with their states. Deferred: routines, focus sessions, daily check-ins.

**Knowledge** is notes, links formed where the writing happens, and derived backlinks. Files are entities with a title and relations, because a photo taken on a phone is a capture like any other. Deferred: versions, text extraction from files.

**Tracking** is items, nested locations, and quantity at whatever precision the person chose, where "just mark it lower" works from the first day. Deferred: consumption and purchase logs, low-stock thresholds, expiry, barcode capture, shopping lists.

Shopping lists are out deliberately rather than incidentally. Their entry model leans on an anchor granularity the substrate does not yet define, and a first release should not be the thing that forces that question.

---

## The cross-cutting spine

- **Relations.** Untyped as the common case, plus the current typed set: Blocks, Uses with its quantity, References.
- **Search crosses all three domains in one pass, literal and semantic together.** Both arms are in v0. Semantic is the last part built, because the corpus and the derivation that feeds it have to exist first, and until it lands literal answers alone and the answer says so. Surfacing is the proactive behaviour rather than the retrieval underneath it, and it is deferred.
- **The capture fast path.** Never stops to ask, and partial and imprecise data is always valid.

---

## Infrastructure

**One self-hosted instance**, the same operating model Lattice runs today. Access control is the operator's gateway, which the vision already places outside the product's commitments; the product ships no identity system.

**Thin clients, and none of them a browser.** A terminal client is the deliberate surface, where the full editing work happens. Capture away from that surface arrives through a message bridge: the person messages themselves, and the bridge submits what they wrote as an ordinary client of the public interface.

**Lookup away from a desk is deferred**, and is the one thing this client set does not cover. The phone's two paths are write a thought down and look a thing up, and v0 answers only the first. The second waits for a phone client rather than being solved another way.

**A bridge is an ordinary client.** It holds nothing the interface does not already carry, submits intents like any other client, and authenticates as the person it captures for. Three things it owes: it accepts only from that person, since anyone who can message them could otherwise write to their instance; it tracks its own position and treats a gap as a fault, because the transport's own acknowledgement says the transport delivered a message and not that the instance holds it; and it does not answer queries, because sending household material back out through a third party is a decision that has not been taken.

**Capture on a client is an append-only outbox.** A capture is written locally, survives app closure and device restart, and replays to the instance when it can. It holds whenever the replay path is closed, and an unreachable instance and an expired session are the same wait. It never drops a capture and never errors one away. This is a queue, not a sync engine: newly created entities cannot conflict with anything, so there is no merge story.

**Queue depth is silent, and faults signal.** The outbox holding is waiting, and no badge, count, or banner reflects how much is in it. An unreachable instance and an expired session are the same wait. What will not clear by continuing to run is a fault: an intent the instance refused is signalled rather than left in the quiet pile.

**No sync engine, no conflict model, no replicas.**

---

## What is absent in v0

The component model states what each side of every boundary owes when the other is not there. v0 stands up fewer components than the full system, and each absence below is one that model already covers, so none of them is a special case and none needs a rule of its own.

**Inference is present.** Semantic retrieval is part of v0, so a bi-encoder runs on hardware the household controls. Inference is several models, each independently absent, so an instance without a cross-encoder is conforming rather than degraded and v0 does not require one.

**The derivation worker is present**, producing embeddings and the record of what produced each. Text extraction from files stays deferred, which narrows what the worker does rather than whether it is there.

**Both are absent for the part of v0's construction that precedes them**, since semantic retrieval is the last part built and the corpus and the writing paths that feed it come first. That interval needs no rule of its own. Literal answers alone and the answer says so, a new entity is findable by its words the moment it lands, and what is outstanding is a fact about the store rather than about a worker, so the first pass is computed when the worker arrives and accumulates nowhere meanwhile.

**Notification delivery is absent by choice.** No transport is configured. Nothing is delivered, which costs notifications and nothing else, because no path depends on one arriving. An undelivered notification is dropped rather than queued, so nothing is owed when a transport is configured later.

**Reclamation is present, and it is not optional.** Its delivery half has nothing to send, but erasing a file removes the record before the bytes, and the bytes go on a later pass. An instance that never reclaims holds erased bytes indefinitely, which is the one absence here with a cost beyond storage.

**No browser client exists.** Every client is a program the person installs or a bridge the operator runs. This costs reaching the instance from a device that has nothing installed, which the browser gave for free, and it is given up deliberately rather than by omission.

**Lookup from a phone is absent**, per the client set above. Capture from a phone is not, because the bridge covers it.

**The operator plane is not required.** Most of what it configures is absent already: no transport, no models, and no derived data to rebuild. What remains is the retention windows and the horizon, and both hold safe values set with the build rather than chosen at runtime, which is why no surface is needed to keep them correct. The horizon in particular is either nothing or longer than every other retention window, and a constant cannot drift into the middle. An instance running on the configuration it holds is a supported state rather than a degraded one.

If the plane is not built, no counts are surfaced anywhere in v0. This follows the rule rather than losing anything to it: depth was never for a daily surface, and finding out that something is wrong does not depend on the plane, because faults signal where the person already is.

**The household is not absent.** It is not a component, and single-user means the audience predicate is constant rather than missing. Both paths still enforce it, on the same predicate, so nothing has to be added to them when the household arrives.

**The public interface is not absent either.** It is a boundary the core owns, so it cannot be. What v0 defers is its definition, and what crosses it is already enumerated in both directions, which is what makes deferring the definition safe rather than a decision postponed.

---

## Deferred Musts

Named so that v0's shape is never mistaken for an amendment.

- **Sync & integrity.** Divergence, block-level granularity, and conflict surfacing are moot with one instance and no replicas. They return with multi-device.
- **Household & privacy.** The entire section. v0 is single-user.
- **Surfacing.** The proactive behaviour only. Retrieval is in v0, including its semantic arm, and what is deferred is material appearing where the person is working without being asked for. Findable without recall is not deferred as machinery and is not claimed as met either: whether it holds is empirical, and it is reached by tuning against the corpus v0 exists to produce.
- **Every capability reachable through the public interface.** Not binding on v0. The constraint carried forward is that nothing in v0 may foreclose it.
- **Complete export and the interchange format.** Consistent with the existing decision: the operator already holds the store, and the constraint carried forward is that no choice forecloses export later.

**Not deferred:** capture never fails, a captured thing is never lost, partial data is always valid, capture never stops to ask, AGPL, self-hosted, no required third-party service, no coercion mechanics, not a feed, nothing rearranges itself.

---

## Rejected while scoping

Recorded here rather than in the scratchpad, because these are decisions this document acts on, not open matter.

**A single-domain v0.** It validates the least doubtful thing. Each domain alone has strong dedicated competitors, and the product is the intentional combination, not a ranking within it. A one-domain v0 tests whether a worse dedicated tool can be built, which was never in doubt.

**Single device as the v0 cut.** The cut it was reaching for was the sync engine, and single device was the wrong name for it. One authoritative instance with thin clients produces no divergence and nothing to merge, and it keeps capture anywhere and lookup anywhere, which are the point of the product rather than a cost to trim.

**Deferring the no-required-third-party-service Must.** Raised on the thought that building v0 on a hosted product the operator already pays for would require it, and withdrawn on rereading: the Must governs what the project ships, not what the operator runs an instance on. Considering a hosted product as the short path to v0 is not the same as shipping a required service. The Must holds through v0.

---

## When v0 has done its job

When it is the place the author's captures land by default, across all three domains, and the connections between them are being used rather than admired.

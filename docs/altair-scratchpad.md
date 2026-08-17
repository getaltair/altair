# Altair Scratchpad

**Status:** Non-normative. Nothing here is decided.
**Last touched:** 2026-08-17

---

## What this is

A holding area for things that have been discussed, sometimes at length, but that do not belong in a normative document yet. Usually because they are mechanism rather than behaviour, presentation rather than methodology, or genuinely unresolved.

**This file has no authority.** If something here contradicts the vision document, a decision record, or a spec, those win. Being written down here is not a decision, and "we talked about it" is not the same as "we decided it."

Every entry carries:

- **Status:** `open` (no leaning), `leaning` (a preference emerged, untested), or `parked` (deliberately deferred, revisit trigger noted)
- **Destination:** where it should eventually be recorded, if it is decided

An entry leaves this file by being written somewhere normative, or by being explicitly dropped. Entries that sit here for a long time without either happening are a signal that nobody actually needs the decision.

**[Rejected](#rejected) works differently.** Entries there are not waiting to move anywhere. They exist so that "why not just do X?" has an answer other than re-arguing it.

---

---

## Storage and instance health

### Surfacing storage pressure

**Status:** leaning
**Destination:** administration surfaces spec

An instance approaching full storage cannot honour *capture never fails*, so it has to be able to say so. The distinction that makes this permissible: a nag tells the user about discretionary work they should do, while an instance health warning describes the machine. The first is prohibited, the second is not.

Properties that seemed right:

- Warn before capture is at risk, not at the point of failure
- About the instance, never about the person. Bytes and headroom, not "you are keeping too much"
- Reflects a condition, so it clears itself when resolved. Never becomes a task waiting to be dismissed

**Complication:** observability is unreliable and cannot be assumed. A sandboxed client often cannot see device storage at all. A server can be under a container quota, on thin-provisioned or network-backed storage, or on a backend with no free-space concept, and can report a number that is absent or simply wrong.

This is why it was cut from the substrate spec: it turned into infrastructure specifics that would date quickly.

### Retention window for deleted content

**Status:** open
**Destination:** administration surfaces spec, or a decision record

The substrate spec requires a holding state and permits scheduled cleanup on a stated window. It does not say how long, and deliberately so. Open: a shipped default, whether it varies by entity type, and whether large files get a shorter window than small notes.

Related and undiscussed: whether the window is expressed in time, in space, or in both.

## Capture edge cases

### Device binding versus session credentials

**Status:** leaning
**Destination:** client implementation guidance, or a decision record

Cut from the substrate spec as mechanism. The observation still holds: which member a device belongs to, which household, and what the audience defaults are, are not secrets, and storing them alongside the credential means an expired token or a forced logout looks like an unbound device.

Keeping binding separate from credentials means credential loss blocks transmission without blocking capture. It does not help after a full storage wipe, which clears everything.

## Capture guarantees and erasure

Worked through 2026-08-06 while opening the foundational architecture questions. All of it is behaviour, so it landed in the substrate spec rather than in an architecture document.

### Discarding a household member's edit without seeing it was theirs

**Status:** parked, with a trigger

The gap: A resolves a conflict, picks their own value, and B's text is gone without B knowing it existed. Attribution while the conflict is live mitigates this, since A can see the other value is B's, but nothing stops A choosing anyway.

**Not solved now, because frequency is unknown.** Whether this is a real problem depends on how often two members edit the same field concurrently, which nothing can currently estimate. Designing a protection for it now would be building against a guess.

**The candidate mitigation is version history**, where the discarded value survives as superseded content. It is partial: versions apply where the domain calls for them, which at present means Knowledge only, and a household may decline them entirely. So it covers the most likely case and is not a general answer.

**Revisit when:** the system has been in multi-member use long enough to say whether concurrent same-field editing happens at all.

**Worth recording, because it will be raised as an argument:** convergent replicated types make this question disappear, since nobody resolves anything. That is a real simplification and its price is the thing the vision document chose deliberately, which is that two people's edits to the same field are not silently reconciled by a machine applying a rule. The awkwardness here is the cost of that choice rather than a flaw in it.

## Architecture

### Rejected consistency models

**Convergent replicated types.** Recorded above under conflict granularity, with the three costs. Block granularity recovered most of what made them attractive.

**An event log as the store's source of truth.** The attraction is real: the change stream becomes nearly free and version history a natural consequence. Rejected because current content would depend on its own history in order to exist, contradicting the commitment that a household may decline version history without content ever being at risk. This is the second time that commitment has constrained a storage instinct. At the boundary a log is fine, since full reconciliation always exists.

**Provisional local acceptance**, where a client holds a write pending a verdict. Rejected because a write awaiting approval is not an accepted write, and capture succeeding means a created entity will reach the instance.

Gating questions taken 2026-08-06, before drafting. Recorded because three of them changed shape under pushback and the reasoning is what stops them being relitigated.

### Gaps the component assembly found

**Status:** open, all of them, and flagged in place in the component model

1. **Nothing in the architecture acts on the passage of time.** Recurrence spawns occurrences roughly a week ahead, scheduled cleanup empties the holding state, the horizon trims what the instance can answer from, and a date the person marked to come forward has to come forward. None of that is a client submitting an intent, and none of it is derivation, whose output is rebuildable while an occurrence is a canonical entity. The proposal in the model is a component that submits through the write path the way a client does, which keeps three paths intact rather than making a fourth. Placement and name are open. Amends the system architecture.

2. **The outbound queue is silent in the substrate and legible in the architecture.** The substrate requires no badge, count, or banner reflecting queue depth. The architecture's write path attributes to the substrate that queue state is visible and a stuck queue is legible rather than silent, and Operations lists queue depth among visible instance health. The reconciling reading is that depth is silent and stuckness is legible, and that instance health covers the instance's own queues rather than a client's outbox, but the documents do not say it. One of the two needs amending.

3. **Audience on the write path is unspecified.** Enforcement is stated for every query surface. A write naming an entity that already exists is not a query surface, and clients are not trusted components, so whether the write path checks that the submitting member can see the entity is undecided. The consequence of not checking is bounded, since a guessed identity buys a write and never a read, and a retained conflict is visible only to those who can already see the entity. Bounded is not the same as intended.

4. **The outbox carries bytes, not only entity records.** A photo taken on a phone is a capture like any other, so acceptance has to be durable including the body, before anything is transmitted. The substrate's queue requirements describe writes and entities and do not say this. The instance-side ordering rule for the two stores is unaffected.

5. **Erasure across the two stores has no stated ordering.** Creation is specified: bytes first, then the record, because an orphan is sweepable and a record pointing at nothing is a broken entity. Erasure inverts the risk, since removing the record first leaves bytes that erasure was supposed to remove and removing the bytes first leaves a live record pointing at nothing. Whichever way it goes, it depends on a sweep, which is more load on the scheduled work component.

6. **Reportable outstanding derivation implies the store knows what is outstanding.** Derived content records what produced it, so what is missing or stale is computable from the store. A queue that is the only record of outstanding work cannot satisfy the reporting requirement if it is lost, and derived data being rebuildable means losing it is expected rather than exceptional. Stating the queue as an optimisation over a store-derived work set closes this; it has not been stated.

7. **Whether operator plane capabilities fall under the interface Must.** No capability may exist only inside a client. The operator plane is a first-party surface, configuration is a capability, and the plane is deliberately not the daily surface. Either configuration is reachable through the public interface, or the Must is about the product's capabilities and not the instance's administration, and the documents do not distinguish.

8. **The public interface is a boundary the core owns, not a component.** The deployment diagram draws it as a box between clients and core, and the component text says the core owns it. Drawing it as a component implies something that can be absent or replaced, and it cannot be. Minor, and resolved in the model by naming it as a boundary.

## Retrieval

### Cross-domain relevance comparability

**Status:** open
**Destination:** substrate spec or a retrieval spec, once there is something to test against

The substrate spec requires that one query reach every type. It deliberately says nothing about whether results from different domains can be ranked against each other.

The question: if a note and an inventory item both match a query, does their relative order mean anything? Merged ranking requires relevance to be comparable across genuinely different kinds of content, which may not be achievable and may not even be desirable. Domain-grouped results sidestep it entirely.

Both have legitimate uses. Looking up something half-remembered wants a merged list. Looking at everything related to a particular quest may want Guidance, Knowledge, and Tracking side by side.

This was briefly written into the substrate spec as a requirement for comparable ranking, and removed, because it forbade the side-by-side view and asserted an answer to a question nobody has tested yet.

**Revisit when:** retrieval is real enough to try both against actual data.

**A candidate answer, noted 2026-08-06.** Rank-based fusion combines ranked lists using position only, with no score normalisation and no weights. That removes the comparability problem rather than solving it: a note and an inventory item never need comparable scores, only ranks within their own lists. It applies to fusing domains as readily as to fusing literal and semantic results. Still requires testing against real data, since it answers whether a merged list can be produced and not whether one is what a person wants.

### Query expansion as a retrieval stage

**Status:** parked, with a hypothesis and a test
**Destination:** an architecture or retrieval document, if it earns a place

The idea: before searching, run the person's query through a generative model to produce alternative phrasings, a hypothetical document that would answer it, or a keyword set, then search on all of those in parallel and fuse the results.

**The hypothesis, which is what makes it worth testing here rather than because other systems do it.** Vocabulary consistency is close to absent in the audience this is built for. Whether something was called programming, development, or embedded is exactly the recall the findable-without-recall Must says a person cannot be asked for, and that failure lands on the query side rather than in the index. Rewording the query is a direct attack on it.

**The test has to run after fusion is in place, not before.** Rank fusion over several retrievers already recovers matches any one of them misses, and a bi-encoder already tolerates paraphrase. Measured against literal search alone, expansion would show a large gain that fusion would also have delivered at none of the cost. The comparison that means anything is fused retrieval with expansion against fused retrieval without it.

**The case where it should win, if it wins anywhere:** a query that describes a thing rather than reusing words from it. "That thing about the boiler pressure" contains no term the note contains. That is a real shape of query for these users and it is what a hypothetical-document stage is actually for. A narrow win there is still a win and is a much smaller claim than the general one.

**The hard constraint, which applies from the first experiment.** Ranking must be deterministic: the same query over the same data produces the same order, every time, for everyone. A generative stage satisfies that only if pinned. An experiment run under sampling that shows a gain and then loses it when pinned has produced nothing usable, so pin it from the start.

**Which AI class it falls in, recorded because it will be argued later.** The retrieval class only reveals and never invents, with output being the person's own entities rather than new text. Expansion generates text that is never shown and only steers the search, and every result is still the person's own material. That reads as inside the retrieval class rather than as generation. If it were treated as generation it would be opt-in and off by default, which makes the stage pointless, since nobody opts into a search improvement they cannot see.

**Not written into any normative document**, deliberately. It is a candidate stage, not a commitment.

### Acting on a surfaced result directly

**Status:** leaning
**Destination:** a retrieval and surfacing spec, if one is written, or client implementation guidance

Anything surfaced can be acted on where it appears, without going and finding it again first. Opening it, linking it to what is being worked on, or correcting the count on a tracked item are all the same case. If a surfaced result is only a pointer that sends the person elsewhere to do anything with it, the cost of acting is high enough that people will not, and surfacing decays into a reminder that something exists.

This is not substrate, which is why it did not go into the substrate spec alongside the rest of the retrieval work. It is an obligation on clients and on the public interface.

**The platform caveat fits an existing pattern.** The substrate spec already sets a floor every client meets with anything above it a platform decision, and the vision document already requires every capability to be reachable through the public interface even where a given client does not offer it. So acting on a surfaced result is owed by the interface always, and by a particular client where the platform makes it reasonable. A constrained client that only opens the thing is conforming. What the same pattern also says is that a client offering it owes the usual guarantees, so a concession that half-forms a relation is not permitted merely because it was a concession.

### Availability for the duration of a piece of work

**Status:** open
**Destination:** unclear, since it is a session concept and the substrate has none

Surfacing during composition is not one retrieval, it is a sequence of them against a context that grows while the person is still inside it. The valuable match can arrive at any point in that sequence. The duplicate case is the clearest: another quest that is the same work may only match once enough has been written, which could be the last thing typed before saving.

Two things came out of that and neither is settled:

- **Surfacing does not wait for completeness.** It runs on partial context, both because partial data is the expected state and because a match arriving only once the writing is finished has already failed to prevent the duplicate.
- **Something that surfaced during a piece of work does not become unavailable because the work continued.** Not that it stays in the same place, and not that it stays on screen. Bounded display and continued availability are separate obligations.

**Complication.** Unbounded accumulation is a harm of its own. An ever lengthening panel is a parsing cost at exactly the moment attention is scarce, so a rule prohibiting removal is not available. What survives is weaker: continuing to work must not make something unrecoverable.

**Partly dissolved by elevation.** A person who sees something surfaced and links it has converted it into something durable, using a mechanism that already exists and costs nothing. That removes much of the weight from the persistence question, since the one result that mattered can be made permanent by the person rather than by the system.

Related and undiscussed: whether reordering counts as removal. Nothing is lost, but a person who located something by position has to find it again, and for these users that is not a small cost.

### Learned association between entities

**Status:** open, and it is an amendment to a Won't rather than a gap
**Destination:** decision record, since the vision document has to change for it to be permitted

The idea: "when you look at this, you usually also look at that" is useful material for someone who has trouble forming habits, and treating all learning as prohibited may be taking the rule past what it was protecting.

**As the documents stand this is excluded.** The vision document rules out ordering that adapts to the person so that two members see different orders for the same data. The substrate spec names learning from what was opened specifically. Reopening it is legitimate; doing so without amending those is not.

Two arguments against it, and only one is strong.

- **Weak: multi-member divergence.** In a single-person household there is no divergence, and the household is the maximum scope anyway.
- **Strong: the prohibition descends from re-entry, not from privacy.** Predictability is what makes returning cheap. A system that has been learning throughout a three week absence has rearranged itself while the person was away, and muscle memory is the one asset a returning user still has.

**What might survive the strong argument.** A learned signal that only ever adds candidates and never reorders what was already there moves nothing the person remembers. That is narrower than the idea as stated and it may be defensible. Over-tuning and anything that pushes or enforces are real harms too, but they are downstream of this one.

## Guidance

### Focus sessions and check-ins

**Status:** parked
**Destination:** the Guidance PRD, when it is revisited

Both are Shoulds in the vision document, so deferring them costs nothing there. The reason for parking is that the information needed to design them does not exist yet: neither has been used for long in any tool, which means anything decided now is a guess about what a check-in is for.

**Revisit when:** after MVP, and specifically once there has been enough sustained use to say what a check-in would have been for rather than what it might be for. Two triggers rather than one, because a date alone would not produce the missing information and the second alone is close to unfalsifiable.

**What stays true meanwhile.** The substrate settles that both are records: content is immutable, corrections are appended, and they remain entities with identity, audience, and relations, so a focus session can be related to the quest worked on during it. Deferring the design does not disturb that.

**A dependency worth watching.** If the today surface or energy filtering turns out to want a signal only a check-in produces, that is a reason to revisit rather than a reason to build a partial version to feed it.

### Relation types: shipped behaviour, user labels

**Status:** leaning
**Destination:** the Guidance PRD for the type set itself; a decision record if the extension point is taken up

Typing earns its place where the type changes what the system can do. Blocking does: a warning when completing something still blocked, which warns rather than prevents. A relation that nothing acts on is decoration.

**Where it landed.** A small shipped set of types with real behaviour, and users may rename them. Renaming satisfies most of what people want from custom types without producing a configuration surface, and it is the same answer quest states landed on, which suggests a general pattern rather than something specific to relations.

**The framing that resolved the apparent conflict with the vision document.** The concern is not extension, it is a product that requires assembly before it works. The test is the default, not the ceiling: the thing works fully untouched, and the extension point stays invisible until someone goes looking. A relation type nobody adds costs nothing; a schema nobody fills in is an empty product. The vision document currently states this more absolutely than that, and whether its wording is amended is a separate question, since it is a Won't.

**A user-added type can only inherit behaviour the system already implements.** Declaring a type asymmetric gets the two-ended reading; declaring it quantified gets a number. It cannot invent behaviour, so a custom type will always do less than a shipped one, and the interface should not obscure that.

**The constraint worth honouring now.** Build the shipped set as declarations the system interprets, not as hardcoded branches. If it is branches, an extension point is a rewrite; if it is declarations, exposing one is exposing what already exists. This is a constraint on how the shipped set is built, the same shape as the do-not-foreclose-export obligation in DR-001, and not a commitment to build anything.

**Unresolved and larger than it looks:** relations carrying properties. "Uses three of that item" needs a quantity on the relation, and the substrate currently gives relations a type and an optional direction and nothing else. This affects Tracking as much as Guidance.

### An exposed extension point for custom behaviour

**Status:** parked
**Destination:** a decision record, and it would have to reopen an existing rejection

The longer-term goal: users write code for a custom type so it can do things the shipped declarations do not cover, which also lowers the barrier to community contribution without affecting the everyday user.

**It collides with an existing rejection and must reopen it explicitly.** An in-app import plugin runtime was rejected on three grounds: a permanent API surface, a trust boundary around third-party code with full data access, and an ongoing compatibility burden. A user writing code for their own instance avoids the trust problem. The API surface and compatibility burden are unchanged, and once community contribution is the point, the trust boundary returns.

The replacement pattern used there, standalone tools outside the application, does not transfer cleanly, because relation behaviour has to run where the data is.

**Revisit when:** there is a community substantial enough for contribution to be the point rather than a hypothetical.

### Naming the schedule surface

**Status:** open
**Destination:** the Guidance PRD

"Today" implies a one-day window whose contents change at midnight, which for these users has the same shape as something resetting while they were away. A name that does not promise a day, or a window the person can widen, avoids committing to that before there is any experience of it. Cheap now, awkward later, and it gets decided by the label if nobody decides it deliberately.

### Naming the recurrence concept

**Status:** open
**Destination:** the Guidance PRD

"Routine" carries habit-tracker connotations, which sit adjacent to the streak and reward mechanics the vision document excludes outright. The concept is closer to recurring household work, mowing the lawn and putting the bins out, than to habit formation.

"Recurring" and "Repeats" are accurate and read as descriptions rather than nouns, which makes them sit oddly beside Campaign, Arc, and Quest. Circuit, Rounds, and Patrol were raised and none was convincing. Accuracy is worth more than theme here, since a full themed vocabulary was already rejected for breaking at the leaf level.

Cheap to change later, which is the reason for leaving it open rather than settling it under pressure.

## Tracking

### Member management is an open area

**Status:** open
**Destination:** its own section, probably in the substrate spec, or its own document if it grows

Nothing anywhere says how a member joins, who admits them, whether removal and departure are different acts, or what a departing person's material is. Four mechanisms reference a member and none of this exists.

**What is settled and is not part of this:** a member is not an entity, references survive departure, administration is takeable and never automatic.

**The questions.**

- Who admits a member. The leaning is an administrator, since admission has consequences for everyone already inside.
- Whether removal and departure are the same act. The leaning is not: leaving is yours, being removed is done to you, and they want different answers about material.
- What "their own material" means when someone leaves with it. The vision document promises this and never defines it. Authored-by is the only clean line and is wrong in both directions: a shared shopping list somebody happened to create is not theirs, and a note they rewrote arguably is. Worth noting that a narrow scope protects little, since anything readable was already copyable.

**The one not to inherit from whichever form gets built first.** Removal is the only coercive act in the design. A household is not always a neutral place, and one member ejecting another whose material is on the instance is a situation this product can produce.

**Not blocking the data model.** What the model needs is settled: a member is referenced by four mechanisms, is not an entity, and its references survive departure.

## Entity model

Surfaced 2026-08-07 while assembling the entity model, which had never been written down and existed only as prose across five documents.

### Which types carry a body

**Status:** leaning
**Destination:** the entity model

A body in the substrate's sense is a specific and expensive thing: divided into blocks, anchorable, the unit at which edits merge or conflict. Which types have one had never been stated, and the substrate's own worked example of how a body divides is a shared shopping list.

**Leaning: a quest carries no body.** Prose about a quest is a related Note. Knowledge holds the content and Guidance holds the shape of the work, and the precedent is already set by a scan and the note annotating it being two entities, with two retrieval results accepted as the cost.

**Accepted cost, recorded so it is not discovered later.** Notes are the fastest-accumulating type, and a large share of them would then exist only because a quest needed a sentence. That is not the rejected claim that Knowledge is subordinate to action, but it is that claim reappearing as a shape rather than as a statement.

**Rejected: an ordinary text field on a quest, which is not a body.** It conflicts wholesale like a title does, carries no blocks and no anchors, and is cheap. The attraction is that it needs none of the block apparatus. Against it: a description field is where the plan starts living, and it grows.

**Open: whether a shopping list carries a body.** Either it does and it is the second type that has one, or the substrate's list-item example is about something else. A list is an ordered set of short items by construction, which is the pattern block division serves best.

**Leaning on the entry: an entry is a block, and a relation to an item anchors to that block as a whole.** This survives the ordinary edit, since a block keeps its identity when its own text changes, so correcting coffee to coffee beans keeps the relation attached.

**What it requires that the substrate does not currently provide.** An anchor is defined there as finer than a block, a span located within one, and explicitly not the same concept. Pointing at a block as a whole is coarser and is not in the document. It generalises to notes rather than being a shopping list special case.

**Two costs it does not remove.** Block identity is described as recognisable after the surrounding text changes rather than as assigned, so a heavy enough rewrite of the entry drops the anchor, which is the span anchor's failure arriving less often. And deleting the entry leaves the relation to the item surviving without an anchor, because relations surviving edits is a rule. Whether that is wrong turns on whether a list holds a relation to an item independently of any entry, which nothing says.

### Creation method, of which capture method is a subset

**Status:** leaning, and it is an amendment to the substrate spec rather than a gap
**Destination:** the substrate spec

The substrate records capture method on every entity and lists quick capture, a form, a barcode scan, a file upload, an import, a routine spawning a quest, and the public interface. Most of those are not captures by that document's own definition.

**Capture is a mode:** unplanned, interrupting something else, with the person able to be somewhere else holding something they are about to lose. A form is deliberate creation by the same distinction. A routine spawning a quest has no person in it at all.

**The field's own first justification proves the scope has to be wider.** It cites DR-001 on the provenance of imported data being unrecoverable loss, and import is not a capture. The field must therefore cover every creation path in order to do the job it exists for. The name is wrong, not the scope.

**What the current wording loses.** The set of creation methods is open and a client may add to it freely. The subset carrying the capture guarantee is not open in the same way, because meeting the offline durability floor is what admits a method to it. An always online client can invent any number of ways to create things and can never add a capture method.

**Whether a method carries the guarantee is a fact about the method, not about the entity**, so it is not a second property to store.

**Unaffected: bulk state.** Its initial value derives from how the entity was created, which works identically under the wider reading.

**Watch:** the existing rejection of recording device identity notes that methods identifying a particular device rather than a way of creating something would reconstitute it by another route. Widening the field's name does not widen that.

## Export

### Domain asymmetry

**Status:** parked
**Destination:** substrate spec or export spec, when export is scoped

Export is not one problem. Knowledge exports to files nearly trivially, since that is already the body format. Tracking is tabular by nature. Guidance is a tree with state, which is more awkward but a solved shape.

**The hard part is the cross-domain layer**, which is exactly what makes Altair distinct. Relations span domains and belong to no single export. History is a parallel record with no place in a snapshot of current state. Audience means nothing outside a household.

Consequence worth remembering: a per-domain export gives back roughly what three separate applications would have given. The connections are what is lost, and they are the reason for using Altair.

**Deliberately not committing to any format yet.**

**Revisit when:** there is a user who is not the person running the instance.

## Rejected

Options that were considered and set aside. Recorded with reasons, because a rejection without its reasoning gets re-argued from scratch.

Rejections are not permanent. If circumstances change, reopening one is legitimate. What is not legitimate is reopening one without knowing why it was closed.

### Arrangement

Left here when arrangement moved from the entity to the container in the substrate spec.

**Rejected: one arrangement belonging to the entity, holding wherever it appears.** It cannot survive a change of container in any acceptable form. Retaining the value on a move lands the entity in a position inherited from a set it is no longer part of, which reads as arbitrary placement. Reassigning it on a move shifts the entity in every other view it appears in, so tidying an arc silently reorders a category. Both are the system moving something the person did not move, against the vision's rule that returning to a view finds it as it was left.

**Rejected: a sortable key of unbounded length, so that placing between two entities never writes a neighbour.** The never-write-neighbours property was carrying two arguments and neither survived. Concurrent reordering by two people is not a case the system owes anything to, because the offline floor is creation and reordering is not capture. And the write cost is trivial at household container sizes. What remains is a growing opaque key that every client must generate correctly, bought for nothing.

**Rejected: an ordered list or map held per container as its own structure.** It satisfies the ordering requirement, and it is rejected for cost rather than correctness once positions became per-container anyway. A position column on the containment relationship is the same information without a second structure to keep consistent with the entities it names, and without an audience of its own that would be the union of everything it holds.

**Rejected: clients assigning position at creation.** Appending requires knowing what is already in the container, which a client that has not synced does not have. The instance assigns, on the same reasoning as block identity.

### Identity assignment

Left here when the question of who assigns entity identity was answered in the architecture foundations, which record client assignment as strongly preferred rather than forced, and the instance assigning as viable.

**Rejected: the instance assigns on arrival, with references rewritten during reconciliation.** Identity is then not stable across sync, so anything that captured the earlier value, including an export or a copied link, points at nothing afterwards. It also puts a local-to-canonical mapping in every client that does more than capture.

**Rejected: the instance pre-allocates ranges of identifiers to clients.** Client assignment with extra steps, and a client that has been away long enough to exhaust its range cannot create until it can reach the instance, which is the one thing capture must never require.

**Recorded because it was raised as an objection to client assignment and is not one.** Identity originating outside the instance's control means a client can assert an identifier that already exists. Every client is one of the household's own authenticated devices, so a hostile one implies compromised credentials, which is a larger problem than a collision.

### Storage and files

**Markdown files as the canonical store.** Rejected in DR-001. Cross-domain retrieval becomes a full-corpus parse, field-level conflict merging is impossible when the unit of change is a whole file, and Guidance and Tracking have no natural file representation, so a second store would be needed anyway. That last point is the strongest one: the single-format simplicity that motivated the option does not survive contact with two of the three domains.

**Markdown plus sidecar files or extended frontmatter.** Rejected in DR-001, and judged the worst of the three options. Two sources of truth that drift with no way to detect it, and a format that accretes optional keys older files lack.

**A hardware ceiling as a substrate requirement.** Removed from the storage requirements table, where it read as running on a modest single-machine deployment. "Modest" is subjective and culturally variable, and any attempt to make it concrete either names hardware that dates immediately or excludes deployments for no reason the requirements themselves demand. It was also not a goal: the intent is not to compromise or optimise prematurely, and broadening what Altair runs on is a legitimate later aim rather than a constraint the substrate has to satisfy now. Retrieval is the workload that made the row awkward, since embedding and query expansion have very different appetites and the second is not interactive on small hardware. Note that a second machine on the user's own network is not an external provider, so nothing in the vision document requires one box.

**Folders with paths, or any navigation surface where the tree is the primary route.** Reaffirmed 2026-08-06, restated because the Knowledge case is where it will be tested. Optional single-parent categories with shallow nesting are not this, and are permitted on the Tracking pattern.

**A numeric size or count limit as the rule for "not a file store."** Rejected. Nobody erodes that boundary by uploading a large video, they erode it by building a Files section with a folder tree. A functional rule that prohibits filesystem-shaped navigation defends the actual boundary; a number defends nothing.

**Attachments and file-notes as two separate concepts.** Rejected. It forces a decision at capture time about which one to use, on the path that must not stop to ask, and makes attachments permanently second-class for relations, search, and history. Collapsed into files-as-entities, with inline versus reference display derived from media type.

### Knowledge

**Stub creation on reference.** Rejected. The attraction is that it makes the forward-reference case work with no new concepts, since the target exists from the first keystroke and the substrate already tolerates an entity with no content. Against it: every typo becomes a permanent entity, the stubs carry titles nobody intended, and the set of empty stubs is the owed-work list under another name, which is a counter rising during absence. It also forces answers about category and audience on a path where nobody was asked anything. Replaced by creation from the reference gesture's empty state, which inverts the order so both endpoints exist before the relation does.

**A reference holding a name rather than an identity, resolving later.** Rejected, and it is the closest thing to what the vision document's dangling-link Should described. Resolution has no good answer: matching by name means the system forms a durable connection nobody asked for on the strength of a string, and manual resolution requires the person to remember the reference exists. It also adds a second kind of pointer to a model that has one, and makes deletion ambiguous, since a resolved reference to a deleted entity would presumably dangle again, contradicting retain-and-hide. Note that Tracking already uses "unresolved" for a different situation, where both endpoints exist and the outcome is not yet known.

**The system reading prose and offering to create notes for names that match nothing.** Rejected. It requires inferring which phrases were meant as references, would fire constantly and mostly wrongly, is the system forming an opinion and acting on it uninvited, and puts a question on the writing path, where questions cost most.

**A corrected-versus-authored tier for derived text.** Rejected as overbuilding. The origin of the words is not the distinction. What survives is one operational fact needing no user-facing vocabulary: an asked-for extraction pass does not overwrite an edit.

**Version history as a general undo.** Rejected, on reasoning already established for deletion. Action-scoped recovery works in the minutes after an act and fails after an absence, when the person remembers the changed content but not how many edits ago it changed. It also implies a general undo mechanism nothing else in the design describes.

**Piecewise restore, taking back one paragraph as a system feature.** Rejected. It is more useful in the case that actually occurs, which is overwriting one section rather than ruining a whole note, and that is a genuine attraction. Against it: it is no longer a single comprehensible act, and it is unusable without a diff, which is a Should, so a core capability would depend on an optional one. Copying out of an old version covers it with ordinary editing.

**Restore that returns a note's relations, category, or audience.** Rejected. Restoring the links a note had last month silently removes a connection made since, and connections are what the product is for. Restore acts on the body and nothing else.

**A standing audience rule attached to a category.** Rejected. The attraction is obvious and people will assume it is what sharing a category does: put a note in the work category and it becomes shared, take it out and it does not. Against it: that is inheritance, which is the second axis the vision document names as the point where audience becomes the excluded permission system. It also promises unsharing, which the product must never do. Replaced by bulk broadening as an announced act, plus per-category creation defaults.

**An ERD as the home for the relation type vocabulary.** Rejected, though the instinct that an ERD is the standard artifact for this job is correct in general. These types are not schema facts. *Blocks* earns its place because completing something still blocked produces a warning; *Uses* earns its place because it commits stock and is resolved at a quest's terminal state. The content is behaviour, not cardinality. An ERD would also have to record that a relation joins any type to any type with no allow-list, at which point the diagram has one box and conveys nothing, and it cannot carry two settled positions: that the set is open, and that types are user-renameable while behaviour stays fixed. An ERD belongs to the architecture layer, which is deliberately not begun.

**Two separate category sets, one per domain.** Rejected. The attraction is avoiding a collision: the organising axes of a pantry and of a note collection may have nothing useful to say to each other, and one set means every picker shows categories used only elsewhere. Against it: two vocabularies to remember is worse than one, and a category reaching only one domain puts back a boundary the product does not otherwise have. The cost is accepted rather than mitigated, since both available mitigations are worse than the problem.

**Dropping categories entirely, leaving retrieval as the only organising mechanism.** Considered seriously and not taken. It is coherent, and it is what dropping tags already implies. What it removes is the ability to browse without knowing what you want, which is precisely the state of someone returning after three weeks. Recorded because it is the cheaper option and someone will propose it again.

**Treating a graph visualisation as a domain question.** Dropped rather than decided. Whether the material around a note is drawn as a diagram depends on the screen in the person's hand, which is the same reasoning that put barcode scanning outside the Tracking domain. What the domain owes is the constraint on any client offering it: nothing reachable only through it, and a client without it is in no way lesser. Recorded because the anti-hobby argument makes it look like a scope question when it is a rendering one.

**Also rejected, in the drafting of that section: a rule that the diagram must not become a client's primary navigation.** It reads as a natural extension of the file-store exclusion and it is not one. Nothing about Altair changes to provide such a client, since the diagram is drawn from the same relations and retrieval as every other surface, so the rule would have been a presentation constraint in a document that parks presentation by policy. The substrate already expects clients to differ this much. The reachability requirement carries the whole guard on its own.

**System-created daily notes.** Rejected. The attraction is that it is the most-used feature of most tools in this category. Against it: it creates containers on a schedule whether or not anything goes in them, which accumulates empties and creates them with no action to attribute them to. Journaling is available as ordinary notes, and date-anchoring is available because any entity can carry a date.

### Sync and capture

**Recording which device created an entity.** Rejected, and removed from the substrate spec's open questions. The proposal was a unique device identity, since a class of device would not serve either justification offered for it: telling two phones in one household apart is the point of conflict provenance, and a class largely duplicates capture method anyway.

Unique device identity is tracking data. It points at one physical object, which in a household means one person, and it accumulates into patterns about when and where someone works rather than which features they use. Capture method's permitted usage reporting was written for facts about tools, and does not survive being applied to this. The perception cost is real too, and it lands hardest on a product whose entire proposition is that nobody is watching.

The practical argument is simpler and would have been enough on its own. Nothing currently needs the field. Debugging the capture guarantee is served by capture method, and resolving a conflict means choosing a value rather than adjudicating its source. Adding a field later when something concrete requires it is cheap; removing one after years of entities carry it is not.

**Watch for it returning sideways.** The set of capture methods is open and grows as clients add ways to create things. Methods that identify a particular device rather than a way of creating something would reconstitute this by another route.

**Full offline operation as a Must.** Rejected as too large a permanent commitment: a complete local replica plus a merge story for every entity. Narrowed to *capture never fails*, with offline reading and editing as a Should. The reasoning is that a narrow commitment always kept beats a broad one quietly broken, and expanding outward from a guaranteed capture path is a viable route to the larger goal.

**A sync engine.** Rejected as a consequence of the above. Offline capture is create-only, so two devices creating different things is not a disagreement and there is nothing to merge. What remains is a durable outbound queue, not a bidirectional reconciliation system.

**Mandating offline relation-forming for all clients.** Rejected. It would force development on constrained platforms that may never benefit. Replaced by a floor every client meets, creation, with anything above it a platform decision, subject to the same guarantees if offered.

### Deletion

**Nothing is ever permanently deleted.** Rejected as impractical. Users legitimately want things gone, and a system that cannot do it invites workarounds.

**Erasure as a routine operation alongside delete.** Also rejected, in the opposite direction. Immediate irreversible deletion is a footgun precisely because the moment a user feels most certain is often the moment they are wrong. Landed on retain-by-default with a holding state, scheduled cleanup on a stated window, and immediate erasure available but always announced.

### Classification

**Tags.** Rejected outright rather than deferred. The attraction is real: everyone has them, they are cheap, and they look like the obvious organising axis for a notes domain.

The objection is that a tag is a retrieval mechanism requiring the person to reproduce a choice they made months ago. Was it programming, development, or embedded. That is exactly the recall the findable-without-recall Must says a person cannot be asked for, so tags fail these users by construction rather than through insufficient discipline.

Consequence accepted: with no tags and no folders, retrieval and optional categories carry the whole organising load. That raises the stakes on retrieval considerably, which is consistent with the vision document already treating it as core rather than as a search box. Revisit only if users ask, and note that semantic retrieval is likely a better answer to what they would be asking for.

**Deriving bulk state from capture origin.** Rejected. A scanned book annotated over a semester genuinely becomes a note, and freezing it as a file forever is wrong.

**Deriving bulk state from current contents.** Also rejected. Entities would move in and out of a filter as extraction runs, with no user action to attribute the change to. Landed on the user authoring their own content into it, with tags, relations, and derived text excluded because all three occur during ordinary bulk workflows.

### Vocabulary

**Knowledge as subordinate to action.** Superseded 2026-08-06. It was protecting a real risk, the graph as an absorbing hobby, but that risk is already defended by a small fixed surface, near-zero configuration, no plugin marketplace, and nothing rearranging itself. The claim is false about ordinary use: an article captured against a breakdown that has not happened serves nothing and is obviously correct to have captured. Replaced by parity between the three domains.

**Inverting it, so that everything is subordinate to Knowledge.** Rejected. The same shape of claim with the sign flipped, and it would license Knowledge features the anti-hobby guard should still block. All three domains are useful in isolation and each has strong dedicated competitors. The product is the intentional combination, not a ranking within it.

**"Snapshot" as the name for per-entity content history.** Rejected on collision. On a self-hosted product the operator also takes real backups, and the word reads as an instance backup to anyone arriving cold. Replaced by version.

**Project, Milestone, Task.** Rejected. "Task" arrives pre-loaded with dread for these users, and the familiarity gained does not offset the activation cost.

**Keeping Epic as the middle tier.** Rejected once Campaign was chosen. The justification for Epic was that it reads as familiar to people arriving from project management tools, which is not a goal this product has, and it was the one term signalling "project tracker."

**Saga instead of Campaign.** Rejected on connotation. A saga colloquially suggests a drawn-out ordeal, which is the wrong affect for the top-level container in a tool built for people who already feel behind.

**A full celestial or voyage theme.** Rejected. Celestial nouns break at the leaf level, which is the level users touch most often.

**Continuous derivation of a container's state from its children.** Rejected. An empty campaign is valid and has nothing to derive from, a person can stop a campaign while quests under it remain untouched, and a recomputing state moves without the person. Note the narrower case is not rejected: a one-way movement of a Waiting parent to Working when a child is started remains open, and is recorded as an open question in the Guidance PRD rather than as a rejection.

**A reduced state set for campaigns and arcs.** Rejected. The argument for it was that a container is arguably only ever live or not, but the middle state does mean something distinct at container level: the person considers it live, which is not the same as any child being active. One vocabulary learned once beat a second smaller set to remember.

**Relations carrying no data beyond type and direction.** Superseded. A quantified relation holds a value that belongs to neither endpoint, and the alternative was an intermediate entity existing only for the model's convenience. Substrate now allows type-defined properties.

**Arbitrary user-defined properties on relations.** Rejected. A general property bag is a schema system by another name, and it would be the one place extensibility leaked into a design that keeps states, types, and entity structure fixed. Properties follow from the type; untyped relations carry none.

**Framing deletion recovery as undoing a user action.** Rejected. Action-scoped recovery works in the minutes after a deletion and fails after an absence, when the person remembers the missing thing but not the act. It also implies a general undo mechanism nothing else in the design describes. Replaced by entity-scoped recovery with the grouping of a single act retained.

**Backlog, Doing, Done as quest states.** Rejected. The set reads as a kanban board, which is the register the rest of the vocabulary work was avoiding, and "Done" claims achievement in a system where the terminal state only means the work stopped. Replaced by Waiting, Working, Worked.

**A separate abandoned or dropped state.** Rejected. It would exist only to record that something did not work out, which is a judgement about the person under another name. A quest interrupted by life sits in the same terminal state as one carried through, and the system does not ask which it was.

**"Graph" and "universal relation system" as vocabulary.** Rejected. The first implies a specific data structure in a document that refuses to imply any; the second is a named subsystem where the document wanted a plain statement. Replaced by "everything connects" and "anything can be linked to anything else."

### Scope and governance

**Binding third-party clients to the full Must list.** Rejected as unenforceable. A vision document cannot govern a fork. Replaced by a rule about the core: every capability is reachable through the public interface, so parity is always achievable even if a given client does not achieve it.

**Provider-optional as an absolute for generative AI.** Rejected as infeasible. If one commercial provider is the only one capable of something, and the feature is opt-in and off by default, nobody is being forced. The bullet was also protecting the wrong thing: the real risk is data leaving the instance, which is now addressed directly by a no-silent-egress rule.

**An in-app import plugin runtime.** Rejected. It creates a permanent API surface, a trust boundary around third-party code with full data access, and an ongoing compatibility burden. Replaced by a documented interchange format with community converters running as standalone tools outside the application.

**Building export early as an obligation.** Rejected as a sequencing claim smuggled into a decision record. A single self-hosting operator already holds the store, so an exporter is worth little until there is a user who is not the person running the instance. What survives is a schema constraint: do not make choices that foreclose export later.

**Evaluated 2026-08-13: building on LifeOS instead of continuing Altair.** Rejected. LifeOS is a conventions-and-prompts layer over an AI coding agent as runtime; it contains no substrate, no sync, no conflict model, no household concept, no API, so there is no wheel there to avoid reinventing. The parts of Altair that are expensive do not exist in it, and the thin layer it does have is shaped around premises the vision excludes: the system deciding the next move, pushed notifications, adaptive self-curating memory, a definitional ritual before the product works, model-written content as the canonical record. The one shared component, markdown notes with wikilinks, already exists as Lattice. Where the reuse instinct is right is one level down, at the library layer, and that evaluation belongs after the entity model and component diagram, per the standing order.

**A leaning for the eventual store evaluation, recorded 2026-08-13.** A hosted Postgres product the author already pays for enters the evaluation as a presumptive candidate: pgvector and embedding generation in the box, the paid plan counted in its favour. Counted against it: hosted infrastructure is readable, priceable, and switch-offable by someone else, which is what the vision's operating model exists to rule out, and the same software self-hosted is the exit if that starts to matter. Not a decision. The evaluation itself still comes after the entity model and the component diagram.

**To research, recorded 2026-08-13: DeepSearcher (zilliztech/deep-searcher) as example or inspiration for retrieval.** An open-source deep-research system over private data: it decomposes a question, searches a vector store iteratively, and reasons over what comes back. Two things make it worth studying rather than merely noting. It demonstrates the whole loop running on hardware the user controls, local models and local embeddings included, which is evidence for the retrieval class's own-instance rule being practical. And its shape splits cleanly across the vision's two AI classes: the retrieval mechanics, decomposition and semantic search over one's own corpus, inform the core retrieval layer, while its report generation produces new text and is therefore generation-class, opt-in territory. Inspiration and reference, not a dependency, and reading it belongs with the retrieval design work rather than before it.

**Settled 2026-08-13: a routine's schedule expresses both pattern families.** Calendar-anchored, every Tuesday and the first of the month, and completion-anchored, an interval from when the last occurrence reached its terminal state. Both are genuinely needed: bins are calendar work, mowing and descaling recur from when they were last done. Completion-anchored also cannot pile up by construction, since the next occurrence does not exist until the previous one resolves. Found while assembling the data model, which surfaced that the Guidance PRD never states what a pattern can express, nor what a routine holds that an occurrence's content is created from. The PRD amendment was made 2026-08-13 and settled the never-marked case per the lean: the routine simply waits, holding one live occurrence and producing nothing more until it resolves, with the next interval anchored from the state change; honest, nothing accumulates, and the cost is a stalled routine that recovers the moment the person touches it. The amendment also states what a recurrence holds, what a person could set on a quest by hand, stamped once at spawn and reaching future occurrences only, and it opens one new Guidance question: whether a quantified relation belongs in the stampable set, since a stamped Uses commits stock at spawn.

**Settled 2026-08-13: an item's name is the universal title.** The Tracking PRD's consistent use of "name" is that domain's word for the title, per the substrate's rule that what a type calls the title may differ, and not a property of its own. Two words for one slot would be a vocabulary problem, and the concept and intent are the same. "An item with a name and nothing else is a complete item" is a validity floor under the domain's vocabulary, not an inventory, and the substrate's actual floor is lower still.

**Settled 2026-08-13: shopping list entries are blocks in a body.** The list's content uses the body mechanism as it stands: each entry is a list-item block with its own identity, the sequence of blocks is the list, and an entry's text is title-shaped plain text. This is what the substrate's block division already yields from a list, and that rule was written with shopping lists as its named motivating case, so concurrent household adds merge with nothing new. Vocabulary for the Tracking PRD: it speaks of entries, not of the body, since body reads as long form and an entry is a few words; the substrate mapping is one sentence. Both substrate amendments were made 2026-08-13: an anchor attaches at a phrase or at a block, and a block keeps its identity across edits to its own text. Without both, an entry-to-item link detaches when the entry is reworded, which is the link failing at its job. Entry removal was settled in the Tracking amendment the same day, per the lean: crossing off is an act on a deliberate surface whose announced effects may include removing the entry, removing the relation, and offering the purchase log, which keeps the rule against removing connections as a side effect intact. Rejected: entries as first-class entities. Ten scribbled lines would be ten entities each carrying identity, audience, arrangement, and lifecycle; composition would demand match-or-create decisions; and the inventory silts up with one-offs, against the PRD's founding position that an entry pointing at nothing is text and complete. Rejected: structured entry records as a new mechanism. A parallel content system that re-answers merging, versioning, and search, which bodies already answer, and whose fields are a schema by another name.

**Settled 2026-08-13: the entry-to-item bridge is one gesture wide, in both directions.** Five gestures. Two already in the Tracking PRD: bulk fill from low stock, and consuming the last of something untracked creating the item at an amount of none, with its landing on a list remaining an offer, since nothing adds to a list unasked. Three added by amendment 2026-08-13: composing a list offers existing items as matches the way the purchase path already does, never demanding; crossing an entry off may offer the purchase log, which can create the item at the moment it entered the household; and an item's own surface offers adding it to a list, creating an entry born with an anchored relation, its text the item's title and thereafter ordinary text. Where more than one list exists, the item-side gesture asks which, or uses a default. A person-designated default and a client remembering the person's own last choice are both permitted: replaying a deliberate choice the person made is deterministic and attributable in one sentence, which is not the inference-driven adaptation the vision excludes. What stays excluded is a default the person cannot trace to an act of their own.

### Process

**A Lattice findings document.** Dropped. It was proposed to preserve knowledge assumed to be fading, but Lattice is still in daily use and its conclusions were already distilled into the vision document. The exercise would have re-derived what had just been derived.

---

**Status:** parked, permanently as far as normative documents are concerned
**Destination:** design system

Recorded only so they are not rediscovered as open problems.

- Where surfacing appears: a panel, an inline marker, an addition to a result set
- What a conflict looks like, and what resolving one involves
- Whether the bulk-capture filter is on by default, and how a filtered view discloses that matches exist outside it
- What the holding state looks like as a surface

The vision document rules these out by policy. They are real questions with real answers; they simply are not settled in documents that answer *whether*.

### Component model decisions, taken in one pass

**Status:** settled, 2026-08-14, recorded in the component model

Seven open items were closed together rather than one at a time, because none of them changed a boundary already drawn and holding them open was costing more than deciding them.

**Time produces no writes.** Recurrences, holding-state expiry, and the horizon are computed when asked. What is left on a clock is reclamation and delivery, which changes no answer and therefore needs no write path. **Rejected: a ticker producing occurrence records in advance.** It needs catch-up logic that must avoid backfilling days that went by, it leaves records behind for recurrences nobody engaged with, and it makes the horizon an artifact of when a job last ran.

**An occurrence becomes a record on first touch.** Before that it is a projection of the recurrence as it stands. Every normative claim in the Guidance PRD holds: independent identity and editability once it exists, no rewriting of an occurrence that exists, and a past occurrence stays past.

The rest, each with its reasoning stated in the component model: the write path enforces audience on the read path's predicate; the outbox carries bytes; erasing a file removes the record before the bytes; outstanding derivation is computed from the store; operator plane capabilities are reachable through the public interface; configuration lives in the structured store; an undelivered notification is dropped rather than queued.

**Amended in the architecture:** the constraint on what may change a surface prohibited the household and the calendar along with the thing it was aimed at, and now names the system authoring a change as the target. Reclamation and delivery is named as a component.

### v0 and semantic retrieval

**Status:** settled, 2026-08-15, applied to Altair v0 Scope

**Semantic retrieval is in v0, and is the last part of it built.** The scope page as first written deferred it, on the ground that tuning needs a corpus only v0 can produce.

**The hole in that argument.** Tuning does not need a corpus, it needs an embedded corpus. If nothing is embedded while v0 runs, then the day tuning begins it is still one backfill away from starting, so the deferral bought a delay rather than avoided work. The sequencing holds for the ranking behaviour and does not carry to the vector production underneath it.

**Two further reasons.** Altair has no tags and no folders, so retrieval and optional categories carry the entire organising load, and a literal-only release leaves recall of one's own wording as the only way in. And semantic retrieval is a large part of how the product answers memory difficulty, which is the need it exists for rather than a refinement of it.

**What stays deferred:** surfacing, which is the proactive behaviour rather than the retrieval, and text extraction from files. A cross-encoder is not required, since inference is several models and each is independently absent.

**Recorded as an error rather than a decision.** The v0 absence list, written earlier the same day, stated that embeddings were deferred and the derivation worker absent in whole. The scope page defers semantic retrieval and text extraction and says nothing about embedding generation, so that third claim was supplied rather than found. It was reasoned from in a store evaluation before being caught, which is the cost of the error and the reason it is recorded here.

### Wave 2.1 decisions, taken at the wave boundary

**Status:** settled, 2026-08-17. One of them amends the substrate spec and the data model, and those amendments are made. The rest constrain how the intent spine is built.

Reading Wave 2.1 against the documents before writing any of it found two things the implementation plan did not know: the store cannot answer the question conflict detection asks, and the wire can name a relation in a removal that the store has nowhere to hold. Both were decided here rather than discovered halfway through the write path.

**Per-part write provenance is a side table.** `entity_part_counter`, keyed by entity and part, holding the counter the part last moved at and the member who moved it, in migration two. The rule is that a stale base touching parts disjoint from what moved since applies without conflict, and evaluating it needs to know which parts moved between two counter values. Nothing held that: the change sequence carries block identities but no field list and no counter, an intent row carries the counter after a write but not what it wrote, and versions are Knowledge-only and declinable. The member is there because a conflict names whose the other value was, and an entity's author is its creator and never changes.

**Rejected: a map of parts to counters as a document column on the entity row.** Fewer moving parts, and it would die with the row rather than needing its own removal on erasure. Against it: the shape is unenforced by the store, and a document column in a schema that rejected one — for content, on reasoning that does not reach this — invites the argument every time somebody reads it. A fourth side table beside dates, assignments, and property values is the idiom already in use.

**Rejected: the changed parts and the counter on the change row.** It reuses a write the transaction already makes and adds no table. Against it: the change sequence is trimmed below the horizon, so conflict correctness would decay as history is trimmed, and the component model is explicit that everything reclamation removes is already gone by predicate. Making the write path depend on a trimmer having not yet run is the wrong direction, and it is the shape of thing that surfaces months later.

**Rejected: deferring conflict detection to multi-device.** The v0 scope defers sync and integrity as moot with one instance and no replicas, so there is cover for it. Against it: the foundations mark the three-outcome rule load-bearing, the machinery is small now, and a counter that advances while nothing reads it is a half-built mechanism whose errors are found years later with a corpus behind them.

**Relations join the holding state.** Removing a relation is reversible, it returns with the act that removed it, and there is no restoring one on its own. Migration two gives a relation a lifecycle, the time it was removed, and the act that removed it. This needs no change to the wire: a removal is already the relation-gone signal, a restoration is an ordinary relation write, and restoring an entity with its group reaches the relations in that group.

**Rejected: a hard delete, documented as such.** It is what the store and the wire already implied, and it would have cost one sentence. Against it: it would be the only permanent destructive act in the design with no holding state and no announcement, in a document that says anything becoming permanent does so visibly and never as a side effect. A removal also groups relations into the same act as entities, which reads as a promise that restoring the act restores all of it.

**Rejected: refusing a removal that names relations, until the question was answered.** Honest, and cheap. Against it: it puts a hole in an accepted contract that a client can reach, and the thing that would find it is a terminal client gesture in Wave 4.

**Wave 2.1 stands up the submission call end to end**, with the other five calls answering unimplemented. Two of the item's requirements are only observable at the wire: a batch that is never all-or-nothing, and a refusal that reveals nothing, which DR-004 extends to the status code, so a submission where every intent is refused answers success with the refusals inside it. Neither is testable from an internal function. It also gives Wave 1.4's token validation its first caller, which is otherwise built and unused.

**Two findings recorded rather than decided.**

- **The schema's cascading deletes do not fire under erasure.** Every one of them hangs off a delete of the entity row, and erasure strips content and leaves a tombstone rather than deleting it. So the erase path removes blocks, dates, assignments, property values, side-table rows, event records, embeddings, and derived text explicitly. The comment above the event record table describing its cascade as erasure describes something that never happens.
- **A single client can conflict with itself**, by composing two edits to one entity offline against the same counter. Closed by a new outbox conformance scenario requiring a client to send the counter it was last acknowledged rather than the one it composed against. Device identity is not available as an instance-side answer to this, being rejected outright above.

**One consequence for the implementation plan, not taken here.** Relation removal cannot be exercised without relation creation, so relation create, remove, erase, and restore belong in 2.1 rather than 2.2, which leaves 2.2 holding anchors and type-declared properties — both of which need bodies and the relation type table anyway. Destination: the v0 implementation plan, at the Wave 2 re-plan.

### Notification content and transport

Rescued from an orientation section that was removed. Recorded here because
both are rejections with reasoning and are written down nowhere else.

**Rejected in passing: disclosing least as a universal default.** It is the obvious safe answer and it is wrong here. A notification carrying no information is indistinguishable from an application asking for attention, and the ordinary response is to mute the application, so a default protecting the content by getting the channel switched off has protected nothing and cost the feature. Keying the default to the transport gets both cases right: nothing leaves the household network, nothing to withhold.

**Rejected: keying the default to the transport.** It looked right, on the reasoning that delivery never leaving the household network means nothing to withhold. The counter-example kills it: someone planning a surprise party keeps the quest private, and a notification naming it appears on their own phone, in their own house, in front of the person it was private from. Delivery was correct and the leak still happened, because being delivered to the right person and being displayed only to them are different things.

**Two exposures that do not move together:** the transport, meaning whoever is in the middle, and the screen, meaning anyone who can read the device. The second is unaffected by the first. The screen exposure is an audience question, which an earlier draft explicitly denied on the grounds that a notification goes to the person it is for. That was wrong for the same reason.

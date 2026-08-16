# Altair Vision & Scope

**One sentence:** Altair is a self-hosted system where the things you are working toward, the things you know, and the things you own live in one connected place you own permanently, and where a thought can always be captured whatever the network is doing.

> ⚠️ **This document is normative and implementation-agnostic.** It describes what Altair *is* and *is not*, never how it is built. The Must / Should / Won't sections are permanent product identity, not a release backlog. Anyone should be able to build Altair from this document without inheriting anyone else's technical decisions.

---

## Contents

- [Vision statement](#vision-statement)
- [The two principles](#the-two-principles)
- [The problem Altair exists to solve](#the-problem-altair-exists-to-solve)
- [What Altair is](#what-altair-is)
- [Vocabulary](#vocabulary)
- [Who Altair is for](#who-altair-is-for)
- [How Altair differs](#how-altair-differs)
- [Must have](#must-have-permanent)
- [Should have](#should-have-desirable-not-defining)
- [Won't have](#wont-have-permanent-exclusions)
- [AI](#ai-two-classes-two-rules)
- [Tradeoffs accepted on purpose](#tradeoffs-accepted-on-purpose)
- [How to use this document](#how-to-use-this-document)

---

## Vision statement

**A single calm place, owned by the person using it, where intention, memory, and resources connect to each other.**

A life does not divide neatly into projects, notes, and stuff. Rebuilding a kitchen is a plan, a pile of decisions, and a shopping list, all at once. Splitting that across three applications creates three sync boundaries, three search boxes, and three places to forget something.

Altair's bet: **one place where anything can connect to anything, searchable in a single pass, is worth more than best-in-class depth in any single domain.**

Or, put the way the idea started: capture loosely, retrieve intelligently, no required rituals.

---

## The two principles

Everything below descends from these. When a design decision is contested, resolve it here first.

### 1. Progress over perfection

**The system must be useful when the data is incomplete, imprecise, and out of date, because it always will be.**

Most tools deliver value proportional to how thoroughly they are maintained. That maintenance burden is exactly what neurodivergent users cannot sustain, so the tool becomes a debt and gets abandoned.

Altair inverts this. Partial data is the **expected** state, not a degraded one.

**In practice:**

- "I don't know how much is left, just mark it lower" is a supported operation.
- A campaign with no arcs is valid. A quest with no parent is valid.
- Six tracked items out of four hundred is a working inventory, not a broken one.
- No completion percentage that implies a target of 100%.
- No empty state that reads as an accusation.

### 2. No barriers to re-entry

**The cost of returning after three weeks away must be near zero.**

For the people Altair is built for, coming back is the hardest moment. It is also the moment most software punishes hardest, with overdue counts, broken streaks, notification backlogs, and a board that quietly became a lie while you were gone.

**In practice:**

- Nothing accumulates that must be cleared before the tool becomes usable again.
- No streak, score, or status that degrades through absence.
- Nothing reorganises itself while you are away, so muscle memory still works when you return.
- Focus sessions and check-ins are **observational records**, never obligations.
- The system helps you find your way back into context you no longer remember building.

> 💡 These two principles are the primary reason Altair is not simply a lighter Notion.

---

## The problem Altair exists to solve

| Failure | What it looks like | Principle violated |
|---------|--------------------|--------------------|
| **Maintenance debt** | Value only arrives when data is complete and current. It never is. | Progress over perfection |
| **Guilt accumulation** | Overdue items and broken streaks make opening the app worse than not opening it. | No barriers to re-entry |
| **Context loss** | Returning means reconstructing what you were doing from scratch. | No barriers to re-entry |
| **Buried work** | You start something you already started, because nothing reminded you. | No barriers to re-entry |
| **Lost capture** | The thought arrives in a tunnel, on a plane, in a basement, and the app cannot take it. | Progress over perfection |
| **Fragmentation** | The note, the task, and the item that belong together live in three products that cannot see each other. | Neither |
| **Impermanence** | The vendor changes pricing, is acquired, or shuts down. Years of context leave with it. | Neither |

Each is a requirement, not a UX polish item.

---

## What Altair is

### Three domains

**Guidance** is what you are working toward.
Campaigns, then Arcs, then Quests, plus Routines that spawn quests, Focus Sessions for time-boxing, and Daily Check-ins. Answers *"what can I actually do right now?"*

**Knowledge** is what you are learning and remembering.
Notes, wiki-style links between them, automatic backlinks, point-in-time snapshots, tags. Answers *"where did I write that down?"*

**Tracking** is the resources a household runs on.
Items across nested locations and categories, consumption and purchase logs, shopping lists linked to real items. Answers *"do we have any left?"*

### Everything connects

**Anything can be linked to anything else**, across all three domains:

- A note attached to a quest
- An inventory item blocking an arc
- A focus session tagged to a campaign

**Search runs across all three domains in one pass.** This is the product, more than any individual domain is.

### Capture never fails

A thought that cannot be written down when it arrives is gone. Nothing else the software does matters if it cannot take the thing you turned to it for.

That is only half of it. A capture that lands and is then buried is lost just as thoroughly, only more slowly and with less to show for it. Altair is built along the line between those two failures, and treats them as the same problem seen from either end.

**Capture is therefore the one path that must work unconditionally.** Not "usually." Not "when connected."

Everything else, meaning reading, editing, browsing, and searching, is wanted without a network too. That is something to grow into, not a line the product refuses to cross.

### Retrieval is the other half

Loose capture without good retrieval is a landfill. Capturing freely only works if the thing you wrote down six months ago comes back to you at the moment it matters, without you having to remember it exists or where you filed it.

**Altair therefore treats retrieval as a first-class capability, not a search box.** Two behaviours make it work:

**Finding by meaning, not just by wording.** You should be able to find a note when you no longer recall the words you used. Literal search fails precisely when memory has faded, which is the case that matters most.

**Surfacing what is relevant to what you are doing.** When you begin planning something you have already begun, Altair should say so and show you the work. When you write that a project needs a particular component, Altair should be able to tell you the household already has three.

This is why "everything connects" is worth anything. Connections that nobody traverses are inert. **Surfacing is the mechanism by which no barriers to re-entry actually gets delivered**, rather than merely promised.

Two constraints keep this from becoming the thing it is trying to prevent:

- **Surfacing responds, it does not interrupt.** It appears because of something you just did, in the place you are already looking. It is not a notification, a digest, or an unread count.
- **Surfacing shows, it does not decide.** It brings your own existing material back into view. It does not rank your priorities or tell you what to work on.

### Files are entities

A file is not a second-class attachment hanging off something else. **A file is a note whose body happens to be a file**, such as a scanned receipt, a manual, or a photo of a serial number.

- **The file is canonical.** Never modified, never replaced by anything derived from it.
- **Extracted text is derived and optional.** Where text extraction exists, it makes the contents searchable. Where it does not, the entity still has a title, tags, and relations, and is therefore still findable.
- **Losing extraction is never data loss.** The file survives any derived layer being turned off or failing.
- **Corrections survive.** Where derived text is editable, a user's correction is not overwritten by a later extraction pass.
- **Display is a rendering decision, never a storage one.** Whether a file appears inline where it is related, or as a reference to open, follows from its type, not from a choice the user made when capturing it.

This is why there is no separate attachment concept: one model, no decision at capture time, and files inherit relations, tags, search, and history for free.

### The operating model

- **Sovereign.** Single-tenant, one household per deployment, and the person using it is the person running it. Deployment of your choosing, whether that is a spare machine, a home server, or a small VPS. Nobody else can raise the price, change the terms, read the contents, or switch it off.
- **Capture-safe.** Writing something down never depends on a network.
- **Free software.** AGPL v3 or later, permanently.

---

## Vocabulary

Altair's Guidance hierarchy is **Campaign, then Arc, then Quest**.

| Term | Scope | Common equivalent |
|------|-------|-------------------|
| **Campaign** | A multi-month effort. "Move house." | Project |
| **Arc** | A chunk you can hold in one sitting. "Pack the kitchen." | Milestone / Epic |
| **Quest** | One thing you can finish. "Box the spice rack." | Task |
| **Routine** | A recurring pattern that spawns quests. | Recurring task |
| **Focus Session** | A bounded, recorded window of work. | Pomodoro |
| **Daily Check-in** | A short end-of-day log. | Journal entry |

### Why not the standard terms

**"Task" arrives pre-loaded.** For the people Altair is built for, the word carries accumulated dread from every system that has failed them. **"Quest" is doing deliberate emotional work.** It reframes the unit of action as something chosen rather than owed.

**The ladder has to be coherent, or the reframing fails.** A single corporate term sitting between two narrative ones reads as a project tracker wearing a costume. Campaign, Arc, and Quest hold together under one reading: a long effort, made of chapters, made of things you can finish.

**Arc is guessable cold.** It requires no genre knowledge, it is short enough for a breadcrumb, and its meaning, a segment of a longer story, is close to universal.

### The tension, stated honestly

Themed vocabulary is in genuine tension with **no barriers to re-entry**. Someone returning after three weeks should not have to remember what a Campaign is. Boring, familiar words lower re-entry cost. Affect-shifted words lower activation cost. **Altair trades a small amount of the former for the latter**, and accepts that this is a judgement call rather than a solved problem.

The mitigation is that the ladder is only three deep and each term is guessable from context.

### The boundary this does not cross

**Vocabulary is not a reward economy.** Narrative naming lowers the activation cost of starting something. It does not license points, levels, streaks, or any other coercion mechanic.

---

## Who Altair is for

**Primary:** ADHD and otherwise neurodivergent adults running their own life and household, who can run a server themselves or live with someone who can.

**Secondary:** anyone who wants durable, private personal infrastructure and is willing to pay the self-hosting tax to get it.

**Explicitly not for:** teams, organizations, or businesses. Not a market Altair will grow into.

---

## How Altair differs

### vs. traditional project management

*(Jira, Linear, Asana, Notion databases)*

Those tools answer **"what is the status of this work, for someone else?"** They are reporting instruments. Estimation, assignment, velocity, and due dates are coordination contracts between people.

Altair answers **"what can I start, right now, given who I am today?"**

| Dimension | Traditional PM | Altair Guidance |
|-----------|----------------|-----------------|
| Purpose of hierarchy | Decomposition for **tracking** | Decomposition for **initiation**, making a scary thing small enough to start |
| Audience of the data | Stakeholders, managers | The person who wrote it |
| Overdue items | Escalation signal | Neutral state, no penalty |
| Assignment | Accountability mechanism | Household coordination only |
| Metrics | Velocity, burndown, cycle time | None |
| Cost of neglect | Board becomes a lie, trust collapses | Expected, and the system degrades gracefully |

**The key inversion:** in PM tools, the plan is an artifact you maintain for others. In Altair, the plan is scaffolding for your own working memory. Nobody audits it.

### vs. knowledge management

*(Obsidian, Roam, Logseq, Notion wikis)*

KM tools make the note the atom and the graph the product. Building and tuning the graph becomes an absorbing hobby that substitutes for the work it was meant to support. Plugin ecosystems accelerate this.

In Altair, **Knowledge is subordinate to action.** A note exists to serve a quest, a campaign, or an item.

| Dimension | Traditional KM | Altair Knowledge |
|-----------|----------------|------------------|
| The unit | The note | The relation between a note and a thing you are doing |
| Extensibility | Large plugin marketplace | Small, opinionated, fixed surface |
| Purpose of linking | Emergent insight, exploration | Retrieval and re-entry into context |
| Who traverses the links | The user, manually, when they remember to | The system, on your behalf, when it is relevant |
| Configuration effort | Unbounded, and enjoyable | Near-zero, and deliberately boring |
| Portability | Varies | Open formats in, open formats out, always |

**Obsidian will always be a better notes app.** That is accepted and intentional.

### vs. inventory management

*(Grocy, Sortly, spreadsheets, home-automation setups)*

Those systems are **complete-inventory** systems. Value arrives only when everything is entered, and accuracy decays from the moment you stop entering. Most home inventories die in week three.

Altair's Tracking treats **partial inventory as a first-class, permanent state**, which is the clearest expression of *progress over perfection*. Track the six things that actually matter, meaning the medication, the coffee, the printer filament, the cat food, and leave everything else untracked forever without the system feeling broken.

| Dimension | Traditional inventory | Altair Tracking |
|-----------|----------------------|-----------------|
| Completeness | Required for value | Never expected |
| Precision | Exact counts, units, expiry | Exact *if you want*, and "just mark it lower" is supported |
| Barcode / expiry | Often mandatory workflow | Optional accelerants |
| Connection to work | Isolated silo | Items link to quests, arcs, notes |
| Purpose | Asset accuracy | Not running out of the thing that ruins your week |

### The synthesis

None of the three categories exist in isolation in a real life. Altair's differentiation is not depth in any one of them. It is that **the boundary between them does not exist**, and that the system will cross those boundaries on your behalf.

---

## Must have (permanent)

If Altair loses any of these, it is no longer Altair.

### Ownership & durability

- [ ] **AGPL v3 or later.** Forever. Non-negotiable.
- [ ] **Runs on the deployment of your choosing.** No commercial hosting requirement, no vendor account, no minimum footprint that rules out a modest machine.
- [ ] **Single-tenant.** One deployment serves one household.
- [ ] **No required third-party service.** Nothing the project ships may depend on an external provider that a user cannot replace or remove. What an operator chooses to run their own instance on is their business.
- [ ] **Complete export in open, documented formats.** Leaving must always be possible, without tooling from the project.
- [ ] **A documented interchange format.** Anyone can write a converter into or out of Altair without the project's involvement.

### Capture

- [ ] **Capture never fails.** An entity can always be created, on any client, regardless of network state.
- [ ] **A captured thing is never lost.** Captured means it survives app closure and device restart with no network available. Anything the user has been shown as accepted meets that bar before they are shown it.
- [ ] **Partial and imprecise data is always valid.** No required field that blocks capture.
- [ ] **Capture never stops to ask.** No prompt for visibility, category, parent, or destination on the fast path.

### Retrieval

- [ ] **Findable without recall.** A user must be able to retrieve something without remembering the words they used, where they put it, or that it exists. Whether this is met is empirical, settled by whether people actually find things, and reached by tuning rather than by any threshold written here.
- [ ] **Retrieval crosses domains.** One query reaches Guidance, Knowledge, and Tracking together.
- [ ] **Relevant material surfaces where you are working.** The system brings connected things into view at the moment they apply, rather than waiting to be asked.
- [ ] **Surfacing is triggered, never pushed.** It follows from something the user just did and appears in the place they are already looking. It is never a notification, a digest, or a count.
- [ ] **Surfacing shows, it never decides.** It reveals the user's own existing material. It does not rank priorities, reorder work, or answer "what should I do today?" on the user's behalf.

### Sync & integrity

- [ ] **Sync is lossless.** No write is dropped because a device was away.
- [ ] **Divergent edits to different fields are not a conflict.** Two devices changing separate fields of the same entity is not a disagreement about intent, and merges without involving the user.
- [ ] **Divergent edits to the same field are a conflict, and are surfaced.** Last-write-wins is not acceptable for user intent. Both versions are retained until the user chooses.
- [ ] **Conflict handling never gates re-entry.** Markers are entity-local and non-blocking. Nothing is queued, counted, or badged.
- [ ] **History is append-only where it matters.** Consumption logs and snapshots are immutable records.
- [ ] **Deletion is explicit and recoverable.** Sync must never look like data loss.
- [ ] **Derived data is never canonical.** Anything extracted, generated, or inferred can be discarded without losing what it came from.

### Connections

- [ ] **Anything can be linked to anything.** No domain is an island.
- [ ] **Every capability reachable through the public interface.** No capability may exist only inside a client. Clients may lag or specialise. The interface may not privilege one of them.
- [ ] **Backlinks derived, not maintained by hand.** Relations are bidirectional by construction.

### Interaction

- [ ] **Nothing accumulates that must be cleared.** No backlog, badge, or counter that gates re-entry.
- [ ] **High-volume capture must not degrade browsable surfaces.** Bulk-captured entities are filterable out of the lists a person actually reads.
- [ ] **Filtering never makes something unfindable.** A filtered view must disclose that matches exist outside it. Hiding by default is acceptable, hiding silently is not.
- [ ] **Bulk-captured is a starting state, not a permanent one.** An entity leaves it when the user edits anything beyond metadata. Tags, relations, and derived text are metadata, since all three occur during ordinary bulk workflows.
- [ ] **Nothing enters or leaves a list except as a result of something the user did.** The cause must be proximate and attributable, so that a user can point at the action they just took and see why the view changed. Enabling a feature once is not standing consent for background processes to reorganise what they see.
- [ ] **Nothing rearranges itself.** A view does not reorder while the user is looking at it, and returning to one finds it as it was left. Layout and navigation do not adapt to behaviour. The product will change as it grows, but those changes are deliberate, versioned, and identical for everyone. What is prohibited is software that quietly reorganises itself around you, not the ordering of results the user just asked for.
- [ ] **WCAG AA minimum.** Reduced-motion honoured, never colour-alone signalling, keyboard-complete.

### Household & privacy

- [ ] **Private by default.** Anything created is visible only to its author until the user says otherwise. Because narrowing visibility is unreliable and broadening it is trivial, the safe default is the closed one.
- [ ] **Defaults are configurable, per entity type.** A household that wants inventory shared on creation can set that. The shipped defaults are conservative, and changing them is the user's decision to make.
- [ ] **Visibility can always be broadened.** Narrowing is best-effort only. A device that already holds an entity cannot be reliably made to forget it, and the product must not pretend otherwise.
- [ ] **Removing a member is non-destructive.** Access ends, authored entities remain with the household.
- [ ] **Departure includes export.** Leaving a household means leaving with your own data.

---

## Should have (desirable, not defining)

Strongly wanted. Their absence is a gap, not an identity crisis.

**Working without a network**

- Reading, browsing, and searching offline
- Editing offline, with changes reconciled on reconnection
- Full local availability of every entity and its relations

> ℹ️ **Why this is a Should and not a Must:** full offline operation is a large and permanent cost, requiring a complete local replica and a merge story for every entity. Altair commits absolutely to the part that is unrecoverable when it fails, which is capture, and treats the rest as something to grow into. Expanding outward from a guaranteed capture path is a viable route to full offline operation. Starting there is not a prerequisite for the product to be itself.

**Guidance**

- Focus sessions with timing
- Routines that spawn quests on a schedule
- A "today" surface that suggests without deciding
- Energy- or context-aware filtering

**Knowledge**

- Snapshot diffing and restore
- Dangling-link surfacing
- Text extraction from files, making their contents searchable

**Tracking**

- Low-stock thresholds and alerts
- Barcode scanning for capture
- Expiry tracking
- Batch consumption logging

**Cross-cutting**

- One first-party importer covering the common case, lossily and immediately
- Notifications with quiet hours
- Read-only calendar ingestion for context
- Instance health and administration surfaces
- Additional client platforms

> 💡 **On import:** the project ships one importer that gets people started in minutes, and publishes the interchange format so anyone can write more. Community converters run as standalone tools that emit the format, not as code executing inside Altair.

---

## Won't have (permanent exclusions)

These are settled. A proposal that contradicts one of these is out of scope regardless of how well it is implemented.

### Altair is not a SaaS product

- ❌ No hosted-first commercial tier, no freemium ladder
- ❌ No vendor account required to use your own instance
- ❌ No telemetry, analytics, or crash reporting without explicit opt-in
- ❌ No advertising, no data monetisation, ever

### Altair is not team or organizational project management

- ❌ No velocity, burndown, cycle time, or throughput reporting
- ❌ No estimation, story points, or capacity planning
- ❌ No roles, org hierarchies, or permission matrices
- ❌ No time tracking for billing, invoicing, or client work
- ❌ No cross-organization assignment or approval workflows
- ❌ No SLA, ticketing, or queue management

> ℹ️ **Not the same thing:** choosing whether a note is private or shared within a household is visibility, not a permission system. Visibility attaches to an entity and answers one question. Roles, groups, and inherited permissions are the excluded thing.

### Altair is not a collaboration or social platform

- ❌ No activity feeds, comment threads, mentions, or reactions
- ❌ No public sharing, publishing, or community layer
- ❌ No real-time multiplayer editing

### Altair does not coerce

*Direct consequence of **no barriers to re-entry**.*

- ❌ No streaks that break
- ❌ No points, badges, levels, or leaderboards
- ❌ No nag escalation or shame-based notifications
- ❌ No productivity scoring of the user
- ❌ No completion metric that implies a target of 100%
- ❌ No counter of any kind that rises while the user is away, including unresolved conflicts

### Altair does not lose things

- ❌ No capture path that can fail, refuse, or silently discard
- ❌ No feature that makes writing something down conditional on connectivity, sign-in state, or sync health

### Altair is not a feed

- ❌ No ordering that changes without the user having asked for it
- ❌ No ordering that adapts to the person, so that two members see different orders for the same data
- ❌ No adaptive or personalised navigation
- ❌ No unprompted stream of suggestions, digests, or recommendations

> ℹ️ Surfacing is the opposite of a feed. A feed decides what you see next. Surfacing answers a question you are already asking, in the place you are already asking it.

<!-- -->

> ℹ️ **Ordering a result set is not a feed.** A search the user just ran is ordered by relevance to what they asked, which may include how recent something is. That is a property of the material, identical for everyone, and it does not change until the user asks again. A feed reorders itself while you are looking at it.

### Altair is not a general-purpose platform

- ❌ No user-defined schemas or custom entity types
- ❌ No formula language or spreadsheet-style computation
- ❌ No arbitrary view builder or database UI
- ❌ No plugin marketplace or in-process extension runtime

> 💡 **Extension path:** the public interface, the interchange format, and the licence. Build alongside Altair, or fork it. There is no sandbox to maintain and no plugin API to keep compatible.

### Altair is not a file store

- ❌ No folder hierarchy or file-browser surface
- ❌ No directory synchronisation
- ❌ No files that sit apart from everything else, unlinked and unfindable

### Altair is not adjacent-category software

- ❌ Not a financial system, so no budgeting, accounting, or receipt ledgers
- ❌ Not an ERP, so no suppliers, purchase orders, multi-warehouse, or valuation
- ❌ Not a calendar or email client, with read-only ingestion at most
- ❌ Not a health or medical record system
- ❌ Not a CRM or contact manager
- ❌ Not a password or secrets manager

---

## AI (two classes, two rules)

Altair distinguishes between AI that **finds** and AI that **produces**. They carry different risks, so they carry different rules.

### Retrieval and surfacing

Semantic search, relevance matching, and cross-domain surfacing. This class only ever shows the user material they created themselves.

It is **core to the product**, not an add-on. Loose capture is only safe if retrieval is good, and surfacing is the mechanism that delivers no barriers to re-entry. A version of Altair without it would fail its own second principle.

**Rules:**

- [ ] **Only reveals, never invents.** Output is the user's own existing entities, not new text.
- [ ] **User-triggered.** Follows from a search, an edit, or opening something. Never a background push.
- [ ] **Never governing.** Shows what is relevant. Does not rank priorities or decide what matters.
- [ ] **Never required.** Literal search must always work on its own, so retrieval degrades to a plain search box rather than breaking.
- [ ] **Runs on the user's own instance.** No dependence on an external provider that the user cannot replace or remove.
- [ ] **Disableable.** A user who prefers literal search on its own can turn the rest off.

### Generation and inference

Summarisation, drafting, extraction, classification, and anything that produces new content or judgements.

**Rules:**

- [ ] **Individually opt-in.** Each feature is toggled separately. There is no single "enable AI" switch.
- [ ] **Off by default.** A fresh instance has every feature in this class disabled.
- [ ] **Never required.** No core path depends on one being on.
- [ ] **Never canonical.** Output is derived data. It sits alongside the original, never replaces it, and is correctable by the user.
- [ ] **Degrades cleanly.** Turning a feature off, or losing the provider, removes the feature and never the underlying data.
- [ ] **Provider dependency stays contained.** A feature may depend on a single provider where no real alternative exists. That dependency must never spread: it cannot become a prerequisite for anything else, and losing the provider costs that one feature and nothing more.
- [ ] **No silent egress.** Any feature that sends user data outside the instance says so plainly before it can be enabled, and names what leaves. Data leaving is never the default and never incidental to something else.

> ℹ️ **Why the two classes differ on this.** Retrieval is core, so it has to run on hardware the user controls. Generation is additive, so it can reach outward. The line that matters is not which vendor a feature uses, it is whether the user knowingly chose to send their data somewhere.

### What is deliberately not decided here

**Which features exist, and when, is a roadmap question.** The permanent lines are the two rule sets above, never the contents of any particular release.

---

## Tradeoffs accepted on purpose

| Commitment | What it costs | Why it stands |
|------------|---------------|---------------|
| Self-hosted only | Excludes the large majority of people who would benefit | Ownership is the point, and a hosted tier compromises every other guarantee |
| Capture guaranteed, the rest not | Reading and editing offline are aspirations, not promises | A narrow commitment that is always kept beats a broad one that quietly isn't |
| Retrieval as core, not optional | Raises the floor on what a minimum viable Altair has to do | Loose capture without good retrieval produces a landfill, not a system |
| Opinionated hierarchy | Some users will find Campaign, Arc, Quest wrong for their brain | Flexibility becomes configuration burden, which becomes avoidance |
| Themed vocabulary | Slightly raises re-entry cost for a returning user | Lowers activation cost, which is the harder problem |
| Three domains, none best-in-class | Obsidian, Linear, and Grocy each beat Altair in isolation | The connections between domains are worth more than depth within one |
| No in-process extensions | Users cannot change behaviour without forking | A plugin interface is a permanent compatibility burden, a documented format is not |
| Lossy import | Imported structure is flattened, not preserved | A flat pile you have today beats a faithful mapping you never finish configuring |
| Files as entities | Bulk capture can crowd the surfaces people browse | One model, no decision at capture, and files get relations and search for free |
| Best-effort narrowing of visibility | Cannot honestly promise "unshare" | Distributed devices cannot guarantee retroactive revocation, and pretending otherwise would be a lie |
| Nothing rearranges itself | Forgoes every convenience that adaptive interfaces offer | Predictability is what makes returning cheap, and it is worth more than cleverness |
| No gamification | Lower short-term engagement than competitors | Coercion mechanics fail neurodivergent users on a delay, and the crash is worse than the boost |
| Household as maximum scope | No path to team or prosumer revenue | There is no revenue path anyway, and scope discipline is the asset |

---

## How to use this document

**This document answers *whether*.** It does not answer *what*, *when*, *why*, or *how*. Those belong to domain specifications, roadmaps, decision records, and design systems respectively. Any of those may be rewritten without touching this one.

**For contributors:** this is the scope authority. If a proposed feature contradicts a **Won't**, the discussion is about amending this document, not about the feature.

**Scope of these commitments.** This document binds what the project builds, ships, and requires. It does not govern what an operator does with their own instance, or what a fork chooses to become. Neither is enforceable, and neither should be. A user who wires their deployment to something excluded here has not violated anything, they have exercised the licence. The commitments are about what Altair asks of people, not what people are permitted to do with it.

**For anyone picking Altair up fresh:** every Must and Won't here is implementable in any language, on any stack, with any storage and sync approach. If a technical choice makes one of them impossible, the technical choice is wrong.

**Amendments** to a Must or a Won't should be recorded as a decision, with the reasoning preserved. Supersession is normal, silent drift is not.

---

## What this document deliberately leaves out

Three kinds of question came up while writing this and were ruled out of scope. Recording them here so they are not mistaken for oversights.

**Presentation.** Where surfacing appears, what a conflict looks like on screen, whether a filter shows a count or a banner. A side-by-side diff and a line-by-line diff resolve a conflict identically. This document constrains the methodology and says nothing about the pixels.

**Sequencing.** What ships in which release, and in what order. The Musts and Won'ts hold across every version. Nothing here implies a first one.

**Thresholds reached by tuning.** How good retrieval has to be, how aggressive surfacing should be. These are settled by trying things and watching what happens, not by a number chosen in advance. Writing a threshold here would only create a commitment nobody could evaluate.

# DR-001: Markdown is the note body, not the storage layer

**Status:** Accepted
**Date:** 2026-08-04
**Supersedes:** nothing
**Related:** Altair Vision & Scope

---

## Context

Lattice, the research predecessor to Altair, stored notes as plain markdown files on disk. That worked well for what Lattice was: a single-user, single-domain knowledge system where a note was the only kind of entity and text was the only thing that mattered.

Altair commits to several things that a text file cannot express:

| Commitment | Why markdown cannot hold it |
|------------|----------------------------|
| Anything can be linked to anything | A relation from a note to an inventory item has no home in a note file |
| Backlinks derived, not maintained by hand | Requires querying all relations, not parsing every file |
| One search across all three domains | Two of the three domains are not documents |
| History is append-only where it matters | Snapshots and consumption logs are records, not revisions of a file |
| Divergent edits to different fields merge | Requires field granularity; a file is one opaque blob |
| Private by default, per entity | Visibility is metadata that must be enforced, not advisory |
| Derived data is never canonical | Extracted text must sit beside the original, not inside it |

Each of these can be forced into markdown through frontmatter conventions. Doing so produces a schema with no validation, no referential integrity, and no way to answer a cross-domain query without reading every file on disk.

The question is therefore not whether markdown is a good format. It is excellent for text. The question is whether it should also carry structure, relations, and history.

## Decision

**Markdown is the format of a note's body. It is not the storage layer.**

Concretely:

1. **The body of a note is markdown.** Text stays text. Nothing about writing changes, and the authoring experience is unaffected by this decision.
2. **Everything that is not body text lives in the structured store.** Relations, tags, visibility, snapshots, conflict state, derived text, and all Guidance and Tracking data.
3. **Export produces markdown with frontmatter**, plus the documented interchange format for everything that frontmatter cannot express. Leaving Altair always yields readable files.
4. **The store is not a directory the user edits.** There is no live external editing surface, and no directory synchronisation.

## Alternatives considered

### Markdown files as the canonical store

Files on disk are the source of truth. Everything else derives from them.

**Rejected.** It maximises portability, but it puts the substrate in direct opposition to the two hardest commitments in the vision document. Cross-domain retrieval becomes a full-corpus parse. Field-level conflict merging is impossible when the unit of change is a whole file. Snapshots become either a second copy of every file or a dependency on an external version control system. Guidance and Tracking have no natural file representation at all, so they would need a second store regardless, which forfeits the single-format benefit that motivated the choice.

### Markdown plus sidecar files or extended frontmatter

Body text in markdown, structure in adjacent files or expanded frontmatter keys.

**Rejected, and considered the worst of the three.** It creates two sources of truth that drift, with no mechanism to detect the drift. Every new capability requires a new frontmatter key that older files lack, so the format accretes optional fields indefinitely and nothing can be relied upon to be present. It carries the query cost of the file-based option and the complexity cost of the structured option, while delivering the guarantees of neither. This is the specific outcome the decision is meant to avoid.

## Consequences

**Gained**

- Relations, backlinks, and cross-domain search are ordinary queries rather than filesystem traversals
- Field-level conflict semantics become expressible, which is a Must the file-based option could not satisfy
- Snapshots and append-only logs have a natural home
- Visibility can be enforced rather than advisory
- Guidance, Knowledge, and Tracking share one substrate instead of Knowledge having a private one

**Given up**

- No editing the store directly with an external tool. Export produces a copy, not a working directory.
- Portability now depends on export being genuinely good rather than being automatic. This raises the stakes on the export and interchange-format commitments considerably.

**Obligations this creates**

- **Do not make choices that foreclose export.** This is a constraint on schema decisions, not a request to build an exporter. Discarding an original after deriving from it, storing a relation in a form that cannot be reconstructed, or dropping the provenance of imported data are all unrecoverable, and none of them announce themselves at the time.
- The interchange format, whenever it is built, must express relations, history, and visibility, since frontmatter alone cannot. Knowing that now is enough. Building it now is not required.

> ℹ️ **On timing.** Under the rejected file-based option, portability came for free. Here it is a feature, and a feature can be deferred. It reasonably should be: a single self-hosting operator holds the store already, so an exporter is worth little until there is a user who is not the person running the instance. That is the point at which this stops being a schema constraint and becomes work.

## Deliberately not decided here

- Which storage technology backs the structured store
- Whether note bodies are stored in the same place as their metadata
- The exact shape of the interchange format
- Whether markdown is normalised, restricted to a subset, or accepted as written

## Notes

This decision does not change the vision document, which is implementation-agnostic and says only that formats must be open and documented. Both the accepted and rejected options satisfy that. The choice is made on the ability to meet the Musts, not on portability, which is achievable either way.

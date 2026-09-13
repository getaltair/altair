# ADHD Personal Management System — Initial Build Definition

**Document revision:** v0.3. **Product target:** the initial v0.1 release.

**Status:** Accepted. The product behavior, scope boundaries, technical foundation, and engineering baseline in this document define the initial build.

**Initial product:** One custom, web-first application with three domain-specific workspaces, shared persistent records and relationships, and contextual recall during creation and editing.

The first usable build must connect actual work, knowledge, and resources. It must not defer cross-domain recall to a later release or replace the domain-specific interfaces with a generic object editor.

## Purpose and scope

This document is the development reference for the initial v0.1 release. It defines the product behavior, scope boundaries, usability criteria, and technical and engineering foundation.

| Requirement | Accepted initial behavior |
|---|---|
| Distinct project/task, knowledge, and resource experiences | A basic board/list, Markdown editor, and resource list/cards/detail editor |
| Shared persistent data and bidirectional relationships | Stable record identities, simple explicit links, and shared views without copies |
| Recall from current, potentially unsaved input in every domain | Text/identifier matching plus semantic retrieval; no conversational agent required |
| User-controlled connections and adjustable proactivity | Separate controls for suggestion breadth and presentation behavior |
| Optional metadata and no mandatory cleanup routine | Untitled/text-first capture, optional quantities, and no classification requirement |
| Linux, managed Windows, and Android access | Single-user, responsive browser application with central HTTPS access; online-first |

Detailed database tables, API schemas, implementation sequencing, and the development backlog are outside this document's scope.

**This is the start of the maintained product, not a disposable demonstration.** Narrow initial functionality does not require permanent code: mocks, placeholders, experiments, replacement, and substantial refactoring are normal parts of development. The engineering baseline below explains this distinction.

## Initial user and access model

**One user, one personal workspace, usable from several devices.** Shared household accounts, teams, roles, and collaboration are outside this build.

Linux and Windows use the browser. Android uses a responsive browser interface with the same core capabilities, adapted to touch and limited screen space rather than displaying a shrunken desktop layout.

Central HTTPS is the access model. No dedicated desktop client, WSL process, VPN, or synchronization agent is required by the application; authentication is required before exposing real personal data remotely.

Workplace access remains subject to the actual network and device policies. Reaching the application does not establish permission to copy business information into it, and a blocked connection does not justify circumventing those controls.

**Online-first does not mean careless about interrupted work.** The build protects drafts and handles connection failures, but does not promise a complete offline workspace, peer synchronization, or simultaneous collaborative editing.

## Records and relationships

### Domain records

| Record | Purpose | Minimum input | Optional initial information |
|---|---|---|---|
| **Project** | A continuing activity that brings work and supporting material together | A name or meaningful description | Description and linked tasks, notes, resources, files, or URLs |
| **Task** | An action to track | A title or meaningful description | Description, project associations, due date, and links |
| **Note** | Authored knowledge, unfinished writing, or a captured fragment | Text, a URL, or an attachment; no separate title required | Title and links |
| **Resource** | A tangible or intangible thing relevant to activities | A name, description, or identifying attachment | Quantity, unit, location, identifiers, details, files, URLs, and links |

A meaningful record need not have a user-authored title. The interface can display a first-line or filename label, or an untitled fallback; that label is not an AI-generated assertion about the content.

These remain different domain records. Shared identifiers, timestamps, attachments, and linking behavior do not imply identical forms or a universal user-facing object schema.

### Relationship behavior

Any domain record can be explicitly linked to another. Each endpoint exposes the connection, and renaming or editing a record does not break it.

The initial general-purpose relationship is simply **related**. The user does not have to choose a relationship type, direction, or explanation to connect two things; a resource-document link can already make a manual accessible without a relationship-taxonomy editor.

**Task/project rule:** tasks explicitly associated with a project appear on its board. A task can be standalone or associated with several projects, but it remains one task with one status wherever displayed.

Project associations and general related material must be recognizable in the interface. A related note is not a task, and a linked resource is not automatically allocated, compatible, available, or owned.

Links reuse records rather than copy them. Removing a link does not delete either endpoint, and adding an existing link again does not create a duplicate.

## Capture without classification

**Global quick capture creates an ordinary note.** The user can enter a fragment, paste a URL, or attach something without choosing a domain, project, folder, tag, or title.

This does not introduce a fourth application or a mandatory inbox-processing workflow. Once saved, the capture is normal knowledge: searchable, eligible for contextual recall, and useful indefinitely in its unfinished state.

A capture can later be linked to an existing record. An explicit “Create task from this” or “Create resource from this” action can prefill a new record while retaining the original note and linking the two; it must not silently reinterpret or delete the source.

Direct creation inside each domain remains the normal path when the user already knows what they are entering. Quick capture does not funnel task and resource work through a generic notes interface.

## Projects and tasks workspace

The initial workspace includes a project list, a task list, and a project board. Standalone tasks remain accessible without creating a placeholder project.

**Board states: To do, Doing, and Done.** New tasks default to To do; the user can change state through drag-and-drop or an equivalent menu/tap action, including on Android.

Creating a task does not require a due date, project, estimate, priority, assignee, or description. A due date, when supplied, is displayed; it does not imply a reminder or calendar subsystem in this build.

Project and task details expose supporting notes, resources, and other explicit links. Opening a project restores a useful context—its work and connected material—not just its name and an empty dashboard.

Recall operates while entering or editing project/task titles and descriptions. A new task can surface an older task or project as well as knowledge and resources; the user can continue with the existing record without silently losing the new draft.

This build does not include dependency scheduling, recurrence, custom workflow builders, subtasks as separate record hierarchies, or automatic completion gates. A Markdown checklist in a description does not create an additional workflow system.

## Knowledge workspace

The initial workspace includes note navigation, search, a Markdown-oriented editor, and a reading/preview mode. Unfinished notes are first-class content; there is no publication or completion step before they become retrievable.

Notes autosave, with visible saving/saved/error state. Recall uses the current editor buffer rather than waiting for autosave or a finished document.

Authored Markdown is the canonical note content. Editor-specific presentation or state must not be the only recoverable representation of the writing. CodeMirror 6 is the selected editor foundation; its Vim and inline Markdown presentation behavior are part of application integration, not assumptions that every interaction is provided out of the box.

**The initial reuse interaction is preview and compare, not an automatic merge engine.** The user can inspect another note, open it alongside the current work where space permits, link it, copy useful material, or resume writing in it while preserving the original draft.

On Android, comparison can use sequential previews with a reliable return path rather than two unusably narrow editors. Both notes remain intact unless the user explicitly edits or archives one.

Existing Markdown files can be imported as notes without mandatory metadata mapping. This is basic seeding of the workspace, not a full migration facility for another knowledge-management application.

## Resources workspace

The initial workspace includes a browsable/searchable list or card view and a lightweight detail editor. The same surface must accommodate a hammer, oil, several interchangeable ESP32 modules, a particular board, or a VM without requiring separate mandatory schemas.

A resource record may describe an individually identified thing or a group of interchangeable things. The user does not have to classify it into an inventory tracking mode before saving it.

### Quantities and identity

Quantity is optional. **Missing quantity means unknown or not tracked—not zero and not an assumed one.** An optional unit can distinguish boards, bottles, litres, or another user-supplied unit; automatic unit conversion is outside scope.

A potential duplicate is a suggestion, not a validation error. The user may use the existing record, update its details, record additional stock, set a corrected quantity, or retain a separate record.

“Add two” and “set the count to two” are distinct actions. An increment against an unknown starting quantity must not manufacture a total; the interface can instead accept a known total or leave it unknown.

Similarity alone does not establish interchangeability. Different variants, locations, serial numbers, or individual devices may legitimately remain separate records, even with similar names.

Resolving a duplicate must preserve the information already entered long enough to reuse or explicitly discard it. Choosing an existing resource must not silently throw away a newly entered serial number, photo, or description.

### Honest recorded state

Display quantities and locations as recorded information, not guaranteed current availability. When showing the age of a count, use when that count was recorded rather than treating an unrelated edit as physical verification.

No periodic inventory reconciliation, reservation ledger, SKU hierarchy, procurement workflow, or mandatory physical verification is required. The system remains useful when some records are incomplete or old.

## Contextual recall across all three workspaces

### Input and retrieval

Every creation/editing surface supplies its current text and relevant surrounding context: for example, the active paragraph and note title, a task title and project, or a partly entered resource description and identifier.

Saved records are the candidate material; unsaved text can drive retrieval without becoming a durable record first. **A save operation is not a prerequisite for receiving suggestions.**

Initial retrieval combines text/identifier matching with semantic matching so relevance is not limited to exact titles or tags. Existing relationships and available record details provide additional context; a manually maintained alias dictionary is not a prerequisite for usefulness.

Semantic retrieval belongs in this build because adjacent relevance is part of the accepted behavior. This does not require an autonomous agent, generated chat answers, automatic knowledge rewriting, or training a custom model.

A resource can be suggested because it may help an activity, not only because its text is nearly identical. Results may be exploratory, but their underlying records and displayed details must be real.

### Presentation and control

Suggestions update after a pause in input without blocking typing or stealing focus. Avoid changing the result under an action the user is taking, and do not replace newer context with a late response to an older query.

Each suggestion shows its record type, identifying label, useful excerpt or detail, and an understandable basis for relevance when available. Existing links are distinguishable from proposed connections; the active record is not suggested as its own duplicate.

**Suggestion controls:** suggestion breadth ranges from focused to exploratory; presentation controls separately govern automatic display/refresh and the amount shown. Broad relevance need not mean pop-ups, and hiding automatic suggestions need not disable manual recall.

Desktop can use a related-content panel. Android needs a compact, accessible entry point and touch-usable preview/actions; recall must still occur during the active workflow rather than requiring the user to leave and search separately.

Ignoring or dismissing suggestions creates no review queue or cleanup obligation. A dismissed suggestion should not immediately reappear for unchanged context, but dismissal is not automatically a permanent ban across unrelated activities.

### Actions

| Candidate | Initial actions |
|---|---|
| Any domain record | Preview, open with a return path, explicitly link, or leave unlinked |
| Earlier/overlapping note | Compare, copy relevant material, link, or continue the earlier note while preserving current work |
| Existing project/task | Inspect or continue that record rather than creating another unnecessarily |
| Possible duplicate resource | Use/update the existing record, deliberately adjust quantity, or keep a separate record |

Suggestions never create links, change counts, edit notes, or move task status on their own. Where an action relates to an unsaved new record, the application must persist the record and requested link successfully before reporting that the link exists.

Recall and editing are separable: a retrieval failure must not prevent saving or ordinary navigation. Search and direct relationships remain usable when semantic retrieval is unavailable, with a clear indication that suggestions are degraded rather than a false claim that nothing relevant exists.

## Files, search, persistence, and data safety

### Files and search

Attach basic files and URLs to records, and open or download them using ordinary supported viewing behavior. Resource photos and manuals therefore have a place without building a separate document-management system.

Initial search and recall cover authored text, record fields, attachment names, and explicit relationships. **OCR, automatic website ingestion, and semantic search inside arbitrary PDF/image attachments are not included by default.** A short note or description can make a file discoverable; an unlabeled image is not silently claimed to be understood.

Global search queries all domains and can be narrowed by domain. New or updated saved material enters retrieval automatically; the user does not maintain an indexing routine.

### Saving and interrupted work

Preserve draft content, cursor/scroll context where applicable, and a clear return path when following a suggestion or changing surfaces. Reports of successful saves and actions must reflect server acknowledgement, not optimistic UI alone.

Protect against interrupted editing and accidental navigation. Device-local recovery of unsent drafts is the personal-device default; persistent local recovery must be disableable on a managed/shared device, with its limitations made clear.

During a connection failure, retain current input, show that it is not yet saved remotely, and allow retry or copying it out. Do not promise cross-device access to a draft that has not reached the server.

Detect conflicting edits from two devices rather than silently overwriting one. Preserve the competing content for an explicit choice; real-time co-editing and automatic distributed conflict merging are not part of the build.

### Recovery, portability, and privacy

Provide reversible removal, such as archive/trash with restore, so ordinary cleanup does not immediately destroy records and links. Suggestions must not silently restore removed records; archived material, when surfaced, must be labeled as archived.

Notes export as Markdown. A workspace export includes structured records, stable IDs, relationships, and attachments so ownership is not limited to individual note files; ordinary backup and restore cover the authoritative database and files.

Retrieval indexes are derived data, not the source of truth. They must be rebuildable from the authoritative content, and removed records must not keep reappearing as actionable suggestions.

Use Authentik for identity and Garage for attachment storage. Initial inference runs on the dedicated GPU server through its OpenAI-compatible API. Sending stored content or unsaved writing to a different provider requires an explicit deployment choice; API compatibility does not imply using OpenAI-hosted services or permitting a hidden cloud fallback.

## When the initial build is usable

The following outcomes define usability criteria for the initial release. Verify them during normal development and use.

| Scenario | Required outcome |
|---|---|
| **ESP32 Clock** | While creating a project/task, recall can surface prior notes, related work, recorded boards, and potentially useful components. The user can inspect and link a useful record without losing the current entry. |
| **LLM training** | While writing, an earlier relevant draft can appear alongside other domain records. The user can compare or continue it while preserving the new work. |
| **Another ESP32 resource** | During entry, a possible existing record appears. The user can distinguish additional stock, a corrected count, and a distinct device without automatic merging or count changes. |
| **Unclassified capture** | A fragment saved without a title, tag, or project is subsequently searchable and eligible for recall without processing an inbox. |
| **Across devices and interruptions** | Saved content and links are available from Linux, Windows, and Android browsers. Preview/navigation preserves drafts; connection failures and conflicting edits do not masquerade as successful saves. |

Use a small collection of real personal material, including inconsistent terminology and plausible false matches. Relevance is judged through these working interactions and the user's breadth preference—not a requirement that every query returns all three domains or that every suggestion is accepted.

The build is incomplete if suggestions arrive only after entry is finished, if one domain cannot participate in live recall, or if retrieval depends on manually organizing the collection first.

## Deliberate boundaries

| Included now | Outside this initial build |
|---|---|
| Three appropriate domain interfaces and shared relationships | Generic page/schema/workflow builders or a graph-visualization application |
| Cross-domain live text and semantic recall | Autonomous agents, AI chat as the primary UI, automatic linking or content rewriting |
| Markdown editing, basic import, attachments, comparison | Automatic note consolidation, OCR, arbitrary attachment-content extraction, web crawling |
| Basic tasks/boards and optional due dates | Calendar sync, notifications/reminders, recurrence, dependencies, time tracking, scheduling engines |
| Optional resource counts, locations, details, and duplicate suggestions | Procurement, reservations, per-location stock ledgers, unit conversion, automatic inventory reconciliation |
| Single-user responsive HTTPS application and draft protection | Native applications, required PWA installation, P2P sync, full offline-first operation, live collaboration |
| Portable export, operational backup/restore, basic recovery | Continuous third-party app synchronization, a plugin marketplace, advanced administration |

These are scope boundaries, not statements that the excluded capabilities lack value. They should not be added indirectly as prerequisites for the included behavior.

## Accepted technical foundation

The following technologies are the selected foundation for sustained use and incremental development. They do not require every supporting mechanism to be implemented before any domain behavior.

| Area | Accepted direction |
|---|---|
| **Frontend** | Vue 3, TypeScript, and Vite; a client-rendered, responsive browser application |
| **Frontend state** | Pinia for shared client state where needed; keep editor drafts, acknowledged saved state, and server-response caches distinct |
| **Backend** | Rust with Axum, organized as one application using ordinary modules |
| **Persistence access** | SQLx, with explicit schema migrations and transactions where required by application behavior |
| **Database and retrieval** | PostgreSQL 18, pgvector, PostgreSQL full-text search, and pg_trgm; further extensions require a concrete need |
| **Knowledge editor** | CodeMirror 6 with Vim support and application-integrated inline Markdown presentation; authored Markdown remains canonical |
| **Identity and sessions** | Authentik through OIDC authorization code flow with PKCE; Axum handles the exchange and maintains an application session |
| **Browser security** | Secure, HttpOnly application-session cookies, appropriate CSRF protection, and provider tokens retained server-side |
| **Attachments** | Existing Garage S3-compatible storage for bytes; PostgreSQL for attachment metadata and relationships |
| **Inference** | Dedicated GPU server, accessed through an OpenAI-compatible API; no product-specific inference protocol |
| **Hosting** | VPS or local application server, still to be selected; central HTTPS browser access remains the baseline |

The accepted stack can evolve when actual requirements justify a change; its selection does not make individual libraries or modules permanent. Additional frameworks or service boundaries are not prerequisites for future growth.

### Deployment and responsibilities

The browser communicates with Axum for application operations and recall. Axum communicates with PostgreSQL, Garage, and the configured inference endpoint; browser use does not require direct access to the GPU server or a GPU-specific client.

Embedding inference runs on the dedicated GPU server. PostgreSQL owns persistent vectors and database-side retrieval; moving inference to the GPU does not move database queries there. Keep the database near the application server where practical; exact placement remains a deployment choice.

Server-to-server access may use a private connection or an appropriately authenticated TLS endpoint. This does not require installing a VPN, sync agent, or inference client on the business laptop. The workplace-policy limitations already stated above still apply.

A VPS-hosted application can still depend on home-hosted services. Account for the actual placement of the GPU endpoint, Garage, and Authentik when deciding which functions remain available during a home-network or service outage.

### OpenAI-compatible inference boundary

**Use the existing OpenAI API contract for the capabilities the application needs. Do not design a separate PMS AI-service API.** The initial required inference operation is embedding generation using the conventional `/v1/embeddings` endpoint and OpenAI-compatible request/response shapes.

The application configuration supplies the endpoint base URL, model identifier, and server-side credentials as applicable. Use the supported standard request fields needed for the operation; do not require the server to implement unrelated portions of the OpenAI API.

Axum should own ordinary client concerns such as request construction, result validation, timeouts, cancellation, and error handling. A focused integration module is sufficient; the contract does not justify an additional gateway, a provider registry, or a general-purpose AI orchestration framework.

The GPU server's inference runtime, model-loading implementation, and hardware management are deployment details behind that endpoint. Initial application behavior must not depend on custom model-management, scheduling, reranking, or other provider-specific endpoints or parameters.

Future product features should use existing OpenAI-compatible operations where they suffice. Expanding beyond that API requires a concrete requirement that the existing contract cannot reasonably satisfy and an explanation of the added integration burden. This is an engineering decision, not a new approval ceremony for ordinary client code.

**Transport compatibility is not an assumption that embedding models are interchangeable.** Retain the content revision, configured model identity/revision, vector dimensions, and relevant embedding configuration needed for consistent indexing and reindexing. Query and stored vectors must use a compatible embedding configuration; a same-shaped response alone must not justify mixing them.

The serving runtime and embedding model remain implementation decisions. Verify the subset of endpoint behavior the application uses through ordinary integration tests.

### Indexing and interactive recall

Saving authoritative content must not wait for GPU inference. Persist the outstanding indexing work so service failure or an application restart does not silently lose it; exact worker organization is an implementation choice, not a requirement for another service or broker.

For live recall, send the relevant current context after an appropriate pause, discard obsolete responses, and bound inference waits. Do not turn every keystroke into accumulated background work.

When inference is unavailable, preserve editing and saving and continue the available text, identifier, and relationship-based retrieval. Indicate degraded semantic recall honestly. Persistent indexing can resume when the endpoint becomes available.

### Attachment recovery

Use new object keys for replacements and retain older referenced objects according to the recovery policy. Do not make application undo or recovery depend on enabling S3 bucket versioning. This is the accepted storage policy, not a requirement for an advanced file-versioning interface.

Database/file backups and restoration must preserve their references consistently. Retention periods and final deletion behavior still need an implementation-level policy.

## Engineering baseline: durable product, replaceable code

> Build v0.1 for real use and continued extension. Protect user data, editing continuity, and the behavior the application claims to provide. Allow temporary code, placeholders, mocks, experiments, deletion, and substantial refactoring as ordinary development tools; do not confuse a maintained product with permanent implementations.

The intent is not to get every design right on the first attempt. It is to develop the selected product incrementally without making a wholesale restart the assumed prerequisite for real use.

| Establish as relevant functionality is built | Purpose |
|---|---|
| **Stable record identities and migrations** | Preserve real records and relationships while schemas and implementations evolve |
| **Explicit draft/save state and conflict handling** | Keep working input distinct from acknowledged persisted state and avoid silent cross-device loss |
| **Defined API contracts and consistent errors** | Keep Rust and TypeScript behavior aligned as the application grows |
| **Persistent indexing work and embedding provenance** | Support interruption, retries, and reindexing without treating vectors as authoritative content |
| **Recovery, portable export, and backup/restore** | Preserve user ownership and recoverability once the application contains real material |
| **Repeatable deployment and useful diagnostics** | Make configuration, migrations, logs, and failure reporting usable beyond a development session |

These are requirements for the real capabilities as delivered, not a demand for a large framework or all infrastructure to be completed up front. Introduce abstractions and additional boundaries only for accepted behavior or an actual implementation need.

### Temporary implementations and test doubles

Mock inference responses, seeded records, placeholder screens, in-memory test stores, and isolated development identity substitutes are legitimate ways to build and test incrementally. Test doubles may remain permanently in tests; they do not all need to be deleted when the live integrations work.

A simple working implementation may be shipped and later replaced or heavily refactored. Future code replacement is not itself a defect and does not require generic interfaces merely to avoid touching code later.

The distinction is between development scaffolding and a claimed usable capability. Fake suggestions do not demonstrate real contextual retrieval; an in-memory demo does not establish persistent storage; a development identity substitute is not authentication for a remotely exposed installation containing real personal data.

Keep mock/demo paths identifiable and separate from real operation. Preserve real data and accepted behavior when replacing an implementation, using migrations or compatible transitions where needed. Remove obsolete code when it is no longer needed.

### Growth without speculative infrastructure

Deferring features such as recurrence, collaboration, procurement, or full offline synchronization does not imply designing away the possibility of adding them. Equally, anticipated growth does not justify building their subsystems in advance.

Use established components and ordinary application boundaries. Do not couple canonical notes solely to editor-specific serialization, task status solely to a board component's private state, or domain behavior to GPU-server internals. These boundaries follow existing requirements; a generic repository hierarchy, plugin platform, or distributed architecture does not.

## Remaining implementation decisions

The **hosting location, inference runtime and model, exact UI and client libraries beyond those selected, detailed API/schema design, and operational retention/deployment settings** remain implementation decisions. Resolve them within the accepted product scope and technical foundation.

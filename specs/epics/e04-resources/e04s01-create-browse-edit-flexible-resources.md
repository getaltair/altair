# e04s01: Create browse and edit flexible resources

## 1. Metadata

- **ID:** e04s01
- **Type:** feat
- **Context:** domain
- **Status:** todo
- **BCPs:** 5
- **Risk:** P0
- **Epic:** e04 Resource Records

## 2. User Story

As the workspace owner, I want to create, browse, search, and edit flexible
resource records so that physical items, groups, components, and intangible
resources remain useful without mandatory classification schemas.

## 3. Problem

Resources vary from hammers and bottles to identified circuit boards and VMs.
A rigid inventory schema creates mandatory cleanup, while an unstructured blob
cannot reliably support identity, recorded location, or later recall.

## 4. Outcome

One resource surface accepts a name or meaningful description as minimum input,
while optional fields describe the resource without selecting an inventory mode.
Active resources are browsable, searchable, and safely editable.

## 5. In Scope

- Create resources with flexible name, description, location, identifiers,
  details, and optional unit.
- Require at least a name or meaningful description until e07s01 integrates
  URL and file attachment controls and attachment-backed creation.
- Browse/search active resources in list or card form and open lightweight details.
- Edit fields through the e01s03 shared `DraftSaveCoordinator`, preserving drafts
  and using its acknowledged save and conflict decisions.
- Recover unsent create/edit drafts locally by default, with a clear control to
  disable persistent local recovery on managed or shared devices.
- Show recorded locations and values as recorded state, not guaranteed availability.
- Use fallback labels from meaningful content or identifying filenames.

## 6. Out Of Scope

- Mandatory categories, SKUs, inventory modes, procurement, reservations, and stock ledgers.
- Unit conversion, automatic verification, depreciation, and availability guarantees.
- OCR, arbitrary attachment-content extraction, and automatic website ingestion.
- URL/file attachment UI and attachment-only creation before e07s01 integration.
- Quantity mutation semantics, duplicate resolution, general links, and final purge.

## 7. Domain Terms

- **Resource:** A tangible or intangible thing relevant to an activity.
- **Flexible resource:** One record shape with optional descriptive fields, not a mandatory subtype.
- **Identifier:** User-authored distinguishing text such as model or serial information.
- **Recorded location:** User-provided location that may become stale.
- **Local draft recovery:** Device-local persistence of unsent input that the user
  can disable and clear on a managed or shared device.

## 8. Preconditions

- An authenticated application session exists.
- e01 provides persistence, revision-aware saves, and typed errors.
- The responsive application shell can host list/card and detail surfaces.

## 9. Dependencies

- e01s03 supplies sessions, CSRF, migrations, generic lifecycle commands, and the
  shared `DraftSaveCoordinator`; this story defines no parallel save/conflict semantics.
- e07s01 later integrates URL/file attachment UI and attachment-backed creation;
  neither is an acceptance dependency for this story.
- e05 later adds general links and e06 supplies cross-domain recall.
- No new external package is required.

## 10. Data

- Resource: stable ID, optional name, optional description, optional location,
  identifier collection, flexible details, optional quantity/unit fields reserved
  for e04s02, lifecycle, revision, and created/updated timestamps.
- At least one of name or description must be meaningful; optional fields remain
  absent, not synthesized.
- Local recovery stores unsent create/edit state separately from acknowledged
  records and can be disabled and cleared through the managed/shared-device control.
- Updated timestamp means record edit time and must not masquerade as quantity verification time.

## 11. API

- Provide authenticated create, active-list/search, detail, and revision-checked update resources.
- Validate flexible details as bounded explicit key/value or structured fields without
  creating a user-facing generic schema builder.
- Responses contain stable ID, lifecycle, acknowledged revision, and recorded fields.
- Return typed validation, not-found, conflict, and authentication errors.
- Search matches persisted authored resource fields and identifiers.
- Resource editors adapt fields to the e01s03 `DraftSaveCoordinator` command/result
  contract rather than defining resource-specific save transitions.

## 12. UI

- Provide a browsable/searchable list or card view and lightweight detail editor.
- Keep optional fields optional and progressively reveal details without requiring classification.
- Label location and other mutable physical facts as recorded information.
- Render Saving, Saved, Error, and Conflict states from `DraftSaveCoordinator`.
- Preserve resource values and a copy-out path while the shared coordinator owns
  navigation, connection-failure, retry, and conflict decisions.
- Recover unsent local drafts by default and provide an understandable setting
  that disables and clears persistent recovery on managed or shared devices.
- Do not present URL/file attachment controls or attachment-only creation until e07s01 integration.
- Adapt cards and editor actions to touch-sized Android layouts.

## 13. Security

- Require authenticated sessions for every resource read and mutation.
- Apply CSRF protection to create and update commands.
- Validate identifiers and detail keys/values at the server boundary.
- Do not render authored text as trusted HTML or leak personal resource fields into logs.
- Treat locally recovered drafts as personal data and remove persisted drafts when
  the managed/shared-device recovery control is disabled.

## 14. Failure Modes

- Reject a resource lacking both a meaningful name and meaningful description.
- Reject oversized fields and stale revisions.
- Network, server, and stale-revision outcomes flow through `DraftSaveCoordinator`;
  the resource adapter retains every field and never reports unacknowledged success.
- Search degradation does not prevent direct browsing or editing.
- Disabling local recovery explains that unsent drafts will not survive browser/device loss.

## 15. Concurrency

- Resource editors use the e01s03 coordinator's expected-revision, idempotency,
  coalescing, and current-generation rules without defining another state machine.

## 16. Accessibility

- Fields, validation, save state, and conflicts have programmatic labels and announcements.
- Search, list/card selection, editing, retry, and conflict actions are keyboard operable.
- Card information has a logical reading order and does not rely on visual position alone.
- Touch targets and editor layouts remain usable at Android widths and zoomed text sizes.

## 17. Acceptance Criteria

#### ADDED: Create and edit flexible resource records

### Happy Path

```gherkin
Scenario: Create and edit two different kinds of resources
  Given I have an authenticated session
  When I create one resource named "Hammer" with location "Garage"
  And I create another resource described as "Build VM for model tests"
  Then both records are acknowledged with distinct stable identities
  And both appear in the same resource browse surface without mandatory categories
  When I edit the VM description using its current revision
  Then the acknowledged detail shows the edit and a newer revision
```

### Edge Case

```gherkin
Scenario: Reject an empty resource before attachment integration
  Given e07s01 has not integrated URL or file attachment controls
  When I create a resource without a meaningful name or description
  Then the resource is not saved
  And attachment-only creation is not offered as an alternative
```

```gherkin
Scenario: Preserve a resource draft after a conflicting edit
  Given another device has saved a newer revision
  When I submit my stale resource draft
  Then the server rejects silent overwrite with a conflict
  And the interface retains both my draft and the current server fields
  And it does not display Saved until I make an explicit resolution
```

```gherkin
Scenario: Disable persistent local recovery on a managed device
  Given an unsent resource draft is recoverable on this device
  When I disable persistent local draft recovery and confirm the limitation
  Then the persisted local draft is cleared
  And later input is not persisted locally while the control remains disabled
```

## 18. Test Strategy

- Domain tests cover name/description minimum input, optional fields, fallback labels,
  and no classification requirement.
- SQLx tests cover stable IDs, active filtering, search fields, revisions, and idempotent replay.
- API tests cover auth, CSRF, validation, typed errors, and conflicts.
- Component tests cover flexible forms, progressive fields, `DraftSaveCoordinator`
  adapters, field preservation, and the local-recovery disable-and-clear control.
- Playwright covers list/card browsing and desktop/Android create-edit-conflict-recovery workflows.

## 19. Implementation Notes

- Purpose: the resources domain owns flexible resource records and honest recorded-state semantics.
- Callers: Axum resource handlers, Vue resource store, browse/search surface, and detail editor.
- Contracts: stable identity, name/description minimum input, optional fields,
  active-only defaults, and the e01s03 shared draft/save contract.
- Keep resource domain types independent of Axum, SQLx, Vue, Garage, and recall payloads.
- Do not introduce resource subtype classes or a schema-builder abstraction.

## 20. Definition Of Done

- Flexible name-only and description-only resources persist and browse correctly.
- `DraftSaveCoordinator` conflict and failure paths preserve all resource input.
- Search, detail editing, responsive behavior, and accessibility have automated coverage.
- Configurable local draft recovery is verified on create and edit surfaces.
- URL/file attachment UI and attachment-only acceptance remain deferred to e07s01.
- Security boundary tests cover session, CSRF, and validation.
- `just test`, `just lint`, and `just build` pass.
- Final `just preflight` reports no new security findings in affected e04s01 paths.

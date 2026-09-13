# e04s02: Optional quantity set increment and clear

## 1. Metadata

- **ID:** e04s02
- **Type:** feat
- **Context:** domain
- **Status:** todo
- **BCPs:** 3
- **Risk:** P1
- **Epic:** e04 Resource Records

## 2. User Story

As the workspace owner, I want explicit set, increment, and clear quantity
actions so that recorded counts remain useful without treating unknown as zero
or silently converting units.

## 3. Problem

Missing quantity means unknown, not zero or one. A generic numeric update cannot
distinguish “add two” from “set total to two” and could manufacture a false total.

## 4. Outcome

Resource quantity has explicit unknown and known states. Set establishes a known
value, increment changes only a known value, and clear returns it to unknown.
Quantity/unit acknowledgements record when the count itself was changed.

## 5. In Scope

- Set a non-negative known quantity with an optional user-authored unit.
- Increment a known quantity by an explicit non-zero delta when the result is valid.
- Clear quantity and unit to unknown through an explicit action.
- Record quantity-specific revision/time independently from unrelated edits.
- Show set and increment as distinct UI actions and reject increment from unknown.

## 6. Out Of Scope

- Unit conversion, measurement normalization, reservations, per-location ledgers,
  automatic reconciliation, procurement, and physical availability guarantees.
- Automatic default quantity of zero or one.
- Automatic increments from duplicate detection.

## 7. Domain Terms

- **Unknown quantity:** Quantity is absent and no numeric total is asserted.
- **Known quantity:** Explicit user-recorded numeric amount, including valid zero.
- **Set:** Replace unknown or known quantity with a user-supplied known total.
- **Increment:** Add an explicit delta to an already known quantity.
- **Clear:** Explicitly remove quantity and unit, returning to unknown.

## 8. Preconditions

- e04s01 provides active revisioned resources and editable details.
- The user has an authenticated session.
- Numeric limits and precision are fixed consistently in domain, database, and API types.

## 9. Dependencies

- Depends on e04s01 resource identity and revisions.
- Uses e01s03 authenticated, CSRF-protected commands and shared
  `DraftSaveCoordinator` for quantity-input save, retry, and conflict decisions.
- e04s03 may invoke these commands only after an explicit duplicate choice.
- No new external package is required.

## 10. Data

- Quantity is nullable; null is unknown and numeric zero is known zero.
- Unit is nullable and acknowledged with quantity; clear sets both to null.
- Quantity revision/time changes only after acknowledged set, increment, or clear.
- Commands carry resource ID, expected resource revision, operation ID, and explicit action payload.
- Precision, maximum, and minimum bounds are represented without floating-point ambiguity.

## 11. API

- Provide separate authenticated commands for set, increment, and clear.
- Set accepts known quantity and optional unit; increment accepts delta but no unit conversion.
- Increment on unknown returns a typed `quantity_unknown` domain error with set guidance.
- All commands atomically compare expected revision and return acknowledged quantity,
  unit, quantity-recorded timestamp, and resource revision.
- Idempotent operation replay returns the original result.

## 12. UI

- Present “Set quantity,” “Add to quantity,” and “Clear quantity” as distinct actions.
- Disable or explain increment when quantity is unknown; offer Set total instead.
- Display Unknown separately from `0` and include unit only when recorded.
- Label quantity age from quantity-recorded time, never general resource update time.
- Adapt quantity input to `DraftSaveCoordinator` so entered values survive
  validation, network failure, retry, and conflict decisions.

## 13. Security

- Require authenticated session and CSRF protection for every quantity command.
- Validate numeric syntax, bounds, unit length, IDs, revisions, and operation IDs server-side.
- Do not interpolate units or numeric text into SQL or trusted HTML.
- Avoid logging resource content or user-authored units unnecessarily.

## 14. Failure Modes

- Reject increment from unknown without changing quantity.
- Reject invalid, non-finite, out-of-range, over-precision, or overflowing results.
- Route stale revisions and network failures through `DraftSaveCoordinator`, keeping
  acknowledged quantity and pending quantity input distinct.
- Clearing an already unknown quantity is idempotent and does not invent a unit.

## 15. Concurrency

- Set, increment, and clear compare and mutate resource revision atomically.
- Concurrent increments against one revision cannot lose an update: one succeeds and
  the other conflicts for explicit retry against the acknowledged total.
- Operation IDs prevent duplicate increments after a lost acknowledgement.
- Use the e01s03 coordinator's generation rule for displayed command progress.

## 16. Accessibility

- Action labels state set, add, or clear instead of relying on icons.
- Numeric and unit fields have programmatic labels, constraints, and associated errors.
- A confirmation identifies clear as changing quantity to Unknown.
- Updated quantity and errors are announced without moving focus unexpectedly.

## 17. Acceptance Criteria

#### ADDED: Explicit quantity commands preserve unknown values

### Happy Path

```gherkin
Scenario: Set increment and clear a quantity explicitly
  Given a resource has unknown quantity
  When I set its quantity to 2 with unit "boards"
  Then it shows a known quantity of "2 boards" and a quantity-recorded time
  When I add 3
  Then the acknowledged quantity is "5 boards"
  When I explicitly clear the quantity
  Then quantity and unit are unknown while the resource remains active
```

### Edge Case

```gherkin
Scenario: Do not increment an unknown quantity
  Given a resource has unknown quantity
  When I request to add 2
  Then the command is rejected as "quantity unknown"
  And no total, zero, one, or unit is manufactured
  And I am offered an explicit Set total action
```

```gherkin
Scenario: Retry a lost increment acknowledgement safely
  Given a resource has known quantity 2
  When an add-3 command commits but its acknowledgement is lost
  And I retry with the same operation ID
  Then the server returns the original acknowledged quantity 5
  And the quantity is not incremented to 8
```

## 18. Test Strategy

- Domain property/unit tests cover unknown/known transitions, zero, precision,
  bounds, overflow, unit behavior, and timestamp semantics.
- SQLx integration tests cover atomic revisions, concurrent increments, and idempotent replay.
- API tests cover auth, CSRF, validation, typed unknown errors, and stale conflicts.
- Component tests cover distinct actions, unknown display, retained input, and announcements.
- Browser tests cover set/add/clear and Android unknown-increment guidance.

## 19. Implementation Notes

- Purpose: resource quantity commands preserve the semantic difference between unknown and known.
- Callers: resource API handlers, resource store, detail editor, and explicit duplicate actions.
- Contracts: nullable unknown, known zero, no unknown increment, no unit conversion,
  atomic expected revision, idempotent commands, and quantity-specific recorded time.
- Use a decimal representation with consistent bounds across domain, SQLx, and TypeScript.
- Keep command semantics in the resources domain rather than UI button handlers.

## 20. Definition Of Done

- All valid quantity transitions and prohibited unknown increment are automated.
- Concurrent and lost-acknowledgement increments cannot double-apply or lose updates.
- UI distinguishes Unknown, zero, set, increment, and clear accessibly.
- Quantity-recorded time is independent of unrelated resource edits.
- `just test`, `just lint`, and `just build` pass.
- Final `just preflight` reports no new security findings in affected e04s02 paths.

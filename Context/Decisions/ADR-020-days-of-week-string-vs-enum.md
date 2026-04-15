# ADR-020: `days_of_week` in Routine FrequencyConfig Stored as Strings, Not Enum

## Status
Accepted

## Date
2026-04-15

## Context

`FrequencyConfig::Weekly { days_of_week: Vec<String> }` in `apps/server/server/src/guidance/routines/models.rs`
accepts any string values. The non-empty check in `FrequencyConfig::validate()` rejects an empty
vector but does not validate that the strings are valid weekday names. A request with
`{"days_of_week": ["banana"]}` passes validation and is stored in the JSONB column.

The alternative is a `DayOfWeek` enum (`Monday`..`Sunday`) which would make invalid values
unrepresentable at the type level, caught at deserialization time before any business logic runs.

Adopting a typed enum has two complications:

1. **Serialization contract change**: the JSONB stored in `guidance_routines.frequency_config`
   currently contains string values (e.g. `"monday"`, `"friday"`). Changing to an enum changes the
   wire format only if the serde representation changes; a `#[serde(rename_all = "snake_case")]`
   enum would serialize identically to the current lowercase strings, making this a safe transition
   with no migration needed.
2. **Existing data**: there is no production data at this stage — the feature is under active
   development. Migration risk is negligible.

## Decision

Defer the `DayOfWeek` enum for now. The `days_of_week: Vec<String>` pattern remains in place with
a validation-only guard (non-empty check). The change to a typed enum is captured as task S014
alongside the `RoutineStatus` enum introduction and should be addressed in the same pass.

The rationale: the `RoutineStatus` enum (S014) is already planned and would establish the same
pattern. Bundling `DayOfWeek` into that task avoids partial type-safety adoption and keeps the
models file change cohesive.

**Important**: because `#[serde(rename_all = "snake_case")]` on the enum produces the same JSON as
the current lowercase strings, no database migration is required for the JSONB column. Existing
rows (none in production) would deserialize correctly after the change.

## Consequences

**Benefits (deferral):**
- No unplanned scope expansion mid-feature.
- Changes bundled with S014 reduce file-change scatter.

**Trade-offs:**
- Invalid weekday strings (e.g. `"banana"`) accepted until S014 is implemented. The risk is low
  because the routine spawn logic reads `frequency_config` from its own writes and does not act on
  weekday values today (scheduler is a future feature).

**Risks:**
- If a scheduler is implemented before S014, it will receive unvalidated weekday strings. The
  scheduler must validate or parse the strings defensively until the enum is in place.

## Compatibility

**Checked against:** ADR-003 (sync conflict resolution), ADR-015 (DB-API type separation)

- ADR-003: Compatible — sync conflict resolution is independent of how weekday values are typed.
- ADR-015: Compatible — this ADR is about input validation, not DB/API struct separation.

No other accepted ADRs address the `FrequencyConfig` JSONB schema.

## Related
- **Feature:** Context/Features/005-GuidanceDomain/
- **Files affected:** `apps/server/server/src/guidance/routines/models.rs`
- **Follow-up task:** S014 (introduce `RoutineStatus` enum — `DayOfWeek` should be added in the same task)
- **Review finding:** P6-016 (PR-feat-guidance-domain-2026-04-15.md)

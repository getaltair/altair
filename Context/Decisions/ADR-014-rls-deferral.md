# ADR-014: Row-Level Security — Deferred to Feature 004 Sync Engine

## Status

Accepted

## Date

2026-04-13

## Context

The PostgreSQL convention (`.claude/rules/postgres.md`) requires Row-Level Security (RLS) on all user-facing tables. Feature 003 added 21 new tables, none of which have RLS policies enabled.

User isolation currently relies entirely on application-layer `WHERE user_id = $user_id` clauses in every service query. Any missed filter in a future handler would expose cross-user data.

## Decision

RLS is deliberately deferred to Feature 004 (Sync Engine). The rationale:

1. **PowerSync dependency**: Sync rules in PowerSync define bucket-based partitioning per user. The RLS policies must be consistent with the sync rules — authoring both independently would risk divergence. Deferring until Feature 004 allows both to be designed together.

2. **Application-layer enforcement**: All service functions in Feature 003 include explicit `WHERE user_id = $1` scoping. SEC-1 invariant (user data isolation) is enforced at the application layer and verified by FA-009 tests. This is not sufficient long-term but acceptable for an in-development self-hosted system.

3. **Migration path**: RLS policies will be added as migrations in Feature 004 alongside sync rule definitions. Each table will get `ALTER TABLE ... ENABLE ROW LEVEL SECURITY` and a `USING (user_id = auth.uid())` policy (or equivalent for the Altair session model).

## Consequences

- All 21 Feature 003 tables lack RLS until Feature 004.
- Any handler that omits a `user_id` filter can return cross-user data — mitigated by code review and FA-009 tests.
- Feature 004 must add RLS migrations as a P0 task, not an optional enhancement.
- This decision must be reviewed and closed when Feature 004 is complete.

## Relates To

- `.claude/rules/postgres.md` (RLS convention)
- `.claude/rules/powersync.md` (sync rules convention)
- P4-018 (review finding)

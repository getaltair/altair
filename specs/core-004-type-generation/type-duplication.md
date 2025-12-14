# Type Duplication Analysis - CORE-004

**Date**: 2025-12-12
**Status**: Documented for resolution in future spec
**Related**: CORE-004 Type Generation

---

## Summary

During Phase 5 implementation, we discovered type duplication between `altair-core` and `altair-db`. This document analyzes the duplication and recommends resolution strategies.

---

## Duplicated Types

### 1. EnergyCost Enum

**Location**: Defined in TWO places with DIFFERENT variants

#### altair-core/src/types.rs (3 variants)

```rust
pub enum EnergyCost {
    Low,
    Medium, // default
    High,
}
```

#### altair-db/src/schema/enums.rs (5 variants)

```rust
pub enum EnergyCost {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
}
```

**Status**: ⚠️ **CONFLICTING DEFINITIONS** - This is a critical issue

**Impact**:

- Type system allows two incompatible EnergyCost values
- Potential runtime bugs if wrong variant is used
- Confusing for developers (which one to import?)
- TypeScript bindings will have two different EnergyCost types

**Recommendation**: **MUST FIX** - Consolidate to single definition

- Keep altair-db version (5 variants) as canonical
- Remove altair-core version
- Update any altair-core imports to use altair-db::schema::enums::EnergyCost
- **Justification**: Database schema is source of truth, 5-variant scale matches ADHD energy management better

---

### 2. EntityStatus Enum

**Location**: Defined in TWO places with IDENTICAL variants

#### altair-core/src/types.rs

```rust
pub enum EntityStatus {
    Active,   // default
    Archived,
}
```

#### altair-db/src/schema/enums.rs

```rust
pub enum EntityStatus {
    Active,   // default
    Archived,
}
```

**Status**: ✅ **IDENTICAL DEFINITIONS** - Lower priority issue

**Impact**:

- No immediate bugs (definitions match)
- Type confusion for developers
- Unnecessary code duplication
- TypeScript bindings will have two identical EntityStatus types

**Recommendation**: **SHOULD FIX** - Consolidate to single definition

- Keep altair-db version as canonical
- Remove altair-core version
- Update altair-core imports to use altair-db::schema::enums::EntityStatus
- **Justification**: Consistency with EnergyCost resolution, database schema is source of truth

---

## Current Usage Analysis

### EnergyCost Usage

**altair-core version** used in:

- altair-core/src/types.rs (definition only)
- Unknown if imported elsewhere (requires codebase search)

**altair-db version** used in:

- altair-db/src/schema/quest.rs (Quest struct)
- altair-db/src/schema/enums.rs (definition)

### EntityStatus Usage

**altair-core version** used in:

- altair-core/src/types.rs (definition only)
- Unknown if imported elsewhere (requires codebase search)

**altair-db version** used in:

- altair-db/src/schema/\*.rs (multiple domain structs)
- altair-db/src/schema/enums.rs (definition)

---

## Resolution Strategy

### Immediate Action (CORE-004 Scope)

**For Phase 5 Task 5.2**:

1. ✅ Add specta derives to altair-core types as-is (UserId, EntityId)
2. ✅ Document this duplication issue (this file)
3. ✅ Complete CORE-004 with duplication present
4. ⚠️ Add TODO comment in altair-core/src/types.rs pointing to this document

**Rationale**:

- CORE-004 scope is "add type generation", not "refactor type hierarchy"
- Adding specta to both versions is safe (no breaking changes)
- Duplication is documented for future resolution
- Prevents scope creep and delays

---

### Future Action (Separate Spec Required)

**Create new spec** for type consolidation:

- **Spec ID**: CORE-005 or similar
- **Title**: "Type Hierarchy Consolidation"
- **Scope**:
  1. Remove EnergyCost from altair-core
  2. Remove EntityStatus from altair-core
  3. Update all altair-core imports to use altair-db types
  4. Verify no breaking changes in dependent crates
  5. Run full test suite
  6. Update documentation

**Dependencies**:

- Requires codebase-wide import analysis
- May require altair-core to depend on altair-db (or extract to shared crate)
- Risk of circular dependencies if not careful

---

## Decision Log

### Decision 1: Keep Duplication for CORE-004

**Date**: 2025-12-12
**Decision**: Leave type duplication in place for CORE-004 completion
**Rationale**:

- CORE-004 scope is type generation, not refactoring
- Safe to add specta to both definitions
- Duplication is documented and tracked
- Prevents spec scope creep

**Alternatives Considered**:

1. ❌ Fix now - Out of scope, requires dependency analysis
2. ❌ Remove altair-core types - Breaking change, needs separate spec
3. ✅ Document and defer - Balances safety with progress

---

### Decision 2: Database Schema as Source of Truth

**Date**: 2025-12-12
**Decision**: When consolidating, keep altair-db definitions as canonical
**Rationale**:

- Database schema defines domain model
- altair-db types are already used in domain structs
- Consistent with SurrealDB-first architecture
- TypeScript bindings should match database types

---

## TODO for altair-core Maintainers

**Before removing types**:

1. Run grep/ripgrep to find all EnergyCost imports from altair-core
2. Run grep/ripgrep to find all EntityStatus imports from altair-core
3. Verify altair-core can depend on altair-db (check for circular deps)
4. Consider extracting shared types to new crate (altair-types) if circular dependency exists
5. Update all imports to use canonical definitions
6. Run cargo test on entire workspace
7. Verify TypeScript bindings generate correctly

---

## References

- **Spec**: specs/core-004-type-generation/spec.md
- **Tasks**: specs/core-004-type-generation/tasks.md
- **Related Code**:
  - backend/crates/altair-core/src/types.rs
  - backend/crates/altair-db/src/schema/enums.rs

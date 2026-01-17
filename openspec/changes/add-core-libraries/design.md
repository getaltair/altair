## Context

Altair requires foundational patterns for DI, navigation, and error handling that work consistently across Android, iOS, and Desktop. ADR-009 specifies Koin, Decompose, and Arrow as the chosen libraries. This change implements Phase 1 of the implementation plan.

**Stakeholders**: All future feature development depends on these patterns.

## Goals / Non-Goals

**Goals:**
- Establish consistent DI pattern using Koin across all platforms
- Set up stack-based navigation with Decompose RootComponent
- Enable typed error handling with Arrow in shared module
- Add testing infrastructure (Mokkery, Turbine) to version catalog

**Non-Goals:**
- Implementing actual features (deferred to Phase 2+)
- Creating platform-specific DI modules (only empty AppModule for now)
- Defining all navigation destinations (only Home placeholder initially)
- Implementing repository interfaces (Phase 3)

## Decisions

### 1. Koin Module Structure

**Decision**: Single `AppModule.kt` in commonMain with platform-specific initialization.

**Rationale**: Start simple with one module; split into feature modules (guidanceModule, knowledgeModule, trackingModule) as features are added.

**Alternatives considered**:
- Separate modules per platform - rejected, would duplicate code
- Pre-structured feature modules - rejected, YAGNI until features exist

### 2. Decompose RootComponent Location

**Decision**: Place RootComponent in `composeApp/src/commonMain/kotlin/.../navigation/`.

**Rationale**: Navigation is UI-layer concern, belongs in composeApp not shared.

**Alternatives considered**:
- Shared module - rejected, navigation components depend on Compose

### 3. Arrow Optics KSP Setup

**Decision**: Add KSP plugin to shared module only, not composeApp.

**Rationale**: Domain models live in shared; optics are primarily for state updates in domain layer.

**Alternatives considered**:
- Add to both modules - rejected, composeApp can use optics from shared transitively

### 4. Config Sealed Class Design

**Decision**: Start with minimal Config sealed class containing only `Config.Home`.

**Rationale**: Other destinations (Guidance, Knowledge, Tracking, Settings) will be added as features are implemented.

```kotlin
sealed class Config {
    data object Home : Config()
    // Future: data class QuestDetail(val id: String) : Config()
}
```

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| Koin runtime DI errors | Add `checkModules()` test in commonTest |
| Decompose version compatibility with Compose | Use versions tested together per Decompose docs |
| Arrow learning curve | Document patterns in AGENTS.md (deferred) |

## Migration Plan

No migration needed - greenfield setup on existing placeholder app.

**Rollback**: Revert commit if builds fail on any platform.

## Open Questions

None - ADR-009 provides clear library choices and versions.

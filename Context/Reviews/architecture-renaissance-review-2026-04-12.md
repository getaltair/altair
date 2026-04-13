# Architecture Review: Renaissance Architecture Assessment

**Date:** 2026-04-12
**Framework:** Renaissance Architecture (first-principles, earned complexity, simplicity-as-default)
**Scope:** Full Altair project architecture
**Status:** Findings formalized into ADRs and plan updates

---

## Assessment Summary

Altair's core innovation — a cross-domain entity relationship graph with offline-first sync — is genuine and well-designed. The architecture review identified areas where infrastructure complexity was not yet earned by real constraints, and where planning specificity exceeded what's useful at this stage.

**Overall verdict:** Strong foundational design with targeted simplification opportunities. The project's genuine innovation (three-domain integration via entity relations, offline-first with conflict awareness) is sound. The issues are in surrounding infrastructure choices, not the core architecture.

---

## Findings

### 1. Zitadel OIDC: Over-Engineered for v1

**Dimension:** Simplicity as Default
**Severity:** High — affects deployment complexity, memory budget, developer experience
**Recommendation:** Replace with built-in Argon2id + server-issued JWT

**Analysis:** Zitadel is enterprise identity infrastructure solving federation, social login, and multi-tenant identity. Altair v1 serves 1-5 household members. The OIDC provider added ~200-400MB RAM, a second Postgres database, a 32-byte master key, guided first-run setup, and three security-focused ADRs (006, 009, 010) — all before a single domain feature was built.

The original architecture spec described Argon2id + JWT. ADR-006 argued for OIDC from day one to avoid migration risk. In practice, the OIDC integration itself became the migration risk — more infrastructure to maintain, more surface area for security findings, more setup friction for self-hosters.

**Decision:** Accepted. Formalized as ADR-012. Zitadel removed from Docker Compose stack.

---

### 2. RPi 4 (4GB) as Minimum Target: Premature Optimization

**Dimension:** Simplicity as Default / Anti-Cargo-Cult
**Severity:** Medium — affects development priorities
**Recommendation:** Defer 4GB target to future optimization; set 8GB as practical v1 minimum

**Analysis:** The 4GB minimum creates a constraint that shapes architecture decisions (service selection, memory budgets) before the system is functional. With MongoDB (~200-400MB) required by PowerSync and the remaining stack, 4GB is tight even after removing Zitadel. Optimizing for this target before the app works is premature.

**Decision:** Accepted. ADR-002 amended — 8GB is practical minimum, 4GB deferred to future optimization work.

---

### 3. PowerSync Exit Cost: Undocumented Lock-In

**Dimension:** Composition Mindset / Minimize Exit Costs
**Severity:** Medium — informational, no action required beyond documentation
**Recommendation:** Document the exit cost explicitly

**Analysis:** PowerSync is the correct choice — it's the only sync solution that meets all requirements (offline local writes, PostgreSQL, Web SDK, Android Kotlin SDK, self-hostable). But it has significant lock-in: proprietary sync rules, client SDKs deeply embedded in the data layer, mandatory MongoDB dependency, and a proprietary wire protocol. The mitigation (repository pattern isolating domain logic from sync plumbing) is already in place but wasn't documented.

**Decision:** Accepted. Exit cost section added to ADR-003. PowerSync stays — the exit cost is the documented price of not building a sync engine from scratch.

---

### 4. Plan Over-Specification Past Step 7

**Dimension:** Anti-Cargo-Cult / Process Over Thinking
**Severity:** Low — affects planning efficiency, not architecture
**Recommendation:** Collapse Steps 8-13 into directional summaries

**Analysis:** Steps 1-7 have clear dependencies and can be planned with confidence. Steps 8-13 (client apps, search, notifications, deployment) depend on decisions that will be made during implementation of earlier steps. Prescriptive task lists for these steps create a false sense of certainty and will require rework when earlier steps surface new constraints.

**Decision:** Accepted. Steps 8-13 in PLAN-001 collapsed to directional summaries (goals, key constraints, open questions). Full task breakdowns will be created when each step is ready for planning.

---

### 5. Cross-Domain Entity Graph: Genuine Innovation

**Dimension:** First-Principles Check
**Severity:** N/A — positive finding
**Assessment:** No action needed

Altair's core value proposition — three domains (Guidance, Knowledge, Tracking) connected through a unified entity relationship graph with offline-first sync — is a genuine architectural innovation. This isn't "todo app with notes" or "inventory tracker." The entity relation system (`entity_relations` table with `source_type`, `target_type`, `relation_type`) enables emergent cross-domain connections that most personal productivity tools handle as siloed data.

The sync-aware conflict resolution for these cross-domain relations (LWW + conflict copies for notes, stricter validation for quantities) is well-thought-out.

---

### 6. Offline-First Commitment: Well-Executed

**Dimension:** Local-First Where It Matters
**Severity:** N/A — positive finding
**Assessment:** No action needed

The offline-first architecture is implemented correctly: clients write to local SQLite, sync via PowerSync when connected, and the server is source of truth. The mutation model (operation-based with outbox), conflict detection (base_version comparison), and conflict resolution (LWW + logging, never silent data loss) align with the stated architectural constraints.

---

### 7. Feature 003 (Server Core) Planning: Needs Replanning

**Dimension:** Informational
**Severity:** Low
**Assessment:** Noted, not acted on in this review

Feature 003 planning files (Spec.md, Tech.md, Steps.md from commit `7887e51`) were in a cleaned-up worktree and don't exist on the current branch. With ADR-012 changing the auth model from OIDC to built-in Argon2id + JWT, Feature 003's auth-related tasks (OIDC relying party, JWKS validation, `oidc_sub` bootstrap) need replanning. This is separate work from this architecture review.

---

## Artifacts Produced

| Artifact | Type | Description |
|----------|------|-------------|
| ADR-012 | New | Built-in Argon2id + JWT auth replacing Zitadel |
| ADR-002 | Amended | 8GB practical minimum, RPi 4 deferred |
| ADR-003 | Updated | PowerSync exit cost section added |
| ADR-006 | Partially superseded | Identity sections replaced by ADR-012; authorization model remains |
| ADR-009 | Superseded | OIDC CSRF protection no longer applicable |
| ADR-010 | Superseded | OIDC callback token storage no longer applicable |
| PLAN-001 | Updated | Zitadel refs removed; Steps 8-13 collapsed to directional summaries |
| Docker Compose | Pending | Zitadel + postgres-init removal (implementation, not done in this review) |

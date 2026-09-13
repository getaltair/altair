# e08s02: End-to-End Release Readiness

## 1. Metadata

- **Story ID:** e08s02
- **Epic:** e08 Cross-Device Release Readiness
- **Type:** feat
- **Context:** release-readiness
- **Status:** todo
- **BCPs:** 5
- **Risk:** P0

## 2. User Story

As the workspace owner, I want the accepted v0.1 scenarios and recovery promises
proven in a production-like deployment so that I can trust Altair with real
personal work before release.

## 3. Problem

Passing isolated tests does not prove that domain workflows connect end to end,
that failures are honest, or that configured inference preserves privacy. The
release can still fail through service placement, browser integration, migration,
logging, backup, or degradation gaps that unit tests cannot expose.

## 4. Outcome

One repeatable release gate starts a production-like authenticated HTTPS
deployment, proves dependency-specific readiness and the five accepted usability
scenarios across desktop and Android-sized browsers, verifies degraded behavior
for each external service boundary, demonstrates backup restoration, and records
non-secret inference privacy evidence before any real content is sent.

## 5. In Scope

- Repeatable production build, clean migration, TLS termination, Authentik login,
  authenticated HTTPS deployment, restart, and cold-start smoke.
- Bounded production-like deployment configuration for TLS termination, Authentik,
  PostgreSQL, Garage, and exactly one inference endpoint before release tests run.
- Dependency-specific startup and readiness checks for PostgreSQL, Garage,
  Authentik, and the configured inference endpoint.
- The five accepted initial-build scenarios using realistic inconsistent terminology and plausible false matches.
- Linux/Windows desktop browser and Android-sized responsive browser coverage.
- PostgreSQL, Garage, Authentik, and inference degradation behavior.
- Inference routing, non-retention/logging, no-store, and no-fallback evidence.

## 6. Out Of Scope

- Unsupported browser certification, native applications, load testing at multi-user scale, and high availability.
- Subjective guarantee that every recall result is relevant or accepted.
- Hosting-provider provisioning, high availability, continuous monitoring, or third-party compliance certification.
- General-purpose deployment automation beyond the bounded v0.1 release harness.
- Sending personal content to any endpoint as test data.

## 7. Domain Terms

- **Accepted scenario:** One of the five usability outcomes in `docs/initial-build.md`.
- **Release fixture:** Synthetic records with cross-domain links, inconsistent terms, attachments, and plausible false matches.
- **Degraded service:** A configured dependency that is unavailable, slow, or returns an invalid response.
- **Privacy canary:** Synthetic unique text used to inspect retention/logging without personal data.
- **Release gate:** Repeatable commands and evidence that must pass before v0.1 is declared usable.
- **Bounded deployment configuration:** Versioned local/CI release-harness settings
  and orchestration for accepted dependencies, without provisioning providers or
  introducing a general deployment platform.
- **Dependency readiness:** A non-secret health result that distinguishes required
  authoritative availability from explicitly supported degraded operation.

## 8. Preconditions

- e01 through e08s01 are implemented with all story tasks passing.
- Production-like test configuration declares exactly one inference endpoint and contains no cloud fallback.
- Repeatable deployment configuration supplies test identities, trusted TLS
  termination, and isolated PostgreSQL/Garage stores without manual repair steps.
- The backup/restore rehearsal starts from a populated source and an empty compatible target.

## 9. Dependencies

- All prior epics, especially e06 recall degradation and e07 recovery integrity.
- `just test`, `just lint`, `just build`, and `just preflight` as stable project gates.
- Playwright browser workflows and deterministic infrastructure fault controls.
- Deployment-owned Authentik, Garage, PostgreSQL, and OpenAI-compatible inference configurations.
- e08s02 owns the repeatable authenticated HTTPS deployment harness, dependency
  readiness evidence, and inference privacy evidence; prior epics own domain behavior.

## 10. Data

The release fixture includes projects/tasks, unfinished Markdown, unclassified
capture, resource variants and unknown/known quantities, bidirectional links,
shared attachments, archived records, and embedding provenance. It contains no
personal data. Verification outputs record versions, scenario IDs, timestamps,
redacted endpoint ownership/configuration, checksums, and pass/fail evidence but
never credentials, raw recall requests, excerpts, cookies, or provider payloads.

## 11. API

- Exercise public application APIs only through authenticated browser or documented operational boundaries.
- Verify typed failures, CSRF rejection, secure server-owned cookies, and `Cache-Control: no-store` for recall/export responses.
- Confirm saves do not wait for inference and stale recall responses never replace current results.
- Liveness reports only process health. Readiness reports PostgreSQL as required
  and reports Garage, Authentik, and inference states separately so supported
  file, new-login, and semantic-recall degradation is visible without exposing secrets.
- Authenticated application routes are exercised only through trusted HTTPS;
  unauthenticated access redirects to or fails closed through Authentik.
- Fault injection is test/deployment control and is not exposed as a production user API.
- Deployment configuration validates required non-secret settings, rejects multiple
  inference endpoints or undeclared fallback, and injects credentials only at runtime.

## 12. UI

- Release scenarios run at `1440x900` desktop and `360x800` touch viewport; minimum-width checks come from e08s01.
- Success and failure assertions use visible user outcomes, not private component state.
- Degraded semantic recall is explicit while lexical/identifier/relationship results remain available.
- Save, conflict, attachment, archive, restore, and export claims appear only after authoritative acknowledgement.
- Every navigation away from active creation has a tested draft-preserving return path.

## 13. Security

- Verify authentication before personal data, CSRF rejection, Secure/HttpOnly cookie attributes, and no browser-visible provider credentials.
- Inspect application, proxy, and inference logs after a synthetic canary; raw canary text and response excerpts must be absent.
- Confirm only the declared inference endpoint receives the canary and no fallback request occurs during failure.
- Record endpoint owner, transport mode, and retention-disabled/self-hosted logging policy without secrets.
- Produce the required redacted inference privacy evidence before enabling real-content inference.
- Run affected-path security review and prevent export/backup artifacts from entering source or ordinary logs.

## 14. Failure Modes

- Inference unavailable/slow: saves continue; text, identifier, and relationship retrieval remain; semantic coverage is labeled degraded.
- Garage unavailable: record editing without file mutation continues; attachment upload/download fails honestly without false acknowledgement or lost draft.
- Authentik unavailable: existing unexpired application sessions follow server-owned session validity; new authentication fails closed without exposing data.
- PostgreSQL unavailable: readiness fails and mutations never claim success; current browser input remains recoverable/copyable.
- Proxy/network interruption: pending input survives, stale responses are rejected, and retry is explicit and idempotent.

## 15. Concurrency

Release tests include edits during save, lost acknowledgements, two-device
conflicts, stale/same-context recall response races, interaction-held recall
publication, project association/archive races, attachment reference/purge races,
and export during mutation. Deterministic barriers establish ordering; tests do
not rely on timing sleeps. Each assertion proves one acknowledged final state and
preservation of competing content where required.

## 16. Accessibility

- Critical scenarios run with keyboard-only navigation and visible focus assertions.
- Android scenarios validate `44x44` primary targets, status announcements, and no page overflow.
- Automated accessibility checks are supplemented by manual screen-reader checks for save, error, degraded, conflict, and completion status.
- Zoom/reflow and reduced-motion settings do not remove accepted actions or information.

## 17. Acceptance Criteria

#### ADDED: Accepted workflows operate in an authenticated production-like deployment

### Happy Path

```gherkin
Scenario: Complete the ESP32 Clock workflow
  Given prior notes, work, boards, and components use inconsistent but related terminology
  When I create an ESP32 clock project and task from current unsaved input
  Then live recall surfaces useful cross-domain candidates with understandable evidence
  And I can inspect and explicitly link one without losing the draft

Scenario: Continue earlier LLM training writing
  Given an unfinished earlier note is relevant to a new note draft
  When recall presents that note alongside other domain records
  Then I can compare or continue the earlier note
  And both authored texts and the return path preserve the new draft

Scenario: Resolve another ESP32 resource deliberately
  Given an existing ESP32 resource has a recorded quantity and identifying details
  When a possible duplicate appears during new resource entry
  Then I can use or update the existing record, explicitly adjust quantity, or keep a separate record
  And no merge or count change happens without my selected command

Scenario: Recall an unclassified capture
  Given I save a fragment without a title, tag, or project
  When I later search and create related work
  Then the capture is searchable and eligible for recall without inbox processing

Scenario: Continue across devices and interruptions
  Given saved content and links exist and another device holds an unsaved edit
  When I use desktop and Android-sized browsers through failure and conflict paths
  Then acknowledged content is available on both devices
  And unsent or competing content is preserved without false success

Scenario: Start the release deployment repeatably
  Given a clean compatible environment and declared dependency configuration
  When I run the documented production deployment and migration commands
  Then Altair starts behind trusted HTTPS and requires Authentik before application data is shown
  And readiness identifies each configured dependency without exposing credentials
```

### Edge Case

```gherkin
Scenario: Inference is unavailable during active editing
  Given the configured inference endpoint times out
  When I edit and save a record and request recall
  Then saving succeeds independently of inference
  And available non-semantic results appear with explicit degraded coverage
  And no fallback endpoint receives content

Scenario: Dependencies report distinct readiness states
  Given the application process is live
  When PostgreSQL is unavailable
  Then readiness fails because authoritative records are unavailable
  When Garage Authentik or inference is unavailable independently
  Then readiness evidence identifies the affected capability and its supported degraded behavior
  And no health response exposes endpoint credentials or personal data

Scenario: Garage fails during attachment replacement
  Given an acknowledged old attachment and an edited record draft
  When Garage becomes unavailable during replacement
  Then the old attachment and draft remain intact
  And replacement is not reported successful

Scenario: PostgreSQL fails during save
  Given my browser contains modified input
  When PostgreSQL becomes unavailable during save
  Then the application does not report Saved
  And I can retry or copy the retained input

Scenario: Privacy canary traverses inference
  Given a unique synthetic canary and one declared inference endpoint
  When semantic recall sends the canary
  Then application, proxy, and inference logs omit its raw text and response excerpts
  And the recall response is non-cacheable and no alternate endpoint is contacted

Scenario: Restore the release fixture from backup
  Given a validated backup of the complete release fixture
  When it is restored into an empty compatible deployment
  Then stable IDs, relationships, lifecycle states, and attachment checksums match
  And derived retrieval can be rebuilt without becoming authoritative
```

## 18. Test Strategy

- A release scenario matrix maps every accepted outcome to desktop, mobile, API, and persistence assertions.
- Playwright executes the five user journeys against a production build and synthetic fixture.
- Integration suites inject deterministic dependency failures and concurrency barriers.
- Deployment tests repeat clean startup, migration, trusted HTTPS access,
  Authentik login, restart, and dependency-specific readiness from documented configuration.
- Configuration tests prove the bounded harness rejects missing TLS/auth/storage
  settings, multiple inference endpoints, and undeclared fallback before startup.
- Backup restore compares canonical records, link closure, lifecycle states, and attachment checksums.
- Before real inference use, write `specs/verifications/INFERENCE_PRIVACY_LATEST.md` with the redacted evidence required by the accepted design plan.

## 19. Implementation Notes

Treat this as bounded deployment implementation plus release evidence, not a
second implementation of domain tests. Implement the production-like HTTPS harness
and validated dependency configuration before running acceptance suites. Reuse
public contracts and production configuration with deterministic fakes only
at external fault boundaries. The release harness's purpose is cross-boundary
proof; callers are local preflight and CI; contracts are reproducibility, no
personal test data, redacted evidence, and failure on any accepted-scenario gap.
It owns deployment assembly and readiness wiring but not provider provisioning.
No new dependency is proposed; any later test tool addition requires package
review and package-manager installation.

## 20. Definition Of Done

- All five accepted scenarios pass at desktop and Android reference viewports.
- A clean environment can repeatedly start, migrate, authenticate, and restart Altair behind trusted HTTPS.
- Bounded deployment configuration is implemented and rejects missing required
  settings, multiple inference endpoints, and undeclared fallback before tests run.
- Readiness evidence distinguishes PostgreSQL-required failure from declared Garage, Authentik, and inference degradation.
- Degraded inference, Garage, Authentik, PostgreSQL, and network outcomes match documented behavior.
- Attachment/archive races and complete backup restore pass with integrity checks.
- Inference privacy evidence proves declared-only routing, non-retention logging, and no-store responses before real content use.
- Ordered `just test`, `just lint`, and `just build` gates pass.
- A final `just preflight` passes with no new security findings in affected paths.

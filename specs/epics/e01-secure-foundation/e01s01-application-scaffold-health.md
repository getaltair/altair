# e01s01 Application Scaffold And Health Path

## 1. Metadata

- **ID:** e01s01
- **Type:** feat
- **Context:** infrastructure
- **Status:** todo
- **BCP:** 3
- **Risk:** P1

## 2. User Story

As the operator, I want one repeatable client/server application with a health path so that I can run and verify the foundation before domain features are added.

## 3. Problem

Altair has no executable application boundary or stable project commands yet.

## 4. Outcome

Vue/Vite and Axum start through project recipes, and the server exposes an unauthenticated liveness response that does not disclose personal or dependency state.

## 5. In Scope

- Scaffold the Vue 3 TypeScript client and one Rust Axum server.
- Add stable `just dev`, `just test`, `just lint`, `just build`, and `just preflight` recipes.
- Add a versioned health endpoint and tests.

## 6. Out Of Scope

- Domain screens, database readiness, authentication, deployment, and observability platforms.

## 7. Domain Terms

- **Liveness:** Whether the server process can answer HTTP requests.
- **Health path:** A minimal endpoint for process-level checks.

## 8. Preconditions

- Rust, Bun, and PostgreSQL-compatible development tooling can be installed by the implementation workflow.

## 9. Dependencies

- Vue 3, Vite, TypeScript, Bun, Rust, Axum, Naive UI, and XIcons are accepted `[OK]` stack dependencies.

## 10. Data

No personal or persistent domain data is created. The response contains only a stable status value.

## 11. API

- `GET /api/health` returns `200` and a typed JSON body such as `{"status":"ok"}`.
- Unsupported methods and paths use the server's consistent non-success response shape.

## 12. UI

The client renders a minimal responsive application shell and can be built independently of a live server.

## 13. Security

The health path is public but exposes no configuration, dependency addresses, credentials, versions, user data, or readiness details.

## 14. Failure Modes

- Startup configuration errors stop startup with actionable, secret-free diagnostics.
- Unknown API paths do not return the client document as a false success.

## 15. Concurrency

The stateless health handler is safe under concurrent requests and performs no external I/O.

## 16. Accessibility

The shell uses semantic landmarks, a meaningful page title, visible keyboard focus, and no desktop-only viewport assumption.

## 17. Acceptance Criteria

### Happy Path

#### ADDED: Repeatable application foundation

The accepted client/server stack builds and the stable `just` recipes provide the project interface.

```gherkin
Scenario: Health path reports a live server
  Given the Axum server is running
  When a client requests GET /api/health
  Then the response status is 200
  And the JSON status is "ok"
```

### Edge Case

```gherkin
Scenario: Health path does not accept an unsupported method
  Given the Axum server is running
  When a client requests POST /api/health
  Then the response is not successful
  And the response contains no configuration or personal data
```

## 18. Test Strategy

Use a Rust boundary test for the health route, frontend smoke/component coverage for the shell, and run all stable test, lint, and production-build recipes.

## 19. Implementation Notes

Keep one ordinary Rust application and one Vue application. The health route proves liveness only; do not add a speculative readiness subsystem or new service boundary.

## 20. Definition Of Done

- All scoped recipes are present and executable.
- Health and shell tests pass.
- `just test`, then `just lint`, then `just build` pass in that order.
- After those gates, `just preflight` reports no new security findings in the affected scaffold and health paths.

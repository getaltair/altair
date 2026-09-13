# e01s02 Authentik Session Boundary

## 1. Metadata

- **ID:** e01s02
- **Type:** feat
- **Context:** infrastructure
- **Status:** todo
- **BCP:** 5
- **Risk:** P0

## 2. User Story

As the workspace owner, I want Authentik-backed application sessions so that personal data is available only after authentication and provider credentials never reach the browser.

## 3. Problem

Remote personal data cannot be exposed safely without a server-owned identity and session boundary.

## 4. Outcome

Axum completes OIDC authorization code flow with PKCE, establishes a revocable server session, protects application APIs, and supports logout.

## 5. In Scope

- Login start, callback, authenticated-session inspection, and logout.
- Authentik OIDC discovery/validation and authorization code exchange with PKCE.
- Opaque Secure, HttpOnly, SameSite application cookie and CSRF protection for mutations.

## 6. Out Of Scope

- Teams, roles, local passwords, account administration, provider-token browser access, and additional identity providers.

## 7. Domain Terms

- **Application session:** Server-owned authenticated state referenced by an opaque cookie.
- **PKCE:** Proof key mechanism binding authorization start to callback.
- **CSRF token:** Server-verifiable proof required for cookie-authenticated state changes.

## 8. Preconditions

- e01s01 supplies the Axum boundary and configuration loading.
- An Authentik OIDC application can be configured for deployed callback URLs.

## 9. Dependencies

- Authentik, the `openidconnect` crate, and `tower-sessions` are `[OK]`; add exact compatible versions through Cargo during implementation.

## 10. Data

Persist a hashed session identifier, subject identity, creation/expiry timestamps, and revocation state. Keep provider tokens server-side and exclude them from logs and API payloads.

## 11. API

- Login start redirects to Authentik with state, nonce, and PKCE challenge.
- Callback validates state, nonce, issuer, audience, signature, code verifier, and expiry before creating a session.
- Session inspection returns only the application identity needed by the client.
- Logout revokes server state and expires the cookie.

## 12. UI

Unauthenticated navigation presents one login action. Expired or revoked sessions return to that entry without displaying protected data.

## 13. Security

Authenticate before every personal-data route. Rotate the application session at login, reject replayed/invalid callbacks, require CSRF protection on mutations, and never expose provider tokens or secrets to browser storage.

## 14. Failure Modes

- Provider outage or invalid callback returns a retryable, secret-free authentication failure without a session.
- Expired, unknown, or revoked cookies return `401`.
- Invalid or absent CSRF proof rejects mutations without changing state.

## 15. Concurrency

Session creation and revocation are atomic. Concurrent logout and mutation must resolve by authoritative session validity at mutation time.

## 16. Accessibility

Login, authentication failure, and session-expiry notices are keyboard accessible, programmatically named, and do not rely on color alone.

## 17. Acceptance Criteria

### Happy Path

#### ADDED: Authenticated application session

Only a fully validated Authentik callback creates a server-owned application session.

```gherkin
Scenario: Owner establishes an application session
  Given a login was started with state nonce and PKCE
  When Authentik returns a valid authorization code
  Then the server creates an authenticated application session
  And the browser receives an opaque Secure HttpOnly cookie
  And no provider token is returned to the browser
```

### Edge Case

```gherkin
Scenario: Callback state does not match
  Given a login was started
  When the callback contains an invalid state value
  Then no application session is created
  And protected APIs remain unauthorized

Scenario: Mutation lacks CSRF proof
  Given the owner has a valid application session
  When a state-changing request omits valid CSRF proof
  Then the request is rejected
  And no data is changed
```

## 18. Test Strategy

Use deterministic OIDC boundary doubles for success and validation failures, database integration tests for session lifecycle, and API tests for cookie flags, authorization, CSRF, logout, and token non-disclosure.

## 19. Implementation Notes

Keep Authentik payloads in an infrastructure module and expose an application-level authenticated identity. Do not invent roles for the single-user v0.1 model.

## 20. Definition Of Done

- Protected routes reject unauthenticated requests.
- Login, callback, session inspection, CSRF rejection, expiry, and logout are tested.
- Provider secrets/tokens remain server-side.
- `just test`, then `just lint`, then `just build` pass in that order.
- After those gates, `just preflight` reports no new security findings in the affected authentication and session paths.

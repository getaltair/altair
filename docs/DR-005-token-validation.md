# DR-005: The instance validates tokens rather than trusting a gateway

**Status:** Accepted
**Date:** 2026-08-15
**Supersedes:** nothing
**Related:** Altair v0 Scope, Altair Substrate Specification, Altair Component Model, Altair Architecture Foundations, DR-004

---

## Context

The scope commits to two things about access: control of it belongs to the operator's gateway, and the product ships no identity system. Neither says how the gateway proves who someone is, and forward-auth had been assumed rather than decided.

**Two things make that assumption worth reopening.** Forward-auth is at its best when the client is a browser, because the browser follows a redirect and completes a login without the application doing anything. The client set no longer contains a browser. What remains is native clients and background replay, which is the case forward-auth handles worst.

**And the substrate makes a demand of this boundary that an HTML challenge cannot satisfy.** An unreachable instance and an expired session are the same wait, waiting is silent, and a fault signals. A client cannot honour that rule if the difference between a wait and a refusal is a page of HTML arriving where a response was expected.

---

## Decision

**Authentik remains, as an OIDC provider. The instance validates the tokens it issues and reads member identity from a claim.**

1. **The instance validates; it does not authenticate.** It fetches and caches the provider's signing keys, verifies a signature, and reads claims. It holds no passwords, performs no recovery, manages no accounts, and owns no user list. Everything the no-identity-system commitment protects stays with Authentik.
2. **A membership is matched by claim.** The instance's membership record is the internal thing; the token's subject is the external key onto it. Authorship, assignment, and audience continue to reference membership and are unaffected by how someone proved who they are.
3. **An absent or expired token is a typed outcome, not a page.** It is unauthenticated, which is a wait: the outbox holds, nothing is dropped, nothing signals, and the ordinary path clears it by continuing to run. A refusal is a different outcome and signals. The distinction is carried by the protocol rather than inferred by the client.
4. **Refresh happens without a person.** Refresh tokens are long-lived so that a client returning after months replays its outbox without anyone opening a browser first. Requiring a human interaction before captures can flush would put a barrier in front of exactly the return the product exists to make free.
5. **Clients obtain tokens by whatever flow their platform makes reasonable**, which is consistent with the substrate's position that anything above the capture floor is a platform decision. The instance accepts a valid token and does not care how it was obtained.
6. **The instance's enforcement does not depend on where it sits.** It may be reachable directly without that weakening anything, because a signature check does not rely on a request having passed through a particular proxy. Audience is enforced at the instance, on the same predicate on both paths, exactly as before.

**Token lifetimes are Authentik's configuration**, not the product's, which keeps them outside the operator plane and consistent with identity staying out of the product.

---

## Alternatives considered

### Forward-auth with proxy-injected headers

**Rejected, having previously been assumed.** Three reasons, in order of weight.

A returning client cannot replay without a person. Sessions expire, and reauthentication means a human opening a browser and signing in before an outbox can drain. That is a barrier to re-entry standing directly in front of the founding scenario.

The wait-or-fault distinction becomes something a client sniffs rather than something it is told. A redirect or a login page arriving in place of a response is ambiguous by construction, and the cost of getting it wrong is captures disappearing into a login page while the interface stays silent.

And enforcement would rest on a topology assumption. Trusting an injected header requires the instance to be unreachable by any other route, which nothing in the instance can verify and which one deployment mistake breaks silently.

### Mutual TLS

**Rejected.** Enrolling a certificate on a phone is unpleasant enough to become the barrier this decision is trying to remove, and a certificate carries member identity poorly compared with a claim.

### The instance owning identity

**Rejected on the commitment.** Passwords, recovery, and account management are precisely what the product does not ship, and nothing here needs them.

### No authentication at all

**Rejected.** Capture from a phone means the instance is reachable from outside a home network in the ordinary case, and audience enforcement is meaningless without a trustworthy member identity.

---

## Consequences

**Gained**

- An expired session is a wait the protocol states rather than one the client deduces
- A client that has been away for months replays without human interaction
- Enforcement rests on a signature rather than on deployment topology
- Native clients authenticate the way their platforms already expect

**Given up**

- Token validation and key caching live in the instance, where forward-auth would have cost nothing
- Clients implement an OAuth flow rather than following a redirect

**Obligations this creates**

- **The audience predicate is only as trustworthy as the claim it rests on**, so a request whose token fails validation is unauthenticated and reaches no query surface.
- **Single-user v0 does not excuse a shortcut.** There is one membership, and the mechanism still resolves identity from a validated claim, because audience enforcement is built on member identity and retrofitting it later is worse than carrying it now.

---

## Deliberately not decided here

- Which flow each client uses, which is a platform decision per the substrate
- Whether the instance ever accepts more than one issuer
- Anything about how Authentik itself is configured, which is the operator's business

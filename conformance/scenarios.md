# Altair Outbox Conformance Scenarios

**Status:** Draft
**Date:** 2026-08-17
**Related:** Altair Substrate Specification, Altair Component Model, Altair v0 Scope, DR-004, DR-005, DR-006

---

## What this is

The outbox exists twice, in Rust and in Kotlin, and its behaviour is fully specified before either exists. These scenarios are the specification made executable. They are written once and run against every implementation, and an implementation is conforming when it passes all of them.

DR-006 states the obligation this discharges: the scenarios are written before the second implementation exists, not after it disagrees with the first.

**Every scenario observes behaviour at a boundary.** What the outbox holds internally, how it stores it, and what it is called are not conformance concerns. What a person sees, what crosses the wire, and what survives a kill are.

**Two boundaries are observed.** The person's, meaning what the client shows and when. And the instance's, meaning what intents arrive, in what order, carrying what identities. A test harness needs a controllable instance that can accept, refuse, stall, and drop connections, and a way to kill the client process without warning.

**Where a scenario says the client shows something**, that covers whatever the platform's surface is. A terminal client and an Android client show a fault differently and both are conforming. What is not permitted is showing nothing.

---

## Vocabulary

**Accepted** means the capture is durable locally and the person has been told it is theirs. Nothing about the network or credentials participates in this.

**Waiting** is a condition the ordinary path clears by continuing to run. An unreachable instance and an expired session are the same wait.

**Fault** is a condition that will not clear that way. A condition not known to be self-clearing is a fault, because a wrong signal costs attention and a wrong silence costs data.

**Intent** is one durable outbox item, carrying its own identity distinct from any entity's.

---

## A. Acceptance

**A1. Capture with the instance unreachable is accepted.**
Given the instance is unreachable.
When the person captures an entity.
Then the client shows it as theirs, and the entity is readable on the client immediately.
And nothing about the unreachability is shown at the moment of capture.

**A2. Capture with an expired session is accepted.**
Given the client holds a token the instance would reject.
When the person captures an entity.
Then acceptance is identical to A1 in every observable respect.

**A3. Capture with no household binding is accepted, where the client offers it.**
Given the client has never been signed in.
When the person captures an entity.
Then the entity is accepted and durable, with no author.
And on binding, the author is set and the intent sends.
This scenario applies only to a client that offers unbound capture. A client that requires binding first is conforming and skips it.

**A4. Acceptance is refused when local storage cannot hold the capture.**
Given local storage is full or unwritable.
When the person captures an entity.
Then the client refuses, states the condition at the moment of the attempt, and shows nothing as accepted.
And the entity does not appear anywhere as though it were held.

**A5. An entity with nothing but an identity, a type, and a timestamp is accepted.**
When the person captures an entity carrying no title and no content.
Then it is accepted and it sends.
And no field is required at any point on this path.

---

## B. Durability

**B1. An accepted capture survives a kill.**
Given an entity has been accepted and not yet sent.
When the client process is killed without warning and restarted.
Then the entity is present, still unsent, and sends when the instance becomes reachable.

**B2. An accepted capture survives a restart of the device.**
As B1, with a full restart rather than a process kill.

**B3. The queue survives alongside the entities it references.**
Given several entities have been accepted and not sent.
When the client is killed and restarted.
Then every intent is present, and every entity each intent refers to is present.
Neither exists without the other at any point observable after restart.

**B4. Acceptance is shown only once the capture is durable.**
When the person captures an entity.
Then the client shows acceptance no earlier than the point at which a kill immediately afterwards would leave the entity present.
A harness verifies this by killing the client at the moment acceptance is shown, on repeated runs.

**B5. A body is durable before its capture is accepted.**
Given the person captures a file.
Then acceptance covers the bytes, and a kill immediately after acceptance leaves both the intent and the bytes present.

---

## C. Ordering

**C1. A create precedes its own edits.**
Given an entity is created and then edited twice, all while the instance is unreachable.
When the instance becomes reachable.
Then the instance receives the create before either edit, and the first edit before the second.

**C2. Ordering is per entity, not global.**
Given entity A is created, then entity B is created, then A is edited, all while unreachable.
When the instance becomes reachable.
Then A's create precedes A's edit.
And nothing is required about where B's create falls relative to either.

**C3. A body precedes the intent that refers to it.**
Given the person captures a file.
When the client sends.
Then the body stream completes before the intent naming that body arrives.

**C4. A queued edit is sent against the counter the instance last acknowledged.**
Given an entity is created and then edited twice while the instance is unreachable, so both edits were composed against the same counter.
When the instance becomes reachable.
Then every edit that arrives for that entity carries the counter the instance acknowledged for the previous write to it, rather than the counter the edit was composed against.

---

## D. Idempotence

**D1. A lost acknowledgement produces one entity, not two.**
Given an intent is sent and the connection drops after the instance committed and before the acknowledgement arrived.
When the client retries.
Then it resubmits the same intent identity, unchanged.
And the instance produces one entity.
And the client accepts the returned acknowledgement as final.

**D2. An intent identity is generated once.**
Given an intent has been created.
When it is retried any number of times, across any number of restarts.
Then its identity is the value it was given when it was created.
This is the single most likely point of divergence between two implementations, which is why it is stated separately from D1.

**D3. Replaying an intent the instance already holds is not a fault.**
Given the instance already holds the acknowledgement for an intent.
When the client submits it again.
Then the instance returns the original acknowledgement unchanged.
And the client treats it as it would have treated the first, showing no fault and no duplicate.

**D4. Replay after an absence produces no duplicates.**
Given a client has been away long enough that some of its intents arrived and their acknowledgements did not.
When it returns and replays its whole outbox.
Then the household holds one of each entity.

---

## E. Non-blocking

**E1. A stalled send does not gate the interface.**
Given a send is in progress and the instance never responds.
When the person captures.
Then acceptance happens at the same speed as it would with nothing in flight.
And nothing in the client is unavailable while the send hangs.

**E2. A faulted item does not stop unrelated items.**
Given an intent for entity A is refused.
When there are also intents for entity B waiting.
Then B's intents send.

**E3. A faulted item stops later intents for the same entity.**
Given a create for entity A is refused.
When an edit to A is waiting behind it.
Then the edit is not sent.
And it is retained rather than discarded.

**E4. A refused intent is retained and not retried.**
Given an intent is refused.
Then it stays in the outbox, and the client does not resubmit it on its own.
Nothing is dropped, and nothing is errored away.

---

## F. Silence and signalling

**F1. Depth is never reported.**
Given any number of intents are waiting.
Then no badge, count, banner, or other indication of how many reaches the person.
This holds at every depth, including after a three week absence.

**F2. An unreachable instance is silent.**
Given the instance is unreachable and intents are waiting.
Then nothing is signalled.

**F3. An expired session is silent.**
Given the session has expired and intents are waiting.
Then nothing is signalled, and the client refreshes without a person.
Per DR-005, this is a wait and not a fault, and it clears by the ordinary path continuing to run.

**F4. A refusal is signalled.**
Given the instance refuses an intent.
Then the client signals a fault.

**F5. A local durability failure is signalled at the moment of the attempt.**
Covered by A4, and repeated here because it is the one condition that reaches the guarantee rather than delaying it.

**F6. An unrecognised condition is signalled.**
Given a condition the client cannot classify as self-clearing.
Then it is treated as a fault and signalled.

**F7. A signal may quantify the fault and never the backlog.**
Given three intents were refused and forty are waiting.
Then a signal may say three.
And nothing may say forty.

**F8. A signal clears when its condition clears.**
Given a fault is signalled and its condition then ceases.
Then the signal goes, with nothing for the person to acknowledge or dismiss.

**F9. A signal does not escalate, repeat, or age.**
Given a fault has been signalled and its condition persists for three weeks.
When the person returns.
Then they find one current statement.
And nothing says how long it has been true, and nothing has repeated in the interval.

---

## G. Outcomes the instance returns

**G1. Partial success leaves the successful part captured.**
Given two hundred intents are submitted and the hundredth is refused.
Then ninety nine are applied.
And the client handles each acknowledgement on its own terms rather than treating the submission as one unit.

**G2. A retained conflict is not a failure.**
Given an edit is applied and its acknowledgement carries a retained conflict.
Then the client treats the intent as done and removes it from the outbox.
And nothing is retried, and nothing is signalled as a fault.

**G3. A recreation is carried, not lost.**
Given an edit is sent for an entity the household has erased.
Then the acknowledgement names a new identity.
And the client's local entity adopts it, and the person's work is present under the new identity.
And this is not signalled as a fault, because nothing was lost.

**G4. A refusal reveals nothing about existence.**
Given an intent is refused as not available.
Then the client shows the same thing whether the entity is absent or invisible to this member, because nothing distinguishes them.

---

## What these scenarios decide that the documents do not

Four behaviours follow from combining requirements rather than from any single statement, and they are recorded here because a conformance suite has to be decidable.

**C4, rebasing a queued edit.** Ordered per entity and a base counter that detects concurrency meet when a person edits the same entity twice while the instance is unreachable. Both edits are composed before either acknowledgement returns, so both name the counter the client last knew. Sent as composed, the second arrives stale against the first and touches the same part, and the substrate's rule for that is to retain both values — a person conflicting with themselves over a sequence they performed in a known order, on a single device, with nobody else involved. The client is the only party that knows that order, so carrying it is its obligation. Coalescing the two into one intent satisfies this equally; what is not permitted is sending a counter the client has already been told is superseded.

**E3, holding later intents for a faulted entity.** Ordered per entity and non-blocking are both required, and they meet when a create is refused and an edit is queued behind it. Sending the edit would break ordering and would name an entity the instance does not hold. Holding it is what these scenarios require.

**E4, retention without retry.** A refusal will not clear by continuing to run, so retrying is pointless, and the v0 scope forbids dropping or erroring away a capture. Retained and not retried is the only remaining behaviour.

**G3, adopting a recreated identity.** The proto states the instance returns a new identity and the substrate states identity is stable from the moment anything refers to it. A client that held the old identity locally has to move to the new one, and these scenarios require it to do so silently.

---

## Deliberately not covered

- **Instance behaviour.** These scenarios constrain the outbox. Where they state what the instance does, it is to establish the condition being tested.
- **Transport.** Retry intervals, backoff, and connection handling are implementation choices with no observable consequence these scenarios can see.
- **Anything above the floor.** A client that offers offline editing owes the same guarantees for it, and the scenarios covering edits apply where a client offers them. A capture-only client skips them and is conforming.
- **The cache.** It is derived and discardable, and no guarantee attaches to it.

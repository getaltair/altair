# altair-conformance

The outbox conformance suite: `conformance/scenarios.md`, made executable.

## This suite is expected to fail. Do not delete it, and do not "fix" it

The outbox does not exist yet. Wave 4.1 writes it. Wave 1.5 wrote the scenarios that judge it, because DR-006's obligation is that they exist before the *second* implementation — and they may as well exist before the first.

Every scenario runs against a stub client, `altair-null-client`, which implements nothing. Each one drives that process through its steps and then fails on the behaviour it is about. **A red suite is the deliverable.** As Wave 4.1 lands behaviour, scenarios turn green one at a time, and that gradient is the whole value. Anything that makes a scenario pass without an outbox behind it destroys it.

## Running it

The scenarios sit behind the `run-conformance` feature, off by default, so `cargo test --workspace` stays green and `main` stays mergeable. The red is one command away and is meant to be looked at:

```bash
mise run conformance
```

CI runs the same thing in a job named `conformance (red until Wave 4.1)`, marked `continue-on-error`, so the red is visible on every pull request without blocking a merge.

What runs in the *default* suite, and is expected to be green:

- `tests/coverage.rs` — every scenario in the document has exactly one test, every test names a real scenario, and each test is named for its id.
- `tests/harness.rs` — the fake instance's behaviours, the process channel, and the difference between a skip and a pass.

If `tests/harness.rs` goes red, the harness is broken. If `tests/scenarios.rs` goes red, that is the expected state until Wave 4.1.

## What Wave 4.1 changes

One function, in `tests/scenarios.rs`:

```rust
fn client_under_test() -> Arc<dyn ClientUnderTest> {
    Arc::new(NullClient::at(env!("CARGO_BIN_EXE_altair-null-client")))
}
```

Point it at an adapter over the terminal client's outbox and the suite starts judging the real thing. The adapter is a process that speaks one newline-delimited JSON channel — an `Action` in on stdin, a `Reply` out on stdout — and it is a separate operating system process because sections B and F kill it without warning.

## Two boundaries, and nothing else

`conformance/scenarios.md` observes what the person sees and what reaches the instance, and says in as many words that what the outbox holds internally, how it stores it, and what it is called are not conformance concerns. Nothing in this crate reads the client's storage or its queue. If a scenario appears to need that, the boundary has been modelled wrong.

## The Kotlin mirror

DR-006 says the outbox is implemented twice and the scenarios are written once. The harness, therefore, is implemented twice as well, and the second time it should be transcription rather than reinterpretation. That is why the fake instance's control surface is one flat enum and two entry points, why the client channel is line-delimited JSON, and why there is no builder and no generic anywhere a transcriber would have to interpret one.

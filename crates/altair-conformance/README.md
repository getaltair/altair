# altair-conformance

The outbox conformance suite: `conformance/scenarios.md`, made executable.

## What this suite is, and what it was

The outbox is Wave 4.1's work and it exists. Wave 1.5 wrote the scenarios that judge it first, because DR-006's obligation is that they exist before the *second* implementation — and they may as well exist before the first.

**Until Wave 4.1 landed, every scenario ran against a stub client that implemented nothing, and a red suite was the deliverable.** That is over. The suite now runs against the terminal client's own binary and is a real gate: a scenario going red is a client that stopped conforming, not a wave that has not happened yet.

## Running it

The scenarios sit behind the `run-conformance` feature, off by default. They launch a client process per scenario and drive it for seconds at a time, which is not something `cargo test --workspace` should carry:

```bash
mise run conformance
```

That task builds the client binary first. It has to: the client is a different package, so cargo will not build it as a side effect of building these tests, and the suite finds it by path rather than through `CARGO_BIN_EXE_*`.

CI runs the same scenarios twice, on Linux and on Windows. **The Windows job is the valuable one**, because it is the one that proves durable local acceptance under a kill, on the platform where a kill is least like the one the scenarios were written against.

libtest has no third verdict, so a scenario a client legitimately skips — the substrate puts everything above the capture floor at the client's discretion — would otherwise be reported `ok` next to one that passed. The task prints a ledger at the end saying which each was, and anything absent from it failed.

What runs in the *default* suite:

- `tests/coverage.rs` — every scenario in the document has exactly one test, every test names a real scenario, and each test is named for its id.
- `tests/harness.rs` — the fake instance's behaviours, the process channel, and the difference between a skip and a pass. This is where `NullClient` still earns its keep: a client that answers everything with "nothing happened" is how the harness's own machinery is exercised without a real client in the way.

If `tests/harness.rs` goes red, the harness is broken. If `tests/scenarios.rs` goes red, the client is.

## What is skipped, and why that is not a pass

**A3, capture with no household binding.** Its condition is a device that has never been signed in. Every client this harness launches is handed a token in its environment, and the terminal client binds from that token, so there is no launch it could read as "never bound" — the scenario cannot arise for this client and it says so in the ledger rather than being faked green. A client whose binding is a separate act, as the Android one will be, declares `unbound_capture` and runs it.

**A4 and F5 on Windows.** Both need local storage the client cannot write to. On unix that is a mode bit; on Windows it is a deny entry in the directory's access control list, which this harness does not yet write. `World::make_state_unwritable` answers whether it could impose the condition, and the two scenarios skip where it could not. Thirty-three scenarios need nothing platform-specific.

## The client under test

**The adapter is a hidden mode of the client's own binary, not a lookalike.** `--conformance-adapter` makes `altair` speak this channel instead of drawing screens, over exactly the same store and the same outbox. A separate adapter binary would be a second implementation of local acceptance, and this suite would then be judging something shaped like the client rather than the client.

The channel is one newline-delimited JSON stream — an `Action` in on stdin, a `Reply` out on stdout — and the client is a separate operating system process because sections B and F kill it without warning.

## Two boundaries, and nothing else

`conformance/scenarios.md` observes what the person sees and what reaches the instance, and says in as many words that what the outbox holds internally, how it stores it, and what it is called are not conformance concerns. Nothing in this crate reads the client's storage or its queue. If a scenario appears to need that, the boundary has been modelled wrong.

## The Kotlin mirror

DR-006 says the outbox is implemented twice and the scenarios are written once. The harness, therefore, is implemented twice as well, and the second time it should be transcription rather than reinterpretation. That is why the fake instance's control surface is one flat enum and two entry points, why the client channel is line-delimited JSON, and why there is no builder and no generic anywhere a transcriber would have to interpret one.

## InFlight `inspect` Deferred

**Invariant.** `inspect(|c| ...)` on an InFlight carrier defers
the closure until the chain's pending future is awaited. Before
`.await`, the closure has not run; after `.await`, the closure has
run against a temporary Resolved view of the awaited state.

**Failure mode this guards.** Three failure modes:

1. **Inspect runs eagerly on InFlight.** The closure would fire
   before `.await`, defeating the "observe the resolved state" use
   case (e.g. for logging tag IDs that don't exist until the
   pending future resolves).
2. **Inspect never runs.** A buggy codegen could generate the
   InFlight arm as a no-op, dropping the closure on the floor.
3. **Inspect runs against wrong state.** If the closure observed an
   intermediate state instead of the resolved one, the test's
   assertion on `tag=1` would fail or surface a different value.

**Setup.** Fresh `Author<Drafted, Resolved>` then `tag()` (lifts to
InFlight). `inspect(move |c| ...)` adds a deferred closure that
records the carrier's name and tag into a `Mutex<Vec<String>>`. The
test asserts the log is empty *before* `.await`. Then `.await` and
asserts the log has exactly one entry with the expected
post-resolve values.

**Assertion.**

- Before `.await`: log is empty.
- After `.await`: log == `["inspected name=gamma tag=1"]`. Resolved
  carrier has `name == "gamma"` and `tag == 1`.

### inflight_inspect_runs_after_pending_resolves

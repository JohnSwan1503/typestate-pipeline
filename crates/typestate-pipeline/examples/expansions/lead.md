If you've used `Option<T>` fields and a `.build().unwrap()` to enforce
"every required field is set," you've already met the runtime check
this crate replaces. If you've written a multi-phase async workflow
where the chain came apart at every `.await?` into a string of
`let`-bindings, you've met the second one. Both have the same answer:
encode the state in the *type* of the value rather than in a separate
boolean or in the call site's discipline. The compiler then refuses
the bad program before it runs.

This guide walks the macros that do that encoding, end to end. Each
section pairs runnable source — the same files the integration test
suite compiles — with a sketch of the surface the macro emits, so you
can see what's expanded without running `cargo expand`.

## What the typestate pattern is doing here

A *typestate* is a type-level encoding of an object's logical state.
Instead of a runtime field like `is_initialized: bool`, the state lives
in the *generic parameters* of the type itself. A method's
preconditions become bounds on those generics. The compiler's
overload-resolution rules decide which methods are callable on the
current shape. There is no runtime check anywhere in that decision.

This crate uses the pattern in two places. The first is *building
values*: `#[derive(TypestateFactory)]` synthesizes a "named-field
accumulator" with one flag per field, where setters are callable
only when the field is unset and `finalize()` only when every
required field is set. The second is *advancing values through
phases*: `#[transitions]` lifts a plain `impl` block on a carrier
into a phase machine where each method consumes one phase type
and produces another. Two more macros — `pipelined!` and
`impl_pipelined!` — declare the carrier on which `#[transitions]`
operates.

The flags themselves are two zero-sized types — [`Yes`] and [`No`] —
that the compiler erases entirely. Every "transition" is a
`self`-consuming move that produces a new type; the runtime cost of
the typestate machinery is zero. What you pay for is exactly what
every Rust program pays for: the compiler's analysis at build time.

[`Yes`]: crate::Yes
[`No`]: crate::No

## How to read this guide

Each section assumes the previous one. The factory comes first because
it's the simplest introduction to "flag generics decide what compiles."
Then `#[transitions]`, which applies the same idea to phase advancement
on a carrier. Then the carrier macros (`pipelined!` and
`impl_pipelined!`) that produce the carrier `#[transitions]` operates
on. The last section shows the combinations that come up in real
codebases.

Every section heading is anchorable.

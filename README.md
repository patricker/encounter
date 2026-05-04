# encounter

[![crates.io](https://img.shields.io/crates/v/encounter.svg)](https://crates.io/crates/encounter)
[![docs.rs](https://img.shields.io/docsrs/encounter)](https://docs.rs/encounter)
[![license](https://img.shields.io/crates/l/encounter.svg)](#license)

`encounter` resolves what happens when several characters interact in a scene. Give it the *what could happen* (a catalog of possible actions) and the *who is present* (the characters); it picks one of three protocols (one-shot exchange, turn-by-turn scene, or long-running scheme) and returns a structured *what happened* — beats and typed effects, replayable and testable. It does not generate prose, run a drama manager, or decide policy — those live in the layer above.

```toml
[dependencies]
encounter = "0.1"
```

## Quickstart

```rust
use encounter::resolution::SingleExchange;
use encounter::scoring::{AlwaysAccept, ScoredAffordance};
use encounter::affordance::{AffordanceSpec, CatalogEntry};
use encounter::types::Effect;

let entry = CatalogEntry {
    spec: AffordanceSpec {
        name: "greet".into(),
        domain: "social".into(),
        bindings: vec!["self".into(), "target".into()],
        considerations: Vec::new(),
        effects_on_accept: vec![Effect::RelationshipDelta {
            axis: "friendship".into(),
            from: "target".into(),
            to: "self".into(),
            delta: 0.05,
        }],
        effects_on_reject: Vec::new(),
        drive_alignment: Vec::new(),
    },
    precondition: String::new(),
};
let scored = ScoredAffordance {
    entry,
    score: 0.8,
    bindings: [("self".into(), "alice".into()), ("target".into(), "bob".into())]
        .into_iter()
        .collect(),
};

let result = SingleExchange.resolve("alice", "bob", &[scored], &AlwaysAccept);
assert!(result.beats[0].accepted);
```

Runnable versions of all three protocols live in [`examples/`](./examples).

## The three protocols

- **`SingleExchange`** — one initiator picks an action, the responder accepts or rejects, the scene ends in a single beat. For one-shot dramatic moments.
- **`MultiBeat`** — turn-based scene; participants cycle and scoring is recomputed each beat, so mid-scene world changes affect later choices.
- **`BackgroundScheme`** — long-running plot that accumulates progress over many ticks, then resolves to one consequential beat.

## Where you plug in

encounter depends on `serde`, `toml`, and `thiserror` — and nothing else. The two pieces of consumer policy it pushes out as traits:

- **`ActionScorer<P>`** — given an actor and the actions available to them, returns each action with a utility score. This is where your scoring policy lives — a utility/salience model, a GOAP planner, or an LLM call.
- **`AcceptanceEval<P>`** — given a responder and a scored action, returns true if they accept. This is where your fabula evaluator, reaction model, or argumentation backend lives. *Fabula* here is the precondition language for action availability — typically a small DSL the bridge crate parses.

On each beat, the protocol asks `ActionScorer` for ranked actions and then asks `AcceptanceEval` whether the chosen action lands.

The `<P>` parameter is the precondition type. The default is `String` (raw fabula source); bridges typically substitute a typed pattern.

## Use with `argumentation`

The canonical reasoning backend is the [`argumentation`](https://crates.io/crates/argumentation) crate, via the [`encounter-argumentation`](https://crates.io/crates/encounter-argumentation) bridge. It implements `ActionScorer` and `AcceptanceEval` using a Dung-framework-style argument graph with weighted-bipolar attacks and a β-budget acceptance dial.

If you need encounter to do more than what the built-in `AlwaysAccept` / `AlwaysReject` test helpers offer, start with that bridge.

## Inspirations

encounter borrows shape from several published systems. The implementations are small, opinionated reductions — not faithful reproductions:

- **`SingleExchange`** reduces the intent/reaction step from McCoy et al., *Comme il Faut* (Game AI Pro 3, ch. 43). Full CiF social-games are out of scope.
- **`MultiBeat`** takes the speaker-rotation loop from Evans & Short, *Versu* (IEEE TCIAIG 2014). Full social-practice goal stacks, role tableaux, and obligations are out of scope.
- **`BackgroundScheme`** takes the progress-bar shape from the scheme system in Crusader Kings III ([CK3](https://ck3.paradoxwikis.com/Schemes)). Agents, discovery rolls, and counter-actions are out of scope.
- **`TurnPolicy::AdjacencyPair`** is the adjacency-pair model from Sacks, Schegloff & Jefferson, *Lectures on Conversation* (1992).

## License

Dual-licensed under either of:

- MIT license — see [LICENSE-MIT](./LICENSE-MIT)
- Apache License 2.0 — see [LICENSE-APACHE](./LICENSE-APACHE)

at your option.

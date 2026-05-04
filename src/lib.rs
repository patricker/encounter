//! # encounter
//!
//! `encounter` resolves what happens when several characters interact in a
//! scene. Give it the *what could happen* (a catalog of possible actions)
//! and the *who is present* (the characters); it picks one of three
//! protocols (one-shot exchange, turn-by-turn scene, or long-running
//! scheme) and returns a structured *what happened* — beats and typed
//! effects, replayable and testable. It does not generate prose, run a
//! drama manager, or decide policy — those live in the layer above.
//!
//! ## Architecture
//!
//! encounter is **standalone**: it depends on `serde`, `toml`, and
//! `thiserror`, nothing else. Scoring and pattern evaluation are delegated
//! to consumers via traits:
//!
//! - [`ActionScorer`] — scores available actions for an actor. This is
//!   where your scoring policy lives — a utility/salience model, a GOAP
//!   planner, or an LLM call.
//! - [`AcceptanceEval`] — determines whether a responder accepts a chosen
//!   action. This is where your fabula evaluator, reaction model, or
//!   argumentation backend lives. *Fabula* here means the precondition
//!   language encounter uses for action availability — typically a small
//!   DSL the bridge crate parses.
//!
//! On each beat, the protocol asks [`ActionScorer`] for ranked actions and
//! then asks [`AcceptanceEval`] whether the chosen action lands.
//!
//! ## Pluggable backends
//!
//! The canonical reasoning backend is the
//! [`argumentation`](https://crates.io/crates/argumentation) crate, via the
//! [`encounter-argumentation`](https://crates.io/crates/encounter-argumentation)
//! bridge. It implements both consumer traits using a Dung-framework-style
//! argument graph with weighted-bipolar attacks and a β-budget acceptance
//! dial. If you need more than the built-in [`AlwaysAccept`] / [`AlwaysReject`]
//! test helpers, start with that bridge.
//!
//! ## Quick example: SingleExchange
//!
//! ```
//! use encounter::resolution::SingleExchange;
//! use encounter::scoring::{AlwaysAccept, ScoredAffordance};
//! use encounter::affordance::{AffordanceSpec, CatalogEntry};
//! use encounter::types::Effect;
//!
//! let entry = CatalogEntry {
//!     spec: AffordanceSpec {
//!         name: "greet".into(),
//!         domain: "social".into(),
//!         bindings: vec!["self".into(), "target".into()],
//!         considerations: Vec::new(),
//!         effects_on_accept: vec![Effect::RelationshipDelta {
//!             axis: "friendship".into(),
//!             from: "target".into(),
//!             to: "self".into(),
//!             delta: 0.05,
//!         }],
//!         effects_on_reject: Vec::new(),
//!         drive_alignment: Vec::new(),
//!     },
//!     precondition: String::new(),
//! };
//! let scored = ScoredAffordance {
//!     entry,
//!     score: 0.8,
//!     bindings: [("self".into(), "alice".into()), ("target".into(), "bob".into())]
//!         .into_iter()
//!         .collect(),
//! };
//!
//! let protocol = SingleExchange;
//! let result = protocol.resolve("alice", "bob", &[scored], &AlwaysAccept);
//! assert_eq!(result.beats.len(), 1);
//! assert!(result.beats[0].accepted);
//! ```
//!
//! Runnable versions of all three protocols live in the `examples/` directory
//! of the source repository.
//!
//! ## Catalog loading
//!
//! ```no_run
//! use encounter::catalog::load_catalog_dir;
//! use std::path::Path;
//!
//! let entries = load_catalog_dir(Path::new("path/to/catalog")).unwrap();
//! println!("loaded {} affordances", entries.len());
//! ```
//!
//! ## Inspirations
//!
//! The protocols are small, opinionated reductions of shapes that have
//! shipped or been published. Each one is named for what it borrows, not
//! for what it reproduces faithfully:
//!
//! - **`SingleExchange`** reduces the intent/reaction step from McCoy et
//!   al., *Comme il Faut* (Game AI Pro 3, ch. 43). Full CiF social-games are
//!   out of scope.
//! - **`MultiBeat`** takes the speaker-rotation loop from Evans & Short,
//!   *Versu* (IEEE TCIAIG 2014). Full social-practice goal stacks, role
//!   tableaux, and obligations are out of scope.
//! - **`BackgroundScheme`** takes the progress-bar shape from the scheme
//!   system in Crusader Kings III (CK3). Agents, discovery rolls, and
//!   counter-actions are out of scope.
//! - **`TurnPolicy::AdjacencyPair`** is the adjacency-pair model from
//!   Sacks, Schegloff & Jefferson, *Lectures on Conversation* (1992).

#![deny(missing_docs)]
#![warn(clippy::all)]

pub mod affordance;
pub mod catalog;
pub mod error;
pub mod escalation;
pub mod practice;
pub mod resolution;
pub mod scoring;
pub mod types;
pub mod value_argument;

pub use affordance::{AffordanceSpec, CatalogEntry};
pub use error::Error;
pub use escalation::{EscalationRequest, FidelityHint, check_escalation};
pub use practice::{DurationPolicy, PracticeSpec, TurnPolicy};
pub use resolution::MultiBeat;
pub use resolution::SingleExchange;
pub use resolution::background::{BackgroundScheme, SchemePhase};
pub use scoring::{AcceptanceEval, ActionScorer, AlwaysAccept, AlwaysReject, ScoredAffordance};
pub use types::{Beat, ConsiderationSpec, DriveAlignment, Effect, EncounterResult};
pub use value_argument::{ValueArgumentInput, ValueArgumentResult, resolve_value_argument};

//! # encounter
//!
//! Social interaction resolution engine. Resolves multi-character encounters
//! via practice-scoped action catalogs and three resolution protocols.
//!
//! ## Architecture
//!
//! encounter is **standalone**: it depends on `serde`, `toml`, and
//! `thiserror`, nothing else. Scoring and pattern evaluation are delegated
//! to consumers via traits:
//!
//! - [`AcceptanceEval`] — determines whether a responder accepts an action.
//! - [`ActionScorer`] — scores available actions for an actor (wraps fabula
//!   sifting + salience scoring in consumer-specific bridge crates).
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
//! ## References
//!
//! - Evans & Short, *Versu* (IEEE TCIAIG 2014) — MultiBeat turn model
//! - McCoy et al., *Comme il Faut* (Game AI Pro 3) — SingleExchange
//! - CK3 interactions/schemes — BackgroundScheme
//! - Sacks, *Lectures on Conversation* (1992) — turn-taking

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
pub use resolution::background::{SchemePhase, SchemeState};
pub use scoring::{AcceptanceEval, ActionScorer, AlwaysAccept, AlwaysReject, ScoredAffordance};
pub use types::{Beat, ConsiderationSpec, DriveAlignment, Effect, EncounterResult};
pub use value_argument::{ValueArgumentInput, ValueArgumentResult, resolve_value_argument};

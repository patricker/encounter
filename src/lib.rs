//! # encounter
//!
//! Social interaction resolution engine. Resolves multi-character encounters
//! via practice-scoped action catalogs and three resolution protocols:
//!
//! 1. [`SingleExchange`] — CiF-style one-shot: initiator picks, responder
//!    accepts or rejects, effects fire.
//! 2. [`MultiBeat`] — Versu-style turn-based scene: beats continue until
//!    the practice resolves or max beats reached.
//! 3. [`BackgroundScheme`] — CK3-style long-duration plot: progress
//!    accumulates over simulation time toward a threshold.
//!
//! encounter is **standalone**: it depends on `serde`, `toml`, and
//! `thiserror`, nothing else. Pattern evaluation (fabula) and utility
//! scoring (salience) are delegated to consumers via the [`AcceptanceEval`]
//! and [`ActionScorer`] traits.

#![deny(missing_docs)]
#![warn(clippy::all)]

pub mod affordance;
pub mod error;
pub mod practice;
pub mod types;

pub use error::Error;

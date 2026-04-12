//! Scoring traits and scored affordance types for encounter resolution.
//!
//! This module provides [`ScoredAffordance`], [`AcceptanceEval`], and
//! [`ActionScorer`] — the core interfaces consumers implement to plug in
//! utility scoring and acceptance logic.

use crate::affordance::CatalogEntry;
use std::collections::HashMap;

/// An affordance that has been sifted and scored. Input to resolution protocols.
/// Generic over precondition type P (from CatalogEntry<P>). Default is String.
#[derive(Debug, Clone)]
pub struct ScoredAffordance<P = String> {
    /// The catalog entry being scored.
    pub entry: CatalogEntry<P>,
    /// The computed utility score for this affordance.
    pub score: f64,
    /// Resolved slot bindings for this affordance instance.
    pub bindings: HashMap<String, String>,
}

/// Evaluates whether a responder accepts an action.
/// Generic over P to match ScoredAffordance<P>.
pub trait AcceptanceEval<P = String>: Send + Sync {
    /// Returns true if the responder accepts the given scored action.
    fn evaluate(&self, responder: &str, action: &ScoredAffordance<P>) -> bool;
}

/// Scores available affordances for an actor.
pub trait ActionScorer<P = String>: Send + Sync {
    /// Returns a scored and ordered list of affordances available to the actor.
    fn score_actions(
        &self,
        actor: &str,
        available: &[CatalogEntry<P>],
        participants: &[String],
    ) -> Vec<ScoredAffordance<P>>;
}

/// Test helper: always accepts any action for any responder.
pub struct AlwaysAccept;

impl<P> AcceptanceEval<P> for AlwaysAccept {
    fn evaluate(&self, _responder: &str, _action: &ScoredAffordance<P>) -> bool {
        true
    }
}

/// Test helper: always rejects any action for any responder.
pub struct AlwaysReject;

impl<P> AcceptanceEval<P> for AlwaysReject {
    fn evaluate(&self, _responder: &str, _action: &ScoredAffordance<P>) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::affordance::AffordanceSpec;

    fn test_scored(name: &str, score: f64) -> ScoredAffordance<String> {
        let spec = AffordanceSpec {
            name: name.to_string(),
            domain: "test".to_string(),
            bindings: vec![],
            considerations: vec![],
            effects_on_accept: vec![],
            effects_on_reject: vec![],
            drive_alignment: vec![],
        };
        let entry = CatalogEntry {
            spec,
            precondition: String::new(),
        };
        ScoredAffordance {
            entry,
            score,
            bindings: HashMap::new(),
        }
    }

    #[test]
    fn always_accept_accepts() {
        let eval = AlwaysAccept;
        let action = test_scored("greet", 0.9);
        assert!(eval.evaluate("alice", &action));
    }

    #[test]
    fn always_reject_rejects() {
        let eval = AlwaysReject;
        let action = test_scored("threaten", 0.5);
        assert!(!eval.evaluate("bob", &action));
    }

    #[test]
    fn always_accept_works_with_unit_precondition() {
        let spec = AffordanceSpec {
            name: "wave".to_string(),
            domain: "social".to_string(),
            bindings: vec![],
            considerations: vec![],
            effects_on_accept: vec![],
            effects_on_reject: vec![],
            drive_alignment: vec![],
        };
        let entry: CatalogEntry<()> = CatalogEntry {
            spec,
            precondition: (),
        };
        let action: ScoredAffordance<()> = ScoredAffordance {
            entry,
            score: 1.0,
            bindings: HashMap::new(),
        };
        let eval = AlwaysAccept;
        assert!(eval.evaluate("carol", &action));
    }
}

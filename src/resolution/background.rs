//! BackgroundScheme resolution protocol — CK3-style long-duration plot.

use crate::types::{Beat, Effect, EncounterResult};

/// Phase of a background scheme.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchemePhase {
    /// The scheme is being set up; no progress has been made yet.
    Preparation,
    /// The scheme is actively in motion.
    Execution,
    /// The scheme has reached its threshold and is complete.
    Resolved,
}

/// State of an ongoing background scheme.
#[derive(Debug, Clone)]
pub struct SchemeState {
    /// The character running the scheme.
    pub initiator: String,
    /// The character the scheme is directed against.
    pub target: String,
    /// Identifier for the type of scheme (e.g. `"assassination"`).
    pub scheme_type: String,
    /// Accumulated progress toward the threshold.
    pub progress: f64,
    /// Progress value at which the scheme resolves.
    pub threshold: f64,
    /// Current phase of the scheme lifecycle.
    pub phase: SchemePhase,
    /// Labels describing situational advantages held by the initiator.
    pub advantages: Vec<String>,
}

impl SchemeState {
    /// Create a new scheme in the [`SchemePhase::Preparation`] phase with zero progress.
    pub fn new(initiator: String, target: String, scheme_type: String, threshold: f64) -> Self {
        Self {
            initiator,
            target,
            scheme_type,
            progress: 0.0,
            threshold,
            phase: SchemePhase::Preparation,
            advantages: Vec::new(),
        }
    }

    /// Advance progress. Returns true if scheme resolved this tick.
    pub fn advance(&mut self, delta: f64) -> bool {
        self.progress = (self.progress + delta).max(0.0);
        if self.phase == SchemePhase::Preparation && self.progress > 0.0 {
            self.phase = SchemePhase::Execution;
        }
        if self.progress >= self.threshold {
            self.phase = SchemePhase::Resolved;
            return true;
        }
        false
    }

    /// Record an advantage label for the initiator.
    pub fn add_advantage(&mut self, label: String) {
        self.advantages.push(label);
    }

    /// Convert resolved scheme to EncounterResult with one beat.
    pub fn to_result(
        &self,
        success_effects: Vec<Effect>,
        failure_effects: Vec<Effect>,
    ) -> EncounterResult {
        let success = self.phase == SchemePhase::Resolved;
        let effects = if success {
            success_effects
        } else {
            failure_effects
        };
        let mut result = EncounterResult::new(
            vec![self.initiator.clone(), self.target.clone()],
            Some(self.scheme_type.clone()),
        );
        let beat = Beat {
            actor: self.initiator.clone(),
            action: format!("{}_resolution", self.scheme_type),
            accepted: success,
            effects,
        };
        result.push_beat(beat);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Effect;

    #[test]
    fn scheme_starts_in_preparation() {
        let state = SchemeState::new(
            "alice".to_string(),
            "bob".to_string(),
            "assassination".to_string(),
            10.0,
        );
        assert_eq!(state.phase, SchemePhase::Preparation);
        assert_eq!(state.progress, 0.0);
    }

    #[test]
    fn advance_transitions_to_execution() {
        let mut state = SchemeState::new(
            "alice".to_string(),
            "bob".to_string(),
            "assassination".to_string(),
            10.0,
        );
        let resolved = state.advance(2.0);
        assert!(!resolved);
        assert_eq!(state.phase, SchemePhase::Execution);
        assert!((state.progress - 2.0).abs() < f64::EPSILON);
    }

    #[test]
    fn advance_resolves_at_threshold() {
        let mut state = SchemeState::new(
            "alice".to_string(),
            "bob".to_string(),
            "blackmail".to_string(),
            10.0,
        );
        let first = state.advance(5.0);
        assert!(!first);
        assert_eq!(state.phase, SchemePhase::Execution);

        let second = state.advance(5.0);
        assert!(second);
        assert_eq!(state.phase, SchemePhase::Resolved);
    }

    #[test]
    fn setback_cannot_go_below_zero() {
        let mut state = SchemeState::new(
            "alice".to_string(),
            "bob".to_string(),
            "seduction".to_string(),
            10.0,
        );
        state.advance(3.0);
        state.advance(-5.0);
        assert_eq!(state.progress, 0.0);
    }

    #[test]
    fn to_result_produces_one_beat() {
        let mut state = SchemeState::new(
            "alice".to_string(),
            "bob".to_string(),
            "spy_ring".to_string(),
            5.0,
        );
        state.advance(5.0);
        assert_eq!(state.phase, SchemePhase::Resolved);

        let success_effects = vec![Effect::RelationshipDelta {
            axis: "trust".to_string(),
            from: "alice".to_string(),
            to: "bob".to_string(),
            delta: -0.5,
        }];
        let failure_effects = vec![];

        let result = state.to_result(success_effects, failure_effects);
        assert_eq!(result.beats.len(), 1);
        assert!(result.beats[0].accepted);
        assert_eq!(result.beats[0].effects.len(), 1);
        assert_eq!(result.relationship_deltas.len(), 1);
    }
}

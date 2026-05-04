//! Escalation request types and detection logic.

use crate::types::Beat;
use serde::Serialize;

/// A request to escalate scene fidelity, emitted during a multi-beat encounter.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct EscalationRequest {
    /// What triggered the escalation.
    pub trigger: String,
    /// The beat index that triggered it.
    pub beat_index: usize,
    /// Suggested fidelity bump.
    pub suggested_fidelity: FidelityHint,
}

/// Advisory fidelity level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum FidelityHint {
    /// No change in fidelity recommended.
    Unchanged,
    /// The scene should become fidelity-aware.
    Aware,
    /// Full active fidelity is recommended.
    Active,
}

/// Check a beat for escalation-worthy conditions.
/// Fires on relationship deltas >= 0.4 or emotion intensity >= 0.6.
pub fn check_escalation(beat: &Beat, beat_index: usize) -> Option<EscalationRequest> {
    use crate::types::Effect;

    let mut max_rel_delta: f64 = 0.0;
    let mut max_emotion_intensity: f64 = 0.0;

    for effect in &beat.effects {
        match effect {
            Effect::RelationshipDelta { delta, .. } => {
                max_rel_delta = max_rel_delta.max(delta.abs());
            }
            Effect::EmotionalEvent { intensity, .. } => {
                max_emotion_intensity = max_emotion_intensity.max(*intensity);
            }
            _ => {}
        }
    }

    if max_rel_delta >= 0.4 || max_emotion_intensity >= 0.6 {
        Some(EscalationRequest {
            trigger: format!(
                "high_impact(rel={:.1},emo={:.1})",
                max_rel_delta, max_emotion_intensity
            ),
            beat_index,
            suggested_fidelity: if max_rel_delta >= 0.5 || max_emotion_intensity >= 0.8 {
                FidelityHint::Active
            } else {
                FidelityHint::Aware
            },
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Beat, Effect};

    fn make_beat(effects: Vec<Effect>) -> Beat {
        Beat {
            actor: "alice".to_string(),
            action: "test_action".to_string(),
            accepted: true,
            effects,
        }
    }

    #[test]
    fn no_escalation_for_mild_effects() {
        let beat = make_beat(vec![Effect::RelationshipDelta {
            axis: "trust".to_string(),
            from: "alice".to_string(),
            to: "bob".to_string(),
            delta: 0.05,
        }]);
        assert_eq!(check_escalation(&beat, 0), None);
    }

    #[test]
    fn escalation_on_betrayal_magnitude() {
        let beat = make_beat(vec![Effect::RelationshipDelta {
            axis: "trust".to_string(),
            from: "alice".to_string(),
            to: "bob".to_string(),
            delta: -0.6,
        }]);
        let result = check_escalation(&beat, 1);
        assert!(result.is_some());
        let req = result.unwrap();
        assert_eq!(req.suggested_fidelity, FidelityHint::Active);
        assert_eq!(req.beat_index, 1);
    }

    #[test]
    fn escalation_on_high_emotion() {
        let beat = make_beat(vec![Effect::EmotionalEvent {
            target: "bob".to_string(),
            emotion: "fear".to_string(),
            intensity: 0.9,
        }]);
        let result = check_escalation(&beat, 2);
        assert!(result.is_some());
        let req = result.unwrap();
        assert_eq!(req.suggested_fidelity, FidelityHint::Active);
        assert_eq!(req.beat_index, 2);
    }
}

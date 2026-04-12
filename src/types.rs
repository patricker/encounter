//! Core data types shared across the crate.

use serde::Deserialize;

/// An effect produced by a social interaction beat.
///
/// Effects are tagged with `kind` in serialized form and categorized into
/// aggregate buckets on [`EncounterResult`] by [`EncounterResult::push_beat`].
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Effect {
    /// One character conveys a belief to another.
    KnowledgeTransfer {
        /// The character transmitting the claim.
        from: String,
        /// The character receiving the claim.
        to: String,
        /// The belief being transferred.
        claim: String,
        /// Where the belief originated.
        provenance: Option<String>,
        /// Starting confidence level (0.0–1.0).
        initial_confidence: Option<f64>,
    },
    /// A shift along a relationship axis between two characters.
    RelationshipDelta {
        /// The relationship axis being modified (e.g. "trust").
        axis: String,
        /// The source character.
        from: String,
        /// The target character.
        to: String,
        /// Signed magnitude of the change.
        delta: f64,
    },
    /// An emotional response triggered in a character.
    EmotionalEvent {
        /// The character experiencing the emotion.
        target: String,
        /// The emotion label (e.g. "joy", "anger").
        emotion: String,
        /// How strongly the emotion is felt (0.0–1.0).
        intensity: f64,
    },
    /// A sustained shift in a character's mood along some axis.
    MoodShift {
        /// The character whose mood shifts.
        target: String,
        /// The mood axis being modified.
        axis: String,
        /// Signed magnitude of the shift.
        delta: f64,
    },
    /// Partial or full satisfaction of a character's need.
    NeedSatisfaction {
        /// The character whose need is satisfied.
        target: String,
        /// The need being addressed (e.g. "belonging").
        need: String,
        /// Amount of satisfaction granted.
        amount: f64,
    },
    /// A nudge to a character's value system.
    ValueShift {
        /// The character whose values shift.
        target: String,
        /// The value being modified (e.g. "honesty").
        value: String,
        /// Signed magnitude of the shift.
        delta: f64,
    },
    /// An actor leaves the current practice.
    PracticeExit {
        /// The character exiting.
        actor: String,
        /// Why they are leaving.
        reason: Option<String>,
    },
}

/// A drive that motivates a character's participation in an encounter.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct DriveAlignment {
    /// The drive category (e.g. "autonomy", "belonging").
    pub kind: String,
    /// How strongly this drive aligns with the action (0.0–1.0).
    pub strength: f64,
}

/// A single consideration in a utility-scoring curve set.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ConsiderationSpec {
    /// Unique identifier for this consideration.
    pub id: String,
    /// The curve type to apply (e.g. "linear", "logistic").
    pub curve: String,
    /// Weight applied to this consideration's score.
    pub weight: f64,
    /// Minimum score threshold; consideration is ignored below this value.
    #[serde(default)]
    pub threshold: Option<f64>,
}

/// A single resolved action exchange within an encounter.
///
/// Beats are constructed by resolution protocols rather than deserialized from
/// external data, so they do not implement `Deserialize`.
#[derive(Debug, Clone, PartialEq)]
pub struct Beat {
    /// The character performing the action.
    pub actor: String,
    /// The action identifier.
    pub action: String,
    /// Whether the action was accepted by the target.
    pub accepted: bool,
    /// Effects that fire as a result of this beat.
    pub effects: Vec<Effect>,
}

/// Aggregated output of a resolved encounter.
#[derive(Debug)]
pub struct EncounterResult {
    /// Characters who participated.
    pub participants: Vec<String>,
    /// The practice that framed this encounter, if any.
    pub practice: Option<String>,
    /// Ordered sequence of beats that occurred.
    pub beats: Vec<Beat>,
    /// All [`Effect::RelationshipDelta`] effects from all beats.
    pub relationship_deltas: Vec<Effect>,
    /// All [`Effect::KnowledgeTransfer`] effects from all beats.
    pub knowledge_transfers: Vec<Effect>,
    /// All [`Effect::EmotionalEvent`] effects from all beats.
    pub emotional_events: Vec<Effect>,
    /// All [`Effect::ValueShift`] effects from all beats.
    pub value_shifts: Vec<Effect>,
    /// Whether any participant requested escalation.
    pub escalation_requested: bool,
    /// Escalation requests emitted during the encounter.
    pub escalation_requests: Vec<crate::escalation::EscalationRequest>,
}

impl EncounterResult {
    /// Create a new, empty result for the given participants and optional practice.
    pub fn new(participants: Vec<String>, practice: Option<String>) -> Self {
        Self {
            participants,
            practice,
            beats: Vec::new(),
            relationship_deltas: Vec::new(),
            knowledge_transfers: Vec::new(),
            emotional_events: Vec::new(),
            value_shifts: Vec::new(),
            escalation_requested: false,
            escalation_requests: Vec::new(),
        }
    }

    /// Append a beat and categorize its effects into the aggregate buckets.
    ///
    /// Only `RelationshipDelta`, `KnowledgeTransfer`, `EmotionalEvent`, and
    /// `ValueShift` effects are aggregated into the top-level buckets.
    /// `MoodShift`, `NeedSatisfaction`, and `PracticeExit` effects are
    /// preserved in the beat's `effects` vec but not duplicated into
    /// separate aggregate fields.
    pub fn push_beat(&mut self, beat: Beat) {
        for effect in &beat.effects {
            match effect {
                Effect::RelationshipDelta { .. } => {
                    self.relationship_deltas.push(effect.clone());
                }
                Effect::KnowledgeTransfer { .. } => {
                    self.knowledge_transfers.push(effect.clone());
                }
                Effect::EmotionalEvent { .. } => {
                    self.emotional_events.push(effect.clone());
                }
                Effect::ValueShift { .. } => {
                    self.value_shifts.push(effect.clone());
                }
                Effect::MoodShift { .. }
                | Effect::NeedSatisfaction { .. }
                | Effect::PracticeExit { .. } => {}
            }
        }
        self.beats.push(beat);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn effect_deserializes_from_toml() {
        let s = r#"
            kind = "relationship_delta"
            axis = "trust"
            from = "alice"
            to = "bob"
            delta = 0.25
        "#;
        let effect: Effect = toml::from_str(s).expect("should deserialize");
        match effect {
            Effect::RelationshipDelta { delta, .. } => {
                assert!((delta - 0.25).abs() < f64::EPSILON);
            }
            other => panic!("unexpected variant: {other:?}"),
        }
    }

    #[test]
    fn knowledge_transfer_deserializes_with_optional_fields() {
        let s = r#"
            kind = "knowledge_transfer"
            from = "alice"
            to = "bob"
            claim = "the vault is open"
        "#;
        let effect: Effect = toml::from_str(s).expect("should deserialize");
        match effect {
            Effect::KnowledgeTransfer {
                provenance,
                initial_confidence,
                ..
            } => {
                assert!(provenance.is_none());
                assert!(initial_confidence.is_none());
            }
            other => panic!("unexpected variant: {other:?}"),
        }
    }

    #[test]
    fn drive_alignment_deserializes() {
        let s = r#"
            kind = "belonging"
            strength = 0.8
        "#;
        let da: DriveAlignment = toml::from_str(s).expect("should deserialize");
        assert_eq!(da.kind, "belonging");
        assert!((da.strength - 0.8).abs() < f64::EPSILON);
    }

    #[test]
    fn encounter_result_categorizes_effects() {
        let mut result = EncounterResult::new(
            vec!["alice".to_string(), "bob".to_string()],
            Some("negotiation".to_string()),
        );

        let beat = Beat {
            actor: "alice".to_string(),
            action: "share_secret".to_string(),
            accepted: true,
            effects: vec![
                Effect::KnowledgeTransfer {
                    from: "alice".to_string(),
                    to: "bob".to_string(),
                    claim: "the vault is open".to_string(),
                    provenance: None,
                    initial_confidence: None,
                },
                Effect::RelationshipDelta {
                    axis: "trust".to_string(),
                    from: "bob".to_string(),
                    to: "alice".to_string(),
                    delta: 0.1,
                },
            ],
        };

        result.push_beat(beat);

        assert_eq!(result.beats.len(), 1);
        assert_eq!(result.knowledge_transfers.len(), 1);
        assert_eq!(result.relationship_deltas.len(), 1);
        assert_eq!(result.emotional_events.len(), 0);
        assert_eq!(result.value_shifts.len(), 0);
    }
}

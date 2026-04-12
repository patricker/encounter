//! Practice specification types: [`PracticeSpec`], [`TurnPolicy`], and [`DurationPolicy`].

use serde::Deserialize;

/// Controls whose turn it is at each beat of a practice.
#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TurnPolicy {
    /// Participants take turns in a fixed rotation.
    #[default]
    RoundRobin,
    /// Each initiating utterance is paired with a response (adjacency pair).
    AdjacencyPair,
    /// Turn order is determined by a consumer-supplied callback.
    Custom,
}

/// Controls how long a practice runs.
#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DurationPolicy {
    /// A single initiator/responder exchange, then the practice ends.
    #[default]
    SingleExchange,
    /// Turn-based scene that continues until resolved or a beat cap is hit.
    MultiBeat {
        /// Maximum number of beats before the practice is forced to end.
        max_beats: usize,
    },
    /// The practice runs until a resolution condition is satisfied.
    UntilResolved,
}

/// Full specification for a practice, deserialized from a TOML definition file.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct PracticeSpec {
    /// Human-readable name for this practice (e.g. "negotiation").
    pub name: String,
    /// Affordance identifiers available within this practice.
    pub affordances: Vec<String>,
    /// How turn order is determined. Defaults to [`TurnPolicy::RoundRobin`].
    #[serde(default)]
    pub turn_policy: TurnPolicy,
    /// How long the practice runs. Defaults to [`DurationPolicy::SingleExchange`].
    #[serde(default)]
    pub duration_policy: DurationPolicy,
    /// Raw fabula DSL source for the practice entry condition. Empty means always enterable.
    #[serde(default)]
    pub entry_condition_source: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn practice_spec_deserializes() {
        let s = r#"
            name = "negotiation"
            affordances = ["offer", "counter_offer", "accept", "reject"]
            turn_policy = "adjacency_pair"

            [duration_policy]
            multi_beat = { max_beats = 10 }
        "#;

        let spec: PracticeSpec = toml::from_str(s).expect("should deserialize");

        assert_eq!(spec.name, "negotiation");
        assert_eq!(spec.affordances.len(), 4);
        assert_eq!(spec.turn_policy, TurnPolicy::AdjacencyPair);
        assert_eq!(
            spec.duration_policy,
            DurationPolicy::MultiBeat { max_beats: 10 }
        );
    }

    #[test]
    fn practice_defaults_to_single_exchange_round_robin() {
        let s = r#"
            name = "greeting"
            affordances = ["wave", "nod"]
        "#;

        let spec: PracticeSpec = toml::from_str(s).expect("should deserialize");

        assert_eq!(spec.name, "greeting");
        assert_eq!(spec.affordances.len(), 2);
        assert_eq!(spec.turn_policy, TurnPolicy::RoundRobin);
        assert_eq!(spec.duration_policy, DurationPolicy::SingleExchange);
        assert_eq!(spec.entry_condition_source, "");
    }
}

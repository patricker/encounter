//! MultiBeat — a turn-based scene with a custom `ActionScorer`.
//!
//! Two characters take three rounds of greetings, scoring each available
//! affordance with a fixed utility for simplicity.
//!
//! Run with: `cargo run --example multi_beat`

use encounter::affordance::{AffordanceSpec, CatalogEntry};
use encounter::practice::{DurationPolicy, PracticeSpec, TurnPolicy};
use encounter::resolution::MultiBeat;
use encounter::scoring::{ActionScorer, AlwaysAccept, ScoredAffordance};
use encounter::types::Effect;

/// A trivial scorer that hands every available affordance a fixed score.
/// In a real consumer this would consult drives, world state, an LLM, etc.
struct FixedScorer(f64);

impl ActionScorer<String> for FixedScorer {
    fn score_actions(
        &self,
        actor: &str,
        available: &[CatalogEntry<String>],
        _participants: &[String],
    ) -> Vec<ScoredAffordance<String>> {
        available
            .iter()
            .map(|entry| ScoredAffordance {
                entry: entry.clone(),
                score: self.0,
                bindings: [
                    ("self".into(), actor.into()),
                    ("target".into(), "other".into()),
                ]
                .into_iter()
                .collect(),
            })
            .collect()
    }
}

fn catalog() -> Vec<CatalogEntry<String>> {
    vec![CatalogEntry {
        spec: AffordanceSpec {
            name: "greet".into(),
            domain: "social".into(),
            bindings: vec!["self".into(), "target".into()],
            considerations: Vec::new(),
            effects_on_accept: vec![Effect::RelationshipDelta {
                axis: "friendship".into(),
                from: "self".into(),
                to: "target".into(),
                delta: 0.05,
            }],
            effects_on_reject: Vec::new(),
            drive_alignment: Vec::new(),
        },
        precondition: String::new(),
    }]
}

fn practice() -> PracticeSpec {
    PracticeSpec {
        name: "chance_meeting".into(),
        affordances: vec!["greet".into()],
        turn_policy: TurnPolicy::RoundRobin,
        duration_policy: DurationPolicy::MultiBeat { max_beats: 3 },
        entry_condition_source: String::new(),
    }
}

fn main() {
    let participants = vec!["alice".to_string(), "bob".to_string()];
    let result = MultiBeat.resolve(
        &participants,
        &practice(),
        &catalog(),
        &FixedScorer(0.8),
        &AlwaysAccept,
    );

    println!("{} beats produced:", result.beats.len());
    for (i, beat) in result.beats.iter().enumerate() {
        println!(
            "  {}: {} {} → {}",
            i,
            beat.actor,
            beat.action,
            if beat.accepted { "✓" } else { "✗" }
        );
    }
    println!(
        "relationship deltas accumulated: {}",
        result.relationship_deltas.len()
    );
}

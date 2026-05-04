//! SingleExchange — a one-shot scene with one initiator pick, one accept/reject.
//!
//! Run with: `cargo run --example single_exchange`

use encounter::affordance::{AffordanceSpec, CatalogEntry};
use encounter::resolution::SingleExchange;
use encounter::scoring::{AlwaysAccept, ScoredAffordance};
use encounter::types::Effect;

fn main() {
    let greet = CatalogEntry {
        spec: AffordanceSpec {
            name: "greet".into(),
            domain: "social".into(),
            bindings: vec!["self".into(), "target".into()],
            considerations: Vec::new(),
            effects_on_accept: vec![Effect::RelationshipDelta {
                axis: "friendship".into(),
                from: "alice".into(),
                to: "bob".into(),
                delta: 0.1,
            }],
            effects_on_reject: Vec::new(),
            drive_alignment: Vec::new(),
        },
        precondition: String::new(),
    };

    let scored = ScoredAffordance {
        entry: greet,
        score: 0.9,
        bindings: [
            ("self".into(), "alice".into()),
            ("target".into(), "bob".into()),
        ]
        .into_iter()
        .collect(),
    };

    let result = SingleExchange.resolve("alice", "bob", &[scored], &AlwaysAccept);

    println!("participants: {:?}", result.participants);
    println!("beats: {}", result.beats.len());
    for (i, beat) in result.beats.iter().enumerate() {
        println!(
            "  beat {}: {} → {} ({})",
            i,
            beat.actor,
            beat.action,
            if beat.accepted {
                "accepted"
            } else {
                "rejected"
            }
        );
    }
    println!("relationship deltas: {}", result.relationship_deltas.len());
}

//! BackgroundScheme — a long-running plot that accumulates progress over
//! several ticks, then resolves to one consequential beat that triggers
//! escalation.
//!
//! Run with: `cargo run --example background_scheme`

use encounter::resolution::BackgroundScheme;
use encounter::types::Effect;

fn main() {
    let mut scheme = BackgroundScheme::new(
        "alice".into(),
        "bob".into(),
        "spy_ring".into(),
        /* threshold */ 5.0,
    );

    // Three ticks of progress — could be game weeks, story beats, etc.
    // The drama-manager layer (consumer's responsibility) decides when to
    // call advance() and by how much.
    scheme.add_advantage("inside_man".into());
    scheme.advance(2.0);
    scheme.advance(2.0);
    let resolved = scheme.advance(1.5);
    assert!(resolved, "scheme should resolve at threshold");

    let success_effects = vec![Effect::RelationshipDelta {
        axis: "trust".into(),
        from: "alice".into(),
        to: "bob".into(),
        delta: -0.6,
    }];
    let failure_effects = vec![];

    let result = scheme.to_result(success_effects, failure_effects);

    println!("scheme: {} ({})", scheme.scheme_type, scheme.initiator);
    println!("advantages held: {:?}", scheme.advantages);
    println!("final progress: {} / {}", scheme.progress, scheme.threshold);
    println!("beats: {}", result.beats.len());
    println!(
        "  resolution: {} ({})",
        result.beats[0].action,
        if result.beats[0].accepted {
            "succeeded"
        } else {
            "failed"
        }
    );
    println!("escalation requested: {}", result.escalation_requested);
    for req in &result.escalation_requests {
        println!(
            "  → trigger={} fidelity={:?}",
            req.trigger, req.suggested_fidelity
        );
    }
}

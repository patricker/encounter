use encounter::affordance::{AffordanceSpec, CatalogEntry};
use encounter::resolution::single::SingleExchange;
use encounter::scoring::{AlwaysAccept, AlwaysReject, ScoredAffordance};
use encounter::types::Effect;

fn make_reveal_secret() -> ScoredAffordance<String> {
    ScoredAffordance {
        entry: CatalogEntry {
            spec: AffordanceSpec {
                name: "reveal_secret".into(),
                domain: "information".into(),
                bindings: vec!["self".into(), "target".into(), "subject".into()],
                considerations: Vec::new(),
                effects_on_accept: vec![
                    Effect::KnowledgeTransfer {
                        from: "self".into(),
                        to: "target".into(),
                        claim: "subject".into(),
                        provenance: Some("confided".into()),
                        initial_confidence: Some(0.9),
                    },
                    Effect::RelationshipDelta {
                        axis: "trust".into(),
                        from: "target".into(),
                        to: "self".into(),
                        delta: 0.1,
                    },
                ],
                effects_on_reject: vec![Effect::RelationshipDelta {
                    axis: "trust".into(),
                    from: "self".into(),
                    to: "target".into(),
                    delta: -0.05,
                }],
                drive_alignment: Vec::new(),
            },
            precondition: String::new(),
        },
        score: 0.8,
        bindings: [
            ("self".into(), "alice".into()),
            ("target".into(), "bob".into()),
        ]
        .into_iter()
        .collect(),
    }
}

#[test]
fn single_exchange_accepted_fires_accept_effects() {
    let protocol = SingleExchange;
    let available = vec![make_reveal_secret()];
    let result = protocol.resolve("alice", "bob", &available, &AlwaysAccept);

    assert_eq!(result.beats.len(), 1);
    let beat = &result.beats[0];
    assert!(beat.accepted);
    assert_eq!(
        result.knowledge_transfers.len(),
        1,
        "expected 1 knowledge_transfer effect"
    );
    assert_eq!(
        result.relationship_deltas.len(),
        1,
        "expected 1 relationship_delta effect"
    );
}

#[test]
fn single_exchange_rejected_fires_reject_effects() {
    let protocol = SingleExchange;
    let available = vec![make_reveal_secret()];
    let result = protocol.resolve("alice", "bob", &available, &AlwaysReject);

    assert_eq!(result.beats.len(), 1);
    let beat = &result.beats[0];
    assert!(!beat.accepted);
    assert_eq!(result.relationship_deltas.len(), 1);

    // Verify it's the negative delta from effects_on_reject
    match &result.relationship_deltas[0] {
        Effect::RelationshipDelta { delta, .. } => {
            assert!(*delta < 0.0, "reject delta should be negative");
        }
        other => panic!("unexpected effect variant: {other:?}"),
    }
}

#[test]
fn single_exchange_picks_highest_scored_action() {
    let mut low = make_reveal_secret();
    low.entry.spec.name = "small_talk".into();
    low.score = 0.3;

    let mut high = make_reveal_secret();
    high.entry.spec.name = "reveal_secret".into();
    high.score = 0.8;

    let protocol = SingleExchange;
    let available = vec![low, high];
    let result = protocol.resolve("alice", "bob", &available, &AlwaysAccept);

    assert_eq!(result.beats.len(), 1);
    assert_eq!(result.beats[0].action, "reveal_secret");
}

#[test]
fn single_exchange_with_no_actions_produces_empty_result() {
    let protocol = SingleExchange;
    let available: Vec<ScoredAffordance<String>> = vec![];
    let result = protocol.resolve("alice", "bob", &available, &AlwaysAccept);

    assert!(result.beats.is_empty());
}

#[test]
fn single_exchange_betrayal_triggers_escalation() {
    let betrayal = ScoredAffordance {
        entry: CatalogEntry {
            spec: AffordanceSpec {
                name: "betray".into(),
                domain: "personal".into(),
                bindings: vec!["self".into(), "target".into()],
                considerations: Vec::new(),
                effects_on_accept: vec![
                    Effect::RelationshipDelta {
                        axis: "trust".into(),
                        from: "target".into(),
                        to: "self".into(),
                        delta: -0.6,
                    },
                    Effect::EmotionalEvent {
                        target: "target".into(),
                        emotion: "anger".into(),
                        intensity: 0.7,
                    },
                ],
                effects_on_reject: Vec::new(),
                drive_alignment: Vec::new(),
            },
            precondition: String::new(),
        },
        score: 0.9,
        bindings: [
            ("self".into(), "alice".into()),
            ("target".into(), "bob".into()),
        ]
        .into_iter()
        .collect(),
    };
    let protocol = SingleExchange;
    let result = protocol.resolve("alice", "bob", &[betrayal], &AlwaysAccept);
    assert!(
        result.escalation_requested,
        "betrayal should trigger escalation"
    );
    assert!(!result.escalation_requests.is_empty());
}

#[test]
fn single_exchange_rejected_romance_fires_emotional_effects() {
    let romance = ScoredAffordance {
        entry: CatalogEntry {
            spec: AffordanceSpec {
                name: "romance".into(),
                domain: "personal".into(),
                bindings: vec!["self".into(), "target".into()],
                considerations: Vec::new(),
                effects_on_accept: vec![],
                effects_on_reject: vec![
                    Effect::EmotionalEvent {
                        target: "self".into(),
                        emotion: "distress".into(),
                        intensity: 0.5,
                    },
                    Effect::EmotionalEvent {
                        target: "self".into(),
                        emotion: "shame".into(),
                        intensity: 0.4,
                    },
                ],
                drive_alignment: Vec::new(),
            },
            precondition: String::new(),
        },
        score: 0.6,
        bindings: [
            ("self".into(), "alice".into()),
            ("target".into(), "bob".into()),
        ]
        .into_iter()
        .collect(),
    };
    let protocol = SingleExchange;
    let result = protocol.resolve("alice", "bob", &[romance], &AlwaysReject);
    assert!(!result.beats[0].accepted);
    assert_eq!(result.emotional_events.len(), 2);
}

use encounter::affordance::{AffordanceSpec, CatalogEntry};
use encounter::practice::{DurationPolicy, PracticeSpec, TurnPolicy};
use encounter::resolution::multi_beat::MultiBeat;
use encounter::scoring::{ActionScorer, AlwaysAccept, ScoredAffordance};
use encounter::types::Effect;

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

fn test_catalog_entries() -> Vec<CatalogEntry<String>> {
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

fn test_practice() -> PracticeSpec {
    PracticeSpec {
        name: "chance_meeting".into(),
        affordances: vec!["greet".into()],
        turn_policy: TurnPolicy::RoundRobin,
        duration_policy: DurationPolicy::MultiBeat { max_beats: 3 },
        entry_condition_source: String::new(),
    }
}

#[test]
fn multi_beat_runs_up_to_max_beats() {
    let protocol = MultiBeat;
    let participants = vec!["alice".to_string(), "bob".to_string()];
    let practice = test_practice();
    let catalog = test_catalog_entries();
    let scorer = FixedScorer(1.0);

    let result = protocol.resolve(&participants, &practice, &catalog, &scorer, &AlwaysAccept);

    assert_eq!(result.beats.len(), 3);
    assert_eq!(result.practice.as_deref(), Some("chance_meeting"));
}

#[test]
fn multi_beat_round_robin_alternates_speakers() {
    let protocol = MultiBeat;
    let participants = vec!["alice".to_string(), "bob".to_string()];
    let practice = test_practice();
    let catalog = test_catalog_entries();
    let scorer = FixedScorer(1.0);

    let result = protocol.resolve(&participants, &practice, &catalog, &scorer, &AlwaysAccept);

    assert_eq!(result.beats.len(), 3);
    assert_eq!(result.beats[0].actor, "alice");
    assert_eq!(result.beats[1].actor, "bob");
    assert_eq!(result.beats[2].actor, "alice");
}

#[test]
fn multi_beat_accumulates_effects_across_beats() {
    let protocol = MultiBeat;
    let participants = vec!["alice".to_string(), "bob".to_string()];
    let practice = test_practice();
    let catalog = test_catalog_entries();
    let scorer = FixedScorer(1.0);

    let result = protocol.resolve(&participants, &practice, &catalog, &scorer, &AlwaysAccept);

    assert_eq!(
        result.relationship_deltas.len(),
        3,
        "expected one RelationshipDelta per beat"
    );
}

#[test]
fn multi_beat_stops_early_on_no_actions() {
    let protocol = MultiBeat;
    let participants = vec!["alice".to_string(), "bob".to_string()];
    let practice = test_practice();
    let catalog: Vec<CatalogEntry<String>> = vec![]; // empty catalog
    let scorer = FixedScorer(1.0);

    let result = protocol.resolve(&participants, &practice, &catalog, &scorer, &AlwaysAccept);

    assert!(result.beats.is_empty());
}

#[test]
fn multi_beat_flags_escalation_on_high_impact_beat() {
    // Create a catalog entry with high-impact effects (e.g., betray with delta -0.6)
    let heavy_entry = CatalogEntry {
        spec: AffordanceSpec {
            name: "betray".into(),
            domain: "personal".into(),
            bindings: vec!["self".into(), "target".into()],
            considerations: Vec::new(),
            effects_on_accept: vec![Effect::RelationshipDelta {
                axis: "trust".into(),
                from: "target".into(),
                to: "self".into(),
                delta: -0.6,
            }],
            effects_on_reject: Vec::new(),
            drive_alignment: Vec::new(),
        },
        precondition: String::new(),
    };
    let practice = PracticeSpec {
        name: "confrontation".into(),
        affordances: vec!["betray".into()],
        turn_policy: TurnPolicy::RoundRobin,
        duration_policy: DurationPolicy::MultiBeat { max_beats: 2 },
        entry_condition_source: String::new(),
    };
    let protocol = MultiBeat;
    let result = protocol.resolve(
        &["alice".into(), "bob".into()],
        &practice,
        &[heavy_entry],
        &FixedScorer(0.9),
        &AlwaysAccept,
    );
    assert!(result.escalation_requested);
    assert!(!result.escalation_requests.is_empty());
}

#[test]
fn multi_beat_handles_empty_participants() {
    let protocol = MultiBeat;
    let result = protocol.resolve(
        &[],
        &test_practice(),
        &test_catalog_entries(),
        &FixedScorer(0.7),
        &AlwaysAccept,
    );
    assert!(result.beats.is_empty());
}

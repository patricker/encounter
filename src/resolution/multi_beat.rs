//! Versu-style turn-based encounter resolution.

use crate::affordance::CatalogEntry;
use crate::practice::{DurationPolicy, PracticeSpec, TurnPolicy};
use crate::scoring::{AcceptanceEval, ActionScorer};
use crate::types::{Beat, Effect, EncounterResult};

/// Multi-beat resolution: participants take turns across multiple beats,
/// re-scoring actions each beat. Supports RoundRobin and AdjacencyPair
/// turn policies, and respects PracticeExit effects.
pub struct MultiBeat;

impl MultiBeat {
    /// Generic over P (precondition type). Scores are recomputed each beat
    /// by the provided `scorer`, allowing world-state changes to influence
    /// later action selection.
    pub fn resolve<P: Clone>(
        &self,
        participants: &[String],
        practice: &PracticeSpec,
        catalog: &[CatalogEntry<P>],
        scorer: &dyn ActionScorer<P>,
        acceptance: &dyn AcceptanceEval<P>,
    ) -> EncounterResult {
        let max_beats = match practice.duration_policy {
            DurationPolicy::MultiBeat { max_beats } => max_beats,
            DurationPolicy::SingleExchange => 1,
            DurationPolicy::UntilResolved => usize::MAX,
        };

        let mut result = EncounterResult::new(
            participants.to_vec(),
            Some(practice.name.clone()),
        );

        // Filter catalog to affordances allowed by the practice.
        let allowed: Vec<CatalogEntry<P>> = catalog
            .iter()
            .filter(|e| practice.affordances.contains(&e.spec.name))
            .cloned()
            .collect();

        let mut speaker_idx = 0usize;

        for _beat_num in 0..max_beats {
            let speaker = &participants[speaker_idx % participants.len()];
            let responder_idx = (speaker_idx + 1) % participants.len();
            let responder = &participants[responder_idx];

            let scored = scorer.score_actions(speaker, &allowed, participants);
            let Some(best) = scored
                .iter()
                .max_by(|a, b| a.score.partial_cmp(&b.score).unwrap_or(std::cmp::Ordering::Equal))
            else {
                break;
            };

            let accepted = acceptance.evaluate(responder, best);
            let effects = if accepted {
                best.entry.spec.effects_on_accept.clone()
            } else {
                best.entry.spec.effects_on_reject.clone()
            };

            let exit_requested = effects.iter().any(|e| matches!(e, Effect::PracticeExit { .. }));

            let beat = Beat {
                actor: speaker.clone(),
                action: best.entry.spec.name.clone(),
                accepted,
                effects,
            };
            result.push_beat(beat);

            if exit_requested {
                break;
            }

            speaker_idx = match practice.turn_policy {
                TurnPolicy::RoundRobin => speaker_idx + 1,
                TurnPolicy::AdjacencyPair => responder_idx,
                TurnPolicy::Custom => speaker_idx + 1,
            };
        }
        result
    }
}

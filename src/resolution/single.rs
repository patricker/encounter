//! CiF-style one-shot resolution protocol.

use crate::scoring::{AcceptanceEval, ScoredAffordance};
use crate::types::{Beat, EncounterResult};

/// Single-exchange resolution: initiator picks highest-scored action,
/// responder accepts or rejects, effects fire, one beat is recorded.
pub struct SingleExchange;

impl SingleExchange {
    /// Generic over P (precondition type). Protocol never inspects P.
    pub fn resolve<P>(
        &self,
        initiator: &str,
        responder: &str,
        available: &[ScoredAffordance<P>],
        acceptance: &dyn AcceptanceEval<P>,
    ) -> EncounterResult {
        let mut result = EncounterResult::new(vec![initiator.into(), responder.into()], None);

        let Some(best) = available.iter().max_by(|a, b| {
            a.score
                .partial_cmp(&b.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        }) else {
            return result;
        };

        let accepted = acceptance.evaluate(responder, best);
        let effects = if accepted {
            best.entry.spec.effects_on_accept.clone()
        } else {
            best.entry.spec.effects_on_reject.clone()
        };

        let beat = Beat {
            actor: initiator.into(),
            action: best.entry.spec.name.clone(),
            accepted,
            effects,
        };
        result.push_beat(beat);
        if let Some(esc) = crate::escalation::check_escalation(
            result.beats.last().unwrap(),
            result.beats.len() - 1,
        ) {
            result.escalation_requested = true;
            result.escalation_requests.push(esc);
        }
        result
    }
}

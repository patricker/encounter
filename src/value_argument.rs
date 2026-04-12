//! DF-style value argument resolution.
//!
//! Characters argue about Schwartz values; the winner shifts the loser's value
//! based on the conviction gap and the defender's openness to persuasion.

/// Input to a value argument resolution.
#[derive(Debug, Clone)]
pub struct ValueArgumentInput {
    /// Name of the character initiating the argument.
    pub attacker: String,
    /// Name of the character defending their position.
    pub defender: String,
    /// The Schwartz value being contested.
    pub value_at_stake: String,
    /// How strongly the attacker holds their position. Range: `[0.0, 1.0]`.
    pub attacker_conviction: f64,
    /// How strongly the defender holds their position. Range: `[0.0, 1.0]`.
    pub defender_conviction: f64,
    /// How open the defender is to changing their position. Range: `[0.0, 1.0]`.
    pub defender_openness: f64,
}

/// Output of a value argument resolution.
#[derive(Debug, Clone)]
pub struct ValueArgumentResult {
    /// Name of the character who won the argument.
    pub winner: String,
    /// Name of the character who lost the argument.
    pub loser: String,
    /// The Schwartz value that was contested.
    pub value_at_stake: String,
    /// Magnitude of the shift applied to the loser's value. Range: `[0.0, 1.0]`.
    pub loser_value_shift: f64,
    /// Small self-reinforcement applied to the winner's value. Range: `[0.0, 0.1]`.
    pub winner_value_shift: f64,
}

/// Resolve a value argument.
///
/// The character with higher conviction wins. On a tie, the attacker wins.
///
/// - Loser shift = `conviction_gap × defender_openness`, clamped to `[0.0, 1.0]`.
/// - Winner self-reinforcement = `conviction_gap × 0.1`, clamped to `[0.0, 0.1]`.
pub fn resolve_value_argument(input: &ValueArgumentInput) -> ValueArgumentResult {
    let attacker_wins = input.attacker_conviction >= input.defender_conviction;

    let (winner, loser) = if attacker_wins {
        (input.attacker.clone(), input.defender.clone())
    } else {
        (input.defender.clone(), input.attacker.clone())
    };

    let conviction_gap = (input.attacker_conviction - input.defender_conviction).abs();

    let loser_value_shift = (conviction_gap * input.defender_openness).clamp(0.0, 1.0);

    let winner_value_shift = (conviction_gap * 0.1).clamp(0.0, 0.1);

    ValueArgumentResult {
        winner,
        loser,
        value_at_stake: input.value_at_stake.clone(),
        loser_value_shift,
        winner_value_shift,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn winner_shifts_loser_value() {
        let input = ValueArgumentInput {
            attacker: "Alice".to_string(),
            defender: "Bob".to_string(),
            value_at_stake: "Benevolence".to_string(),
            attacker_conviction: 0.8,
            defender_conviction: 0.4,
            defender_openness: 0.6,
        };
        let result = resolve_value_argument(&input);
        assert_eq!(result.winner, "Alice");
        assert!(result.loser_value_shift > 0.0);
        assert!(result.loser_value_shift <= 1.0);
    }

    #[test]
    fn defender_wins_when_more_convinced() {
        let input = ValueArgumentInput {
            attacker: "Alice".to_string(),
            defender: "Bob".to_string(),
            value_at_stake: "Power".to_string(),
            attacker_conviction: 0.3,
            defender_conviction: 0.9,
            defender_openness: 0.5,
        };
        let result = resolve_value_argument(&input);
        assert_eq!(result.winner, "Bob");
    }

    #[test]
    fn winner_gets_small_self_reinforcement() {
        let input = ValueArgumentInput {
            attacker: "Alice".to_string(),
            defender: "Bob".to_string(),
            value_at_stake: "Security".to_string(),
            attacker_conviction: 0.8,
            defender_conviction: 0.4,
            defender_openness: 0.6,
        };
        let result = resolve_value_argument(&input);
        assert!(result.winner_value_shift > 0.0);
        assert!(result.winner_value_shift < result.loser_value_shift);
    }

    #[test]
    fn equal_conviction_favors_attacker() {
        let input = ValueArgumentInput {
            attacker: "Alice".to_string(),
            defender: "Bob".to_string(),
            value_at_stake: "Tradition".to_string(),
            attacker_conviction: 0.5,
            defender_conviction: 0.5,
            defender_openness: 0.8,
        };
        let result = resolve_value_argument(&input);
        assert_eq!(result.winner, "Alice");
        assert_eq!(result.loser_value_shift, 0.0);
    }
}

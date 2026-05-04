//! Affordance catalog types: [`AffordanceSpec`] and [`CatalogEntry`].

use crate::types::{ConsiderationSpec, DriveAlignment, Effect};
use serde::{Deserialize, Serialize};

/// Metadata for a single action, deserialized from a catalog TOML file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AffordanceSpec {
    /// Human-readable name for this action.
    pub name: String,
    /// The practice domain this action belongs to (e.g. "negotiation").
    pub domain: String,
    /// Slot bindings required to instantiate this action (e.g. ["initiator", "target"]).
    pub bindings: Vec<String>,
    /// Utility-scoring considerations evaluated before scoring this action.
    #[serde(default)]
    pub considerations: Vec<ConsiderationSpec>,
    /// Effects that fire when the target accepts this action.
    #[serde(default)]
    pub effects_on_accept: Vec<Effect>,
    /// Effects that fire when the target rejects this action.
    #[serde(default)]
    pub effects_on_reject: Vec<Effect>,
    /// Drive alignments that motivate choosing this action.
    #[serde(default)]
    pub drive_alignment: Vec<DriveAlignment>,
}

/// An affordance spec paired with a compiled or raw precondition.
///
/// The default type parameter `P = String` holds raw fabula DSL text.
/// Bridge crates may substitute a typed pattern by specializing `P`.
#[derive(Debug, Clone)]
pub struct CatalogEntry<P = String> {
    /// The affordance metadata.
    pub spec: AffordanceSpec,
    /// The precondition that must hold for this action to be available.
    pub precondition: P,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn affordance_spec_deserializes_from_toml() {
        let s = r#"
            name = "share_secret"
            domain = "negotiation"
            bindings = ["initiator", "target"]

            [[considerations]]
            id = "trust_level"
            curve = "linear"
            weight = 0.7

            [[considerations]]
            id = "mood_openness"
            curve = "logistic"
            weight = 0.3

            [[effects_on_accept]]
            kind = "knowledge_transfer"
            from = "initiator"
            to = "target"
            claim = "the vault is open"

            [[effects_on_reject]]
            kind = "relationship_delta"
            axis = "trust"
            from = "target"
            to = "initiator"
            delta = -0.1

            [[drive_alignment]]
            kind = "belonging"
            strength = 0.8

            [[drive_alignment]]
            kind = "autonomy"
            strength = 0.4
        "#;

        let spec: AffordanceSpec = toml::from_str(s).expect("should deserialize");

        assert_eq!(spec.name, "share_secret");
        assert_eq!(spec.domain, "negotiation");
        assert_eq!(spec.bindings.len(), 2);
        assert_eq!(spec.bindings[0], "initiator");
        assert_eq!(spec.bindings[1], "target");
        assert_eq!(spec.considerations.len(), 2);
        assert_eq!(spec.effects_on_accept.len(), 1);
        assert_eq!(spec.effects_on_reject.len(), 1);
        assert_eq!(spec.drive_alignment.len(), 2);
    }

    #[test]
    fn affordance_spec_handles_missing_optional_sections() {
        let s = r#"
            name = "greet"
            domain = "social"
            bindings = ["initiator"]
        "#;

        let spec: AffordanceSpec = toml::from_str(s).expect("should deserialize");

        assert_eq!(spec.name, "greet");
        assert_eq!(spec.domain, "social");
        assert_eq!(spec.bindings.len(), 1);
        assert!(spec.considerations.is_empty());
        assert!(spec.effects_on_accept.is_empty());
        assert!(spec.effects_on_reject.is_empty());
        assert!(spec.drive_alignment.is_empty());
    }

    #[test]
    fn catalog_entry_holds_spec_and_string_precondition() {
        let spec = AffordanceSpec {
            name: "threaten".to_string(),
            domain: "conflict".to_string(),
            bindings: vec!["aggressor".to_string(), "target".to_string()],
            considerations: Vec::new(),
            effects_on_accept: Vec::new(),
            effects_on_reject: Vec::new(),
            drive_alignment: Vec::new(),
        };
        let precondition = "aggressor.power > target.power".to_string();

        let entry = CatalogEntry {
            spec,
            precondition: precondition.clone(),
        };

        assert_eq!(entry.spec.name, "threaten");
        assert!(!entry.precondition.is_empty());
        assert_eq!(entry.precondition, precondition);
    }

    #[test]
    fn catalog_entry_works_with_unit_precondition() {
        let spec = AffordanceSpec {
            name: "wave".to_string(),
            domain: "social".to_string(),
            bindings: vec!["actor".to_string()],
            considerations: Vec::new(),
            effects_on_accept: Vec::new(),
            effects_on_reject: Vec::new(),
            drive_alignment: Vec::new(),
        };

        let entry: CatalogEntry<()> = CatalogEntry {
            spec,
            precondition: (),
        };

        assert_eq!(entry.spec.name, "wave");
        let () = entry.precondition;
    }
}

# Changelog

## [0.1.0] - 2026-05-03

Initial release.

### Added
- Core types: `Effect` (7 variants), `AffordanceSpec`, `CatalogEntry<P>`,
  `PracticeSpec`, `Beat`, `EncounterResult`, `DriveAlignment`,
  `ConsiderationSpec`. All `serde::Serialize` (and `Deserialize` where they
  are loaded from catalog data).
- Catalog loader: `load_catalog_dir` reads paired `.toml` + `.fabula` files.
- Three resolution protocols:
  - `SingleExchange` — one initiator pick, one accept/reject.
  - `MultiBeat` — turn-based scene, scoring recomputed per beat.
  - `BackgroundScheme` — long-running plot that accumulates progress and
    resolves to one consequential beat. Runs `check_escalation` on the
    resolution beat so high-stakes outcomes surface in
    `EncounterResult.escalation_requests`.
- `EscalationRequest` and `FidelityHint` for scene-boundary signaling to a
  drama manager.
- Scoring traits `AcceptanceEval<P>` and `ActionScorer<P>`
  (consumer-implemented). No `Send + Sync` supertrait — consumers add the
  bound at the call site if they need cross-thread sharing.
- DF-style value-argument resolution via `value_argument` module.
- Test helpers `AlwaysAccept` and `AlwaysReject`.
- Runnable examples for each protocol under `examples/`.
- Integration test against the `societas/catalog/` directory (>= 30
  affordances). The test self-skips when the sibling repo is absent.

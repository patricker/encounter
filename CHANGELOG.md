# Changelog

## [0.1.0] - 2026-04-12

### Added
- Core types: `Effect` (7 variants), `AffordanceSpec`, `CatalogEntry<P>`,
  `PracticeSpec`, `Beat`, `EncounterResult`, `DriveAlignment`, `ConsiderationSpec`.
- Catalog loader: `load_catalog_dir` reads `.toml` + `.fabula` pairs.
- Three resolution protocols: `SingleExchange`, `MultiBeat`, `BackgroundScheme`.
- DF-style value argument resolution.
- `EscalationRequest` for scene-boundary signaling to the drama manager.
- Scoring traits: `AcceptanceEval<P>`, `ActionScorer<P>` (consumer-implemented).
- Test helpers: `AlwaysAccept`, `AlwaysReject`.
- Integration test against the `societas/catalog/` directory (31 actions).

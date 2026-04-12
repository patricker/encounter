# encounter

Social interaction resolution engine. Resolves multi-character encounters via practice-scoped action catalogs and three resolution protocols.

## Protocols

- **SingleExchange** — CiF-style one-shot: initiator picks, responder accepts or rejects.
- **MultiBeat** — Versu-style turn-based scene with configurable turn policy.
- **BackgroundScheme** — CK3-style long-duration plot with progress accumulation.

## Standalone

encounter depends on `serde`, `toml`, and `thiserror`. Nothing else. Pattern evaluation (fabula) and utility scoring (salience) are delegated to consumers via the `AcceptanceEval` and `ActionScorer` traits.

## Generic over precondition type

`CatalogEntry<P>` is generic over its precondition. The catalog loader produces `CatalogEntry<String>` (raw fabula DSL text). Bridge crates can compile to `CatalogEntry<Pattern<L, V>>` for typed pattern matching. Consumers who don't use fabula can use `CatalogEntry<()>`.

## References

- Evans & Short, *Versu* (IEEE TCIAIG 2014)
- McCoy et al., *Comme il Faut* (Game AI Pro 3 Ch. 43)
- CK3 interactions (https://ck3.paradoxwikis.com/Interactions)
- Sacks, *Lectures on Conversation* (1992)

## License

Dual-licensed under MIT or Apache-2.0.

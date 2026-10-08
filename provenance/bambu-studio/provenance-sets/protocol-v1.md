# Bambu Protocol Provenance Set v1

- Record ID: `BBL-PROTOCOL-PROVENANCE-SET-v1`
- Scope: protocol-v1/raw-core-3MF/settings-v2 runtime described by the
  [capability baseline](../capabilities/protocol-v1.md)
- Target: Bambu Studio 2.7.1.62 at
  `42d319c6692fa8e64790fddf0cdaafd2a4254bcc`
- Neutral input: Onshape Export
  `793ac0d5ab8e1db1ff6183fc651a568e5bacfe34`
- Existing immutable capability sets:
  [named objects v1](named-objects-v1.md) and
  [support-blocking volumes v1](support-blocking-volumes-v1.md)
- New relationship evidence: [source influence](../source-influence/protocol-v1.md)
- Applicable terms, dependency resolution and notices:
  [protocol review](../reviews/protocol-v1.md), root `NOTICE`, and neutral MIT notice
- Implementation: `src/generator_protocol.rs`, `cache_key.rs`, `settings.rs`,
  `geometry.rs`, `runtime.rs`, `main.rs` in `crates/bambu-studio`, with inherited
  `lib.rs` capabilities
- Regression/fixture recipes: `tests/common/mod.rs`, `tests/protocol_runtime.rs`,
  `examples/protocol_validation_fixture.rs`, and inherited capability tests
- Authors/review: Codex operating for Kyle Altendorf, with separate protocol and
  geometry review agents, 2026-10-08

Public source incorporation is supported by exact upstream identities, consulted
references/content hashes, classification/evidence pairs, license selections and
retained attribution. Immutable local implementation commits and exact-target
results are recorded by the subsequent verification supplement before PR delivery.
The earlier capability records are preserved unchanged.

This set is for development/public incorporation. It is not the aggregate
`BBL-MVP-PROVENANCE-SET-v1`, does not identify distributable generator bytes, and
does not authorize package release or service integration. Issue #9 owns that
release-only evidence and approval.

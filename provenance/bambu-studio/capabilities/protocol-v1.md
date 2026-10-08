# Bambu Protocol v1 Capability Baseline

- Record ID: `BBL-PROTOCOL-BASELINE-v1`
- Destination: `crates/bambu-studio`, AGPL-3.0-only
- Scope: neutral protocol v1, settings v2, bounded raw core Geometry 3MF
  realization, absolute placement once, named-object and support-blocker dispatch,
  final target-aware semantic validation, atomic candidate/result commit
- Upstream target: `bambulab/BambuStudio`, tag `v02.07.01.62`, revision
  `42d319c6692fa8e64790fddf0cdaafd2a4254bcc`
- Canonical URL: <https://github.com/bambulab/BambuStudio>
- Neutral contract: `altendky/onshape-export`, revision
  `793ac0d5ab8e1db1ff6183fc651a568e5bacfe34`
- Author: Codex operating for Kyle Altendorf <sda@fstab.net>, 2026-10-08
- Review: separate Codex `protocol_research` and `geometry_research` agents,
  2026-10-08; findings corrected and regression-tested

The [runtime profile](../../../docs/src/project/bambu-protocol-v1.md) defines
exact identities, supported geometry, explicit unsupported cases, bounds, roles,
normalization, and validation. The unchanged service schemas are retained in
`crates/bambu-studio/neutral/`; source selectors and transform derivation are
outside the generator. No service contract is extended.

This baseline incorporates the completed named-object and support-blocking
capabilities at revision 1. Their existing records are retained unchanged. See
[source influence](../source-influence/protocol-v1.md),
[terms/dependency review](../reviews/protocol-v1.md), and
[protocol provenance set](../provenance-sets/protocol-v1.md).

This is a public-source incorporation baseline, not package release approval.
The aggregate MVP release and exact package bytes remain issue #9.

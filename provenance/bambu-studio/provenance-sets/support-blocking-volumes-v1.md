# Bambu Support-Blocking Volumes Provenance Set v1

## Identity

- Record ID: `BBL-SUPPORT-BLOCKING-VOLUMES-PROVENANCE-SET-v1`
- Capability ID and revision: `bambu-studio.support-blocking-volumes`, `1`
- Destination: `crates/bambu-studio`
- Exact validated target: Bambu Studio `2.7.1.62`
- Upstream revision:
  `42d319c6692fa8e64790fddf0cdaafd2a4254bcc`
- Implementation commits:
  `50825bfc5628cc588a4ae2803b8e9477d7548b77`,
  `de0952d7ffc0476185dc9c0cf9ef2e136591361e`, and
  `2113110348d6f45f495b52481e8a5d0b9139dd7d`
- Evidence commit:
  `add6b12a7ac45a6790df8a219c77d51a9d31ef7b`

## Covered Records

This set resolves the capability revision to these version-controlled records:

- Capability baseline: [`BBL-SUPPORT-BLOCKING-VOLUMES-BASELINE-v1`](../capabilities/support-blocking-volumes-v1.md)
- Source-neutral requirements: [`BBL-MVP-INPUT-v1`](../requirements/mvp-input-v1.md)
- Baseline source and AppImage terms: [`BBL-UPSTREAM-TERMS-v1`](../reviews/bambu-studio-v02.07.01.62.md)
- Capability terms supplement: [`BBL-SUPPORT-BLOCKING-VOLUMES-TERMS-v1`](../reviews/support-blocking-volumes-v1.md)
- Dependencies: [`BBL-SUPPORT-BLOCKING-VOLUMES-DEPENDENCIES-v1`](../reviews/support-blocking-volumes-rust-dependencies-v1.md)
- Source influence: [`BBL-SUPPORT-BLOCKING-VOLUMES-SOURCE-INFLUENCE-v1`](../source-influence/support-blocking-volumes-v1.md)
- Implementation: [`BBL-SUPPORT-BLOCKING-VOLUMES-IMPLEMENTATION-v1`](../implementation/support-blocking-volumes-v1.md)
- Verification: [`BBL-SUPPORT-BLOCKING-VOLUMES-VERIFICATION-v1`](../verification/support-blocking-volumes-v1.md)
- Notices: [`BBL-SUPPORT-BLOCKING-VOLUMES-NOTICES-v1`](../notices/support-blocking-volumes-v1.md)
- Reproducible environment: [support-blocking target validation](../validation/support-blocking-volumes-v1/README.md)

The implementation commits contain the role-aware input and output API,
structured validation and atomic failures, deterministic minimum 3MF writer,
source-authored fixture, integration tests, and exact duplicate-identity and
validation-branch coverage. The evidence commit contains relationship
classifications, exact upstream hashes and permalinks, terms and attribution,
dependency and notice review, exact-target observations and hashes,
reproduction environment, and repository status documentation.

## Compatibility Result

The source-authored input SHA-256 is
`6910a5bcc6e1301637829079d03807edef5e6a7239b6e67a8388564ae97f2780`.
The official target AppImage SHA-256 is
`fa98b608532dfbbbb2b0931483aac41e57fb19c175a2cc7bd7d528d5e0fbb287`.
The target-saved project SHA-256 is
`f9ed8a21ea4129b3ef9a9a0d7d21924661d3e98d8ebaf48d0a1da4cdcd2cee14`.
The pinned target loaded both models and all three requested blockers, retained
each blocker as `support_blocker` under its intended model through save, and
reloaded its native archive through importer completion.

## Gates

This set completes public-incorporation evidence for support-blocking-volumes
revision 1. It is not a generator release review. Distribution remains blocked
pending an actual package candidate, package and build identity and hashes,
complete aggregate MVP evidence, distribution review, and named release
approval. It does not approve the protocol runner, service integration,
sandbox, cache, deployment, generated-artifact publication, or production use.

# Bambu Named Objects Provenance Set v1

## Identity

- Record ID: `BBL-NAMED-OBJECTS-PROVENANCE-SET-v1`
- Capability ID and revision: `bambu-studio.named-objects`, `1`
- Destination: `crates/bambu-studio`
- Exact validated target: Bambu Studio `2.7.1.62`
- Upstream revision:
  `42d319c6692fa8e64790fddf0cdaafd2a4254bcc`
- Implementation commit:
  `383e480673d73096942cd11d04ebe597f467a05b`
- Evidence commit:
  `1d58da1e7cf423105f6987b1cf024618c2989e11`

## Covered Records

This set resolves the capability revision to these version-controlled records:

- Capability baseline: [`BBL-NAMED-OBJECTS-BASELINE-v1`](../capabilities/named-objects-v1.md)
- Source-neutral requirements: [`BBL-MVP-INPUT-v1`](../requirements/mvp-input-v1.md)
- Source and AppImage terms: [`BBL-UPSTREAM-TERMS-v1`](../reviews/bambu-studio-v02.07.01.62.md)
- Dependencies: [`BBL-NAMED-OBJECTS-DEPENDENCIES-v1`](../reviews/rust-dependencies-v1.md)
- Source influence: [`BBL-NAMED-OBJECTS-SOURCE-INFLUENCE-v1`](../source-influence/named-objects-v1.md)
- Implementation: [`BBL-NAMED-OBJECTS-IMPLEMENTATION-v1`](../implementation/named-objects-v1.md)
- Verification: [`BBL-NAMED-OBJECTS-VERIFICATION-v1`](../verification/named-objects-v1.md)
- Notices: [`BBL-NAMED-OBJECTS-NOTICES-v1`](../notices/named-objects-v1.md)
- Reproducible environment: [named-objects target validation](../validation/README.md)

The implementation commit contains the library, structured validation errors,
deterministic 3MF writer, source-authored fixtures, integration tests, locked
dependencies, license policy, and scaffold changes. The evidence commit contains
the relationship classifications, exact upstream hashes and permalinks,
dependency and AppImage reviews, notices, exact-target observations and hashes,
reproduction environment, and repository status documentation.

## Compatibility Result

The source-authored input SHA-256 is
`77dad0c1544adcd9c441e658bc8489c915b6a463fb3792a88766e9a517c53f3c`.
The official target AppImage SHA-256 is
`fa98b608532dfbbbb2b0931483aac41e57fb19c175a2cc7bd7d528d5e0fbb287`.
The target-saved project SHA-256 is
`e79f1ba7dabd60d909910b2fc098e2e80bc979d865bb11183ffb180bb33b7ef9`.
The pinned target loaded all four generated objects, saved the project with all
names retained, and reloaded its native archive through importer completion.

## Gates

This set completes public-incorporation evidence for named-objects revision 1.
It is not a generator release review. Distribution remains blocked pending an
actual package candidate, package/build identity and hashes, complete MVP
evidence, distribution review, and named release approval. It does not approve
the protocol runner, service integration, sandbox, cache, deployment,
generated-artifact publication, or production use.

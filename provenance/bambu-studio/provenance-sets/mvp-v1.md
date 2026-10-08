# Bambu MVP Provenance Set v1

- Record ID: `BBL-MVP-PROVENANCE-SET-v1`
- Author: Codex operating for Kyle Altendorf; date: 2026-10-08
- Destination: `crates/bambu-studio`, AGPL-3.0-only
- Implementation snapshot: `54a0fa1635f75039d6b858a95bbe803bb61103cf`
- Package: Bambu Studio generator 0.1.0, `x86_64-unknown-linux-gnu`

This aggregate resolves `bambu-studio.named-objects@1` and
`bambu-studio.support-blocking-volumes@1` to the exact released implementation,
neutral runtime and release evidence below. It supplements the earlier
development/public-incorporation sets without rewriting them or treating their
earlier release gates as approvals.

## Immutable Relationships

The implementation snapshot contains the preserved
[named-object set](named-objects-v1.md),
[support-blocking set](support-blocking-volumes-v1.md), and
[protocol set](protocol-v1.md), including their relationship-level source
influence, implementation, terms, notices, requirements, fixtures, authorship,
reviews and verification records. Their exact paths and SHA-256 values are
bound by the release [source inventory](../releases/v0.1.0/source-files.json).
The full snapshot and unchanged vendored dependencies accompany the release.

Bambu source remains `bambulab/BambuStudio`, tag `v02.07.01.62`, commit
`42d319c6692fa8e64790fddf0cdaafd2a4254bcc`. Its consulted references, official
permalinks, SHA-256 content hashes, contributors and modifications are recorded
by the three source-influence records. Neutral reuse remains
`altendky/onshape-export` commit `793ac0d5ab8e1db1ff6183fc651a568e5bacfe34`,
with the retained MIT grant. No new upstream algorithms or constants are
introduced by this release-only work.

Every previously recorded `direct_source_reuse`, `adapted_algorithm` and
`adapted_constant` relationship remains `source_informed`; the target
`schema_fact` relationships remain `source_informed_schema_fact`.
Inherited target observations retain their recorded relationship classifications,
matching evidence kinds and explicit prior source use, including the blocker's
`schema_fact` / `source_informed_schema_fact` verification relationship.
This aggregate does not reclassify them. No clean-room claim is made.

The new release-script relationships are explicit:

| Local relationship | Classification | Evidence kind and basis |
| --- | --- | --- |
| `releases/v0.1.0/build.py` descriptor identity and `validate.py` descriptor/invocation/manifest/input-set identities | adapted_algorithm | source_informed: the MIT neutral identity algorithms in `crates/bambu-studio/src/cache_key.rs` lines 10–35 and `generator_protocol.rs` lines 402–439, 490–531, 742–758 at the implementation snapshot; inherited protocol source-influence references and MIT notice apply |
| `prepare.py` source/dependency/notice collection and file inventories; `build.py` deterministic archive construction | independently_derived_behavior | independently_derived_behavior: repository-authored assembly recipes, no relevant implementation-source reuse; two clean source/package experiments and byte comparisons in release verification |
| `validate.py` positive-case and failure checks, synthetic unsupported-materials mutation | independently_derived_behavior | independently_derived_behavior: original experiment recipe using the existing source-authored fixture inputs, with explicit neutral identity algorithm adaptation in the preceding row |

All local script and consulted implementation-file hashes are fixed by the
source inventory; their authorship is Codex operating for Kyle Altendorf,
2026-10-08. Applicable neutral terms/notices, separate agent review and
reproducible tests are in the distribution and named release reviews.

The protocol implementation commits are
`17c1b8b2e4efe772249ccfac3cf6257e1b98285a`,
`136decb3fb099e84b1da34b650e1a8729498f03f`, and
`e632030efc4fde9e77a3665279f25bc3014ac52e`; prior capability implementation
commits remain recorded in their preserved sets. Authors are Codex operating
for Kyle Altendorf; the separate named protocol and geometry agents reviewed
the implementation and the final release as recorded in the release review.

## Exact Contract And Artifacts

Protocol v1 accepts `raw-geometry-3mf-v1` under
`raw-geometry-3mf-core-profile-v1`, generator-settings v2, ordered rawGeometry
and auxiliaryGeometry roles, the two capability revisions above, dialect
`bambu-studio-2.7.1.62-3mf-v1`, normalization
`bambu-3mf-realize-meters-place-once-v1`, and validation
`bambu-3mf-semantic-readback-v1`. Unit/internal-transform realization precedes
the absolute neutral placement exactly once; target output is millimeters.

The unchanged protocol schema SHA-256 is
`604676b76189003392332a94dd97a7dd7f0ef992321022639f18fff8c7ea403a`.
The settings schema byte SHA-256 is
`f7ef791c8f0666a599157a34e3f4984f80026a3d7a121ebc04644281652ae06c`;
its protocol JCS identity is
`adfbdd411a8562cd84918ca9facf8c91f9cfeebed1a5cc606f46b2289920e465`.
Per-case settings and invocation identities appear in the
[CLI results](../releases/v0.1.0/cli-results.json).

The exact executable, descriptor, build and package identities are in
[identities](../releases/v0.1.0/identities.json), distinct from archive hashes.
The [artifact record](../releases/v0.1.0/artifacts.json) binds package and
corresponding-source bytes. Complete dependency resolution and checksums,
toolchain/system inputs, source hashes/modes, package file hashes, reproducible
experiments, representative failures, and exact-target compatibility are in
the [release verification](../releases/v0.1.0/verification.md).

Applicable distribution obligations and retained notice locations are in the
[distribution review](../releases/v0.1.0/distribution.md). The actual candidate
requires the [named release approval](../releases/v0.1.0/review.md).
The release tag `bambu-studio-v0.1.0` fixes these records and identities;
archive digests independently bind the exact bytes. The full implementation
snapshot already fixes the inherited provenance records without a tag cycle.

Compatibility is established only for Bambu Studio 2.7.1.62 and the enumerated
synthetic configured/rigid-assembly cases. The bounded core input profile,
unsupported features and resource limits remain those in
`docs/src/project/bambu-protocol-v1.md` at the implementation snapshot. This
generator release supplies no service manifest, deployment, cache, artifact
publication or production approval.

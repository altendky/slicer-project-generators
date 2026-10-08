# Bambu MVP Release Review v1

- Record ID: `BBL-MVP-RELEASE-REVIEW-v1`
- Decision date: 2026-10-08
- Outcome: **APPROVED for generator release distribution**
- Implementation: `54a0fa1635f75039d6b858a95bbe803bb61103cf`
- Aggregate: [`BBL-MVP-PROVENANCE-SET-v1`](../../provenance-sets/mvp-v1.md)
- Release tag: `bambu-studio-v0.1.0`

## Exact Approved Candidate

| Artifact | Bytes | SHA-256 |
| --- | ---: | --- |
| Linux x86_64 package | 1,354,728 | a9c859962d23de9812a28871fa6a80334b7543ab695b52db73ca2d91387c3d9c |
| Complete corresponding source | 8,076,180 | bcc3c15220f739b9df9c6b5a6b2422c05ac4c126a37e2e8c5a8640b3184535ae |
| Executable inside package | — | 734aa2340c954b7d06f1cea0d6c307b33279ffdd74b10cfe1639cdefc7e5e44c |

The exact filenames, descriptor/build/package identities, dependency graph,
source and package inventories, fixture hashes and validation results are fixed
by [artifacts.json](artifacts.json), [identities.json](identities.json),
[distribution review](distribution.md) and [verification](verification.md).
The protocol package identity is
`0a65ae55eb5042b9f3b3987d2b4b81afdbcf05b9c94c2d44c30392b30651e2cd`;
the build identity is
`79ba9c5c4abfc8e20a61e266a9df6ef44116f10d34c625c2e805b1b72af8e34f`.
The adjacent descriptor identifies the same aggregate provenance set.

## Actual Named Reviews

These are actual separate Codex agent reviews, not human signatures, legal
opinions or invented external approvals. Codex `/root` authored the release
recipes and records and performed the final synthesis for Kyle Altendorf under
the user's explicit implementation, release and confident admin-merge authority.

| Reviewer | Date | Outcome and inspected evidence |
| --- | --- | --- |
| Codex `/root/protocol_research` | 2026-10-08 | APPROVE replacement artifacts: independently verified both archive digests, all 127 package-file hashes, 2,186 source hashes, 2,185 mode entries, 36 registry package checksums and 1,945 vendor-file checksums; reviewed license texts, retained attribution, exact provenance, source availability, descriptor/build/binary and evidence consistency |
| Codex `/root/geometry_research` | 2026-10-08 | APPROVE replacement artifacts: verified both archive pairs, all 128 package and 2,187 source files including inventory manifests, normalized modes, unchanged compilation inputs, metadata/evidence hashes; reviewed five successful and eleven failed CLI cases, repeated reports, five target comparisons and 47 release-profile tests |
| Codex `/root` | 2026-10-08 | APPROVE: reviewed scope/diff, preserved inherited relationship classifications, verified reproducible source/package/binary/report bytes, actual extracted-package invocation including Ubuntu 24.04, official schemas, bounded failures and exact-target evidence; no runtime implementation changed in the release work |

All reviewers bind the exact approved package/source hashes above. No remaining
unknown, provisional, inconsistent or adverse fact was identified in this
candidate's required evidence. Compatibility is limited to the documented
bounded core profile and the recorded Bambu Studio 2.7.1.62 synthetic cases.

## Superseded Candidate And Resolved Findings

An earlier candidate package SHA-256
`fd23ed0c9c76cc14de34ca4dda0bcac14df34a1a94993346d9b650a97fdad170`
and source SHA-256
`3490cd37de18b52dc9a2f7e10a7e04127d5698177de239920cab3256ef56c377`
received geometry approval but **BLOCKED** distribution review from
`/root/protocol_research` on the same date. Its glibc notice referenced LGPL-2.1
without supplying the full text. That candidate is not released. Complete
referenced common-license texts were added, both archives and identities were
regenerated twice, all identity-bound CLI checks were repeated, and both agents
reviewed and approved the replacement hashes. The finding was resolved, not
waived. The executable and compilation inputs remain identical.

Earlier recipe review also corrected source executable-mode retention, matching
result assertions, full source-file-set checking, the 256 KiB failure-result
bound, positive-only schema checking and deterministic negative-fixture ZIP
timestamps. These corrections affect release tooling/evidence only. The initial
no-preset native target reload crash and the existing-output smoke-test rejection
remain recorded as failed attempts in their verification records.

## Release Boundary

Publish the exact package and complete corresponding source together under the
immutable release tag after the signed evidence PR passes required CI and
merges. Enable GitHub release immutability before publishing so its tag and
assets are protected; upload and verify the complete draft before publication.
No unreviewed artifact or changed hash may use this approval.

This approval covers the generator distribution. It supplies no service
manifest entry, selection, installation, deployment, advertisement, cache use,
generated-artifact publication or production authorization. Those reviews remain
with `onshape-export`.

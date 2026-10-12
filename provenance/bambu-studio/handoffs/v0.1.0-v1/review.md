# Bambu Neutral Handoff Review v1

- Record ID: `BBL-DEPLOYMENT-HANDOFF-REVIEW-v1`
- Date: 2026-10-12
- Outcome: **APPROVED for exact supplemental metadata publication**
- Authors: Codex `/root` operating for Kyle Altendorf
- Scope: existing Bambu Studio generator 0.1.0, unchanged published executable
- Public neutral decision: [review.json](../../../../handoffs/bambu-studio/v0.1.0-v1/review.json)

## Exact Reviewed Subjects

| Subject | SHA-256 |
| --- | --- |
| Public canonical handoff | 8547eae6e1d9378040922972ab9c9bc4561d54afb9c0ba9e3e7305e320eeb40f |
| Closed handoff schema | e630b0d81c9c7861dd577437c7adcd0905b90d3f8dda2c26ec314146fd3e7171 |
| Public detached release metadata | 1874abee0bdc31d52812b47729e84e42ffee5b28ef26d1f88cfd6b527f205c72 |
| Generator-local full verification report | 0b67b09fdf4a48455ed48fc32d79724e2bc9099bff60bf2d7a883ca8886ddc28 |
| Verification recipe | c897d34deb50b8ad901a83f660adc9106d39e833a950c09238a9ff787726c30a |
| Verification dependency lock | 594c763cfb63dbd34546dc2f053a4363a6248cfcbbfe5cba9e5c089a80d0bfcf |

The [generator-local record](README.md) gives exact source/contract identities,
SHA-256 values, classifications and matching evidence kinds, authorship, license
choices/notices, commands, environments and bounded conclusions. The supplement
is fixed by its own full Git commit; it does not edit the inherited release or
embed its own commit/digest in the handoff. The metadata publication can be pinned
without acquiring target source. No new generator package release is needed.

## Actual Independent Reviews

| Named reviewer | Date | Actual outcome and evidence |
| --- | --- | --- |
| Codex `/root/protocol_research` | 2026-10-12 | APPROVE: independently verified the four primary subject digests, all 14 deployment fields, copied service schema/MIT notice, inherited review/provenance pins and source hashes; reviewed neutral/licensing boundary, complete successful reported bindings, corrected negative cross-bindings and original-byte association |
| Codex `/root/geometry_research` | 2026-10-12 | APPROVE: independently checked exact JCS bytes, file hashes, static identity, recipe/report binding, two identical reports, original package/source/descriptor/binary hashes and all 128 installed files; reviewed five complete successful identity exchanges and three bounded candidate-free failures |

These are actual separate agent reviews, not invented human signatures, legal
opinions or service approvals. The author independently reviewed the complete
diff and verification results under the user's approved metadata-only PR plan.
No unknown, disputed, provisional or inconsistent binding remains in these
reviewed subjects.

The protocol reviewer initially requested the full content hash of the consulted
service static-identity document before approval. That SHA-256 was added to the
local source-influence record, then final approval was obtained; no waiver was
used. Earlier review also required anchoring the installed file inventory to
the approved release inventory and checking parsed failure invocation identities.
Both corrections preceded the final repeated runs and named subject approvals.

## Exact Original Release Association

The approved supplement pins original release `bambu-studio-v0.1.0` at
`09dc4068e950a03eaf5c50fe56f5367be45cd79c`, original release-review/identity/
provenance records at `a729af8c8dce6ac48c6521907cd6c77d984533d1`, and:

- Package SHA-256:
  `a9c859962d23de9812a28871fa6a80334b7543ab695b52db73ca2d91387c3d9c`.
- Corresponding-source SHA-256:
  `bcc3c15220f739b9df9c6b5a6b2422c05ac4c126a37e2e8c5a8640b3184535ae`.
- Executable SHA-256:
  `734aa2340c954b7d06f1cea0d6c307b33279ffdd74b10cfe1639cdefc7e5e44c`.
- Adjacent descriptor SHA-256:
  `a93a499aa55a5096b2c0c264dd7db537e305148f664a9ed2253062cd7358d8c1`.

The package and source were independently downloaded and verified against the
existing immutable-release attestation. Full package measurements were repeated
after successful and failed invocations. Original runtime, lockfile, release
records and capability provenance sets remain unchanged. No original asset was
replaced or added to the immutable release.

## Boundary And Publication

This approval resolves the missing complete neutral handoff. Service consumers
may construct their existing closed deployment document by adding only their
own absolute executable path. They must still perform service-side review,
installation checks and approval before deployment, capability advertisement,
cache use, generated-artifact publication or production use. The supplement
grants none of those authorities and introduces no new target compatibility
claim or target-format knowledge into the service repository.

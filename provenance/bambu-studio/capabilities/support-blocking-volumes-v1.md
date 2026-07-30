# Bambu Studio Support-Blocking Volumes Capability Baseline v1

## Identity And Scope

- Record ID: `BBL-SUPPORT-BLOCKING-VOLUMES-BASELINE-v1`
- Capability ID: `bambu-studio.support-blocking-volumes`
- Capability revision: `1`
- Destination crate: `crates/bambu-studio`
- Source-neutral requirements: [`BBL-MVP-INPUT-v1`](../requirements/mvp-input-v1.md)
- Terms and compatibility review: [`BBL-UPSTREAM-TERMS-v1`](../reviews/bambu-studio-v02.07.01.62.md)

This capability creates a Bambu Studio project in which every accepted logical
support-blocking volume is associated with its intended object and recognized as
a support blocker by the validated target version. It does not implement named
objects beyond shared identity requirements, support generation, slicer
settings, service transport, or publication.

Missing, unsupported, duplicate, or ambiguous role mappings must fail. A
support blocker must never be silently omitted, converted to a normal model
part, or associated with a different object.

## Upstream Baseline

- Project: `bambulab/BambuStudio`
- Component: Bambu project 3MF model-settings import/export in
  `src/libslic3r/Format/bbs_3mf.cpp` and model-volume role definitions and
  mappings in `src/libslic3r/Model.hpp` and `src/libslic3r/Model.cpp`
- Canonical URL: <https://github.com/bambulab/BambuStudio>
- Release: `v02.07.01.62` (`2.7.1.62 Public Release`)
- Pinned revision: `42d319c6692fa8e64790fddf0cdaafd2a4254bcc`
- Release URL: <https://github.com/bambulab/BambuStudio/releases/tag/v02.07.01.62>

The release and commit are the only upstream baseline for capability revision
`1`. A different upstream revision requires a superseding baseline.

## Consulted Upstream References

The issue 3 baseline research consulted these exact references:

- Upstream identity and lineage in
  [`README.md` line 8](https://github.com/bambulab/BambuStudio/blob/42d319c6692fa8e64790fddf0cdaafd2a4254bcc/README.md#L8-L8),
  plus its license statement and networking-plugin boundary in the same file's
  [license section](https://github.com/bambulab/BambuStudio/blob/42d319c6692fa8e64790fddf0cdaafd2a4254bcc/README.md#L42-L52),
  Git blob SHA-1 `68bc1bd632af66dd00ff1b308169404711c4739f`.
- Governing license text:
  [`LICENSE`](https://github.com/bambulab/BambuStudio/blob/42d319c6692fa8e64790fddf0cdaafd2a4254bcc/LICENSE),
  Git blob SHA-1 `dba13ed2ddf783ee8118c6a581dbf75305f816a3`.
- Model-volume type and name keys:
  [`bbs_3mf.cpp` lines 347-354](https://github.com/bambulab/BambuStudio/blob/42d319c6692fa8e64790fddf0cdaafd2a4254bcc/src/libslic3r/Format/bbs_3mf.cpp#L347-L354).
- Current subtype parsing and association with the containing object:
  [`bbs_3mf.cpp` lines 4346-4369](https://github.com/bambulab/BambuStudio/blob/42d319c6692fa8e64790fddf0cdaafd2a4254bcc/src/libslic3r/Format/bbs_3mf.cpp#L4346-L4369).
- Legacy volume-name and metadata-key role parsing:
  [`bbs_3mf.cpp` lines 5221-5236](https://github.com/bambulab/BambuStudio/blob/42d319c6692fa8e64790fddf0cdaafd2a4254bcc/src/libslic3r/Format/bbs_3mf.cpp#L5221-L5236).
- Volume subtype and name serialization:
  [`bbs_3mf.cpp` lines 7988-7998](https://github.com/bambulab/BambuStudio/blob/42d319c6692fa8e64790fddf0cdaafd2a4254bcc/src/libslic3r/Format/bbs_3mf.cpp#L7988-L7998),
  Git blob SHA-1 `1b5831b273d00a3ab8828c191ee8a91cbcea6dc5`.
- Model-volume type definitions and predicates:
  [`Model.hpp` lines 328-336](https://github.com/bambulab/BambuStudio/blob/42d319c6692fa8e64790fddf0cdaafd2a4254bcc/src/libslic3r/Model.hpp#L328-L336)
  and [`Model.hpp` lines 982-986](https://github.com/bambulab/BambuStudio/blob/42d319c6692fa8e64790fddf0cdaafd2a4254bcc/src/libslic3r/Model.hpp#L982-L986),
  Git blob SHA-1 `6bdc01b502017968bfd76bab73ebdee2c9c9bb34`.
- String-to-role and role-to-string mappings:
  [`Model.cpp` lines 3400-3426](https://github.com/bambulab/BambuStudio/blob/42d319c6692fa8e64790fddf0cdaafd2a4254bcc/src/libslic3r/Model.cpp#L3400-L3426),
  Git blob SHA-1 `083102f4dc295c82573dd8a83dc87b3b1553e9ac`.

These are baseline references, not complete implementation evidence. Actual use
must be recorded in `BBL-SUPPORT-BLOCKING-VOLUMES-SOURCE-INFLUENCE-v1` with
exact relevant lines, symbols, a stated SHA-256 content-hash method and result,
and a description of what was learned, reused, or adapted.

## Expected Relationships

These classifications are planning expectations only:

- Target project fields and layout that encode volume roles: `schema_fact`
  with `source_informed_schema_fact` evidence.
- Serialized support-blocker identifiers and other target constants:
  `adapted_constant` with `source_informed` evidence.
- Role serialization or association logic structurally adapted from source:
  `adapted_algorithm` with `source_informed` evidence.
- Source text retained verbatim, if any: `direct_source_reuse` with
  `source_informed` evidence.
- Request validation established without relevant source influence, if any:
  `independently_derived_behavior` with `independently_derived_behavior`
  evidence.

The final record must classify each implementation, constant, schema, fixture,
test, and behavioral claim separately. Independent or clean-room evidence must
not be claimed unless that method is actually used and documented.

## Planned Local And Evidence Records

- Source influence: `BBL-SUPPORT-BLOCKING-VOLUMES-SOURCE-INFLUENCE-v1`
- Implementation: `BBL-SUPPORT-BLOCKING-VOLUMES-IMPLEMENTATION-v1`
- Verification: `BBL-SUPPORT-BLOCKING-VOLUMES-VERIFICATION-v1`
- Notices: `BBL-MVP-NOTICES-v1`
- Provenance set: `BBL-MVP-PROVENANCE-SET-v1`
- Release review: `BBL-MVP-RELEASE-REVIEW-v1`
- Planned implementation area: `crates/bambu-studio`

These names do not assert that any record or implementation exists.

## Verification Requirements

Verification must be reproducible and retain inputs, outputs, hashes, commands,
environment and tool identities, dates, and immutable results. At minimum it
must demonstrate against Bambu Studio `2.7.1.62` at the pinned revision that:

- A valid model plus blocker loads with both geometries present and the blocker
  recognized as a support-blocking volume associated with the intended object.
- Multiple model objects and blockers retain unambiguous object and role
  associations after load and save/reload where applicable.
- Missing, duplicate, ambiguous, unsupported, or conflicting role mappings fail
  before a candidate project is completed.
- A requested blocker is never omitted or emitted as an ordinary model part.

The verification record must identify the exact tested source build or official
binary artifact and its hash. No binary, fixture, experiment, or result has been
selected or produced by this baseline.

## Gates

Affected public incorporation is blocked until actual relationships, consulted
references, local paths, authorship, source influence, terms, attribution, and
notices are complete. Release remains blocked until all planned records contain
complete immutable evidence and named review. Service approval and generated
artifact publication are separate and are not authorized by this baseline.

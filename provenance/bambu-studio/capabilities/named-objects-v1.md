# Bambu Studio Named Objects Capability Baseline v1

## Identity And Scope

- Record ID: `BBL-NAMED-OBJECTS-BASELINE-v1`
- Capability ID: `bambu-studio.named-objects`
- Capability revision: `1`
- Destination crate: `crates/bambu-studio`
- Source-neutral requirements: [`BBL-MVP-INPUT-v1`](../requirements/mvp-input-v1.md)
- Terms and compatibility review: [`BBL-UPSTREAM-TERMS-v1`](../reviews/bambu-studio-v02.07.01.62.md)

This capability creates a Bambu Studio project in which every accepted logical
model object has a stable, nonempty requested name that remains distinguishable
when the project is opened by the validated target version. It does not define
support-volume roles, slicer settings, service transport, publication, or a
general Bambu Studio project implementation.

Missing, empty, duplicate, conflicting, or ambiguous object identity or name
mappings fail. Duplicate display text remains permitted for distinct
identities. The capability does not silently omit an object name, substitute an
unrelated name, or collapse distinct logical objects.

## Upstream Baseline

- Project: `bambulab/BambuStudio`
- Component: Bambu project 3MF model and model-settings import/export in
  `src/libslic3r/Format/bbs_3mf.cpp`, with model identity definitions in
  `src/libslic3r/Model.hpp`
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
- Object-name import and fallback behavior:
  [`bbs_3mf.cpp` lines 2164-2196](https://github.com/bambulab/BambuStudio/blob/42d319c6692fa8e64790fddf0cdaafd2a4254bcc/src/libslic3r/Format/bbs_3mf.cpp#L2164-L2196).
- Object-name parsing from project model XML:
  [`bbs_3mf.cpp` lines 3673-3676](https://github.com/bambulab/BambuStudio/blob/42d319c6692fa8e64790fddf0cdaafd2a4254bcc/src/libslic3r/Format/bbs_3mf.cpp#L3673-L3676).
- Object-name serialization into model settings:
  [`bbs_3mf.cpp` lines 7961-7969](https://github.com/bambulab/BambuStudio/blob/42d319c6692fa8e64790fddf0cdaafd2a4254bcc/src/libslic3r/Format/bbs_3mf.cpp#L7961-L7969),
  Git blob SHA-1 `1b5831b273d00a3ab8828c191ee8a91cbcea6dc5`.
- In-memory object name field:
  [`Model.hpp` lines 346-352](https://github.com/bambulab/BambuStudio/blob/42d319c6692fa8e64790fddf0cdaafd2a4254bcc/src/libslic3r/Model.hpp#L346-L352),
  Git blob SHA-1 `6bdc01b502017968bfd76bab73ebdee2c9c9bb34`.

These baseline references are completed by the source-influence record, which
adds exact relevant lines, whole-file SHA-256 hashes, local relationships, and a
description of what was learned or adapted.

## Implemented Relationships

The actual relationship-level classifications and evidence kinds are recorded
in [`BBL-NAMED-OBJECTS-SOURCE-INFLUENCE-v1`](../source-influence/named-objects-v1.md).

## Local And Evidence Records

- [Source influence](../source-influence/named-objects-v1.md):
  `BBL-NAMED-OBJECTS-SOURCE-INFLUENCE-v1`
- [Implementation](../implementation/named-objects-v1.md):
  `BBL-NAMED-OBJECTS-IMPLEMENTATION-v1`
- [Verification](../verification/named-objects-v1.md):
  `BBL-NAMED-OBJECTS-VERIFICATION-v1`
- [Notices](../notices/named-objects-v1.md):
  `BBL-NAMED-OBJECTS-NOTICES-v1`
- [Provenance set](../provenance-sets/named-objects-v1.md):
  `BBL-NAMED-OBJECTS-PROVENANCE-SET-v1`
- Release review: `BBL-MVP-RELEASE-REVIEW-v1`
- Implementation: `crates/bambu-studio` at
  `383e480673d73096942cd11d04ebe597f467a05b`

## Verification Requirements

Verification must be reproducible and retain inputs, outputs, hashes, commands,
environment and tool identities, dates, and immutable results. At minimum it
must demonstrate against Bambu Studio `2.7.1.62` at the pinned revision that:

- Two or more objects with distinct valid names are all present and retain the
  requested names after project load and save/reload where applicable.
- Names requiring supported escaping and Unicode handling round trip according
  to the final, explicitly bounded capability contract.
- Missing or empty required names and duplicate or ambiguous identity-to-name
  mappings fail before a candidate project is completed.
- Unsupported input and malformed geometry references fail without silently
  omitting objects or names.

The [verification record](../verification/named-objects-v1.md) identifies the
exact official artifact, source-authored fixture, isolated environment, hashes,
commands, load/save/reload observations, and results.

## Gates

Public incorporation requirements for revision 1 are recorded. Generator
release remains blocked pending an actual package candidate, complete MVP
release evidence, and named release review. Service approval and generated
artifact publication are separate and are not authorized by these records.

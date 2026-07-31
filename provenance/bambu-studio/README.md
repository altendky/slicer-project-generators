# Bambu Studio Provenance Records

This directory contains records for the Bambu Studio generator. Records are
append-only and apply only to the capability revision, destination crate,
upstream component, and pinned revision that they name.

## Established Records

| Record ID | Record | Status |
| --- | --- | --- |
| `BBL-NAMED-OBJECTS-BASELINE-v1` | [Named objects capability baseline](capabilities/named-objects-v1.md) | Baseline established; implementation evidence remains pending |
| `BBL-SUPPORT-BLOCKING-VOLUMES-BASELINE-v1` | [Support-blocking volumes capability baseline](capabilities/support-blocking-volumes-v1.md) | Baseline established; implementation evidence remains pending |
| `BBL-UPSTREAM-TERMS-v1` | [Bambu Studio 2.7.1.62 terms and compatibility review](reviews/bambu-studio-v02.07.01.62.md) | Source and access terms reviewed; implementation-specific copyrights and notices remain blocking |
| `BBL-MVP-INPUT-v1` | [Source-neutral MVP input requirements](requirements/mvp-input-v1.md) | Requirements established; no wire schema is defined here |

Both capability baselines use Bambu Studio tag `v02.07.01.62`, commit
`42d319c6692fa8e64790fddf0cdaafd2a4254bcc`.

## Planned Records

The following IDs reserve the records required by later implementation,
verification, and release work. These records do not yet exist, and listing
them does not assert implementation, evidence, review, or approval:

- `BBL-NAMED-OBJECTS-SOURCE-INFLUENCE-v1`
- `BBL-NAMED-OBJECTS-IMPLEMENTATION-v1`
- `BBL-NAMED-OBJECTS-VERIFICATION-v1`
- `BBL-SUPPORT-BLOCKING-VOLUMES-SOURCE-INFLUENCE-v1`
- `BBL-SUPPORT-BLOCKING-VOLUMES-IMPLEMENTATION-v1`
- `BBL-SUPPORT-BLOCKING-VOLUMES-VERIFICATION-v1`
- `BBL-MVP-NOTICES-v1`
- `BBL-MVP-PROVENANCE-SET-v1`
- `BBL-MVP-RELEASE-REVIEW-v1`

The source-influence records must replace expected classifications with the
actual relationship-level classifications and exactly matching evidence kinds.
The implementation and verification records must add immutable local commits,
authorship, fixtures, hashes, tests, results, environments, and exact validated
target versions. The notice, provenance-set, and release records must cover the
complete candidate contents and cannot be completed by these baselines.

## Current Gates

Public incorporation of affected Bambu-derived implementation, target schemas,
fixtures, constants, or evidence remains blocked until its actual consulted
references, relationship classifications, applicable input review, local paths,
authorship, attribution, and notices are complete. Generator distribution and
release additionally require the immutable provenance set, reproducible test
results, exact package identity and hashes, distribution review, and named
release approval required by the canonical provenance policy.

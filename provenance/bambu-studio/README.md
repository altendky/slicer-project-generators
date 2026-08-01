# Bambu Studio Provenance Records

This directory contains records for the Bambu Studio generator. Records are
append-only and apply only to the capability revision, destination crate,
upstream component, and pinned revision that they name.

## Established Records

| Record ID | Record | Status |
| --- | --- | --- |
| `BBL-NAMED-OBJECTS-BASELINE-v1` | [Named objects capability baseline](capabilities/named-objects-v1.md) | Implemented and validated for target 2.7.1.62 |
| `BBL-SUPPORT-BLOCKING-VOLUMES-BASELINE-v1` | [Support-blocking volumes capability baseline](capabilities/support-blocking-volumes-v1.md) | Planning baseline retained; completed by the records below |
| `BBL-UPSTREAM-TERMS-v1` | [Bambu Studio 2.7.1.62 terms and compatibility review](reviews/bambu-studio-v02.07.01.62.md) | Source and exact AppImage test input reviewed |
| `BBL-NAMED-OBJECTS-DEPENDENCIES-v1` | [Rust dependency review](reviews/rust-dependencies-v1.md) | Locked dependency graph reviewed |
| `BBL-MVP-INPUT-v1` | [Source-neutral MVP input requirements](requirements/mvp-input-v1.md) | Requirements established; no wire schema is defined here |
| `BBL-NAMED-OBJECTS-SOURCE-INFLUENCE-v1` | [Named objects source influence](source-influence/named-objects-v1.md) | Relationship classifications complete |
| `BBL-NAMED-OBJECTS-IMPLEMENTATION-v1` | [Named objects implementation](implementation/named-objects-v1.md) | Immutable implementation commit recorded |
| `BBL-NAMED-OBJECTS-VERIFICATION-v1` | [Named objects verification](verification/named-objects-v1.md) | Automated and exact-target compatibility results recorded |
| `BBL-NAMED-OBJECTS-NOTICES-v1` | [Named objects notices](notices/named-objects-v1.md) | Source lineage and dependencies recorded |
| `BBL-NAMED-OBJECTS-PROVENANCE-SET-v1` | [Named objects provenance set](provenance-sets/named-objects-v1.md) | Public-incorporation evidence complete; not released |
| `BBL-SUPPORT-BLOCKING-VOLUMES-DEPENDENCIES-v1` | [Support-blocking Rust dependency review](reviews/support-blocking-volumes-rust-dependencies-v1.md) | Locked dependency graph reviewed |
| `BBL-SUPPORT-BLOCKING-VOLUMES-TERMS-v1` | [Support-blocking terms supplement](reviews/support-blocking-volumes-v1.md) | Source, attribution, and exact AppImage use reviewed |
| `BBL-SUPPORT-BLOCKING-VOLUMES-SOURCE-INFLUENCE-v1` | [Support-blocking source influence](source-influence/support-blocking-volumes-v1.md) | Relationship classifications complete |
| `BBL-SUPPORT-BLOCKING-VOLUMES-IMPLEMENTATION-v1` | [Support-blocking implementation](implementation/support-blocking-volumes-v1.md) | Immutable implementation commit recorded |
| `BBL-SUPPORT-BLOCKING-VOLUMES-VERIFICATION-v1` | [Support-blocking verification](verification/support-blocking-volumes-v1.md) | Automated and exact-target compatibility results recorded |
| `BBL-SUPPORT-BLOCKING-VOLUMES-NOTICES-v1` | [Support-blocking notices](notices/support-blocking-volumes-v1.md) | Source lineage and dependencies recorded |
| `BBL-SUPPORT-BLOCKING-VOLUMES-PROVENANCE-SET-v1` | [Support-blocking provenance set](provenance-sets/support-blocking-volumes-v1.md) | Public-incorporation evidence complete; not released |

Both capability baselines use Bambu Studio tag `v02.07.01.62`, commit
`42d319c6692fa8e64790fddf0cdaafd2a4254bcc`.

## Planned Records

The following IDs reserve the records required by later implementation,
verification, and release work. These records do not yet exist, and listing
them does not assert implementation, evidence, review, or approval:

- `BBL-MVP-PROVENANCE-SET-v1`
- `BBL-MVP-RELEASE-REVIEW-v1`

The later MVP provenance set and release review belong to issue #9 and must
cover the complete named-object, support-blocking, protocol, and package
candidate; capability evidence does not complete or authorize that release
work.

## Current Gates

The named-objects and support-blocking public-incorporation records are
complete. Generator distribution and release remain blocked pending a complete
MVP candidate, package identity and hashes, distribution review, and named
release approval. The protocol, service approval, deployment, and
generated-artifact publication remain outside this repository and are not
authorized.

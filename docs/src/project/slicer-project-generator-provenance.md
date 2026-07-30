# Slicer Project Generator Provenance Policy

> **Status: Normative.** This is the canonical generator-side policy for source
> access, evidence, target-derived implementation, target fixtures and schemas,
> generator package builds, generator release review, and provenance sets in
> this repository. It is process guidance, not legal advice.

## Authority And Ownership

This repository owns target-derived generator implementation, its evidence and
provenance records, target fixtures and schemas, generator package builds, and
generator package release decisions.

[`onshape-export`](https://github.com/altendky/onshape-export) owns the
source-neutral generator protocol, the approved-generator manifest, runtime
sandboxing, independent output validation and hashing, caching, generated
artifact publication and revocation, and interface, distribution, and
deployment review. A generator package release here does not approve a service
integration or the publication or production use of generated artifacts.
Conversely, a service review does not replace this repository's evidence or
generator release review.

No service integration commit is pinned by this policy. Any future integration
must record its own immutable service-side reference after the corresponding
service changes exist.

## Required Start And Input Gates

Before relevant upstream implementation source is accessed, or a
source-informed, independently derived, or clean-room capability is begun:

1. Assign a stable capability identifier and narrowly define the proposed work.
2. Create a named source-access or capability-start record under `provenance/`
   in a committed revision of this repository. The record must identify the
   requester, participants and roles, reviewer, approver, target and pinned
   revision, permitted inputs and path categories, purpose, isolated working
   location, permitted provisional outputs, controls, and approval date.
3. Obtain explicit approval in that committed record before the relevant access
   or work. Temporary files, private notes, chat, pull-request discussion, and
   uncommitted records do not satisfy this gate.
4. Record each participant's prior and permitted source access and any
   separation or communication controls.
5. Assign provisional classifications and evidence kinds and name the records
   that will collect implementation, verification, terms, and release evidence.
6. Complete and approve the applicable input terms and license review described
   below for every evidence or implementation-input category before that
   category is accessed or used.

The start record and each terms review may be superseded only by a new,
version-controlled record that preserves the earlier decision. Approval is
specific to the named people, scope, target revision, inputs, and isolated
location; it is not general permission to inspect other source or begin another
capability. Upstream implementation must not be inspected merely to estimate
work before these gates are met.

## Provisional Evidence-Completion Work

After the approved start record and all reviews applicable to the inputs being
used are complete, named participants may perform the following work only in
the recorded isolated location:

- Access approved source and other approved inputs within the recorded scope.
- Create provisional implementation commits and target-derived schemas or
  fixtures needed to establish evidence.
- Build provisional generator packages and run fixtures and tests needed to
  complete local implementation and validation records.

This permission resolves the need to create immutable implementation and test
evidence after access without treating provisional work as released. All such
work and outputs must be conspicuously marked provisional and
non-distributable. They must remain isolated from release channels and from the
`onshape-export` production service.

Until final records and reviews are complete, the capability and its outputs
must not be advertised; generator packages must not be distributed or released;
the service must not approve or run the generator; and fixtures, generated
artifacts, or other outputs must not be published or used in production.

## Two-Level Taxonomy

Every relationship among a capability and its implementation, constants,
schemas, fixtures, tests, or behavioral claims has one classification and one
matching evidence kind.

The classifications are:

- `direct_source_reuse`: source retained verbatim or with mechanical changes.
- `adapted_algorithm`: logic derived from source-level algorithms, structure,
  control flow, or design.
- `adapted_constant`: constants, tables, identifiers, templates, or serialized
  values taken or transformed from implementation source.
- `schema_fact`: a fact about a target-derived format or schema.
- `independently_derived_behavior`: behavior established without relevant
  implementation-source use through permitted documentation, experiments, or
  interoperability observations.
- `clean_room_implementation`: implementation produced under documented
  observer and implementer separation.

The evidence-kind mapping is exact:

| Classification | Permitted evidence kind |
| --- | --- |
| `direct_source_reuse` | `source_informed` |
| `adapted_algorithm` | `source_informed` |
| `adapted_constant` | `source_informed` |
| `schema_fact` | `source_informed_schema_fact` or `independently_established_schema_fact` |
| `independently_derived_behavior` | `independently_derived_behavior` |
| `clean_room_implementation` | `clean_room` |

Classification describes origin evidence and does not decide copyright status,
license compatibility, or any other legal conclusion. A broad package-level
statement cannot replace relationship-level records. Actual source influence
must be recorded under the matching source-informed evidence kind.

## Input Terms And License Review

Before any category of material is accessed as evidence or used as an
implementation input, a named, approved, version-controlled review must address
that material, the planned use, every affected destination crate, and any
planned retention or distribution. Categories include implementation source,
specifications, documentation, EULAs, software, services, fixtures, schemas,
observations, and captured or generated outputs.

The minimum access needed to locate, retrieve, and review license texts, EULAs,
access terms, copyright statements, exceptions, and notices is permitted solely
to complete this gate. It must be limited to that legal and metadata material
and recorded in the review. It does not permit inspection of implementation
content, schemas, fixtures, behavior, or other substantive evidence before the
approved start and input gates are complete. If governing material cannot be
identified without substantive access, stop and obtain qualified review rather
than treating the gate as satisfied.

For implementation source, identify the exact target package or component and
pinned revision and review governing licenses, copyrights, exceptions, access
terms, attribution, source and modification obligations, and notices. Exact
files, symbols, and lines consulted are provenance references within that
package-level review; file-specific terms and notices must be incorporated when
applicable.

For every other category, identify the exact version or immutable artifact and
review all governing licenses, EULAs, service and access terms, confidentiality
limits, copyright, exceptions, permissions to observe or capture behavior, and
permissions to retain, modify, test, or distribute inputs and outputs. Public
availability is not permission by itself.

An unresolved or adverse review blocks use of that input. It may require a new
evidence plan, different isolation controls, changed crate licensing, omitted
fixtures, additional notices, or qualified legal review. Required terms and
notices must be retained and supplied with generator packages or fixtures where
applicable.

## Evidence Requirements

### Source-Informed Evidence

`source_informed` and `source_informed_schema_fact` records must contain:

- The official repository owner, name, and canonical URL.
- The target release or tag, if any, and full commit hash.
- Repository-relative file, symbol when available, and exact relevant lines.
- An immutable official permalink and a stated-algorithm content hash.
- The approved source-access record and applicable terms and license review.
- A description of reuse, adaptation, or learned schema facts.
- Destination crate, local paths and immutable commits when available,
  authorship, review, fixtures, tests, and exact validated target versions.

Unresolved fields may be explicit in provisional records while isolated work
generates the missing evidence. They must be resolved for generator release.

### Independent Evidence

`independently_established_schema_fact` and
`independently_derived_behavior` records must contain:

- Immutable official specification or documentation references where
  available, including versions, sections, stable identifiers, hashes when
  obtainable, and their approved terms reviews.
- A reviewed explanation of the evidence basis when no immutable authoritative
  reference exists.
- Reproducible experiment plans, inputs, observed outputs, results, dates,
  environments, and tool versions.
- Fixture identifiers and hashes linked to experiments and tests.
- Authorship, independent review, and participant source-access declarations.

These records must not invent implementation-source references. Black-box
observation and its fixtures remain subject to the start and input review gates.

### Clean-Room Evidence

`clean_room` records must contain:

- Versioned, hashed source-neutral requirements and every input supplied to the
  implementation team.
- Named observer, specification, implementation, and review roles; access
  controls; communication boundaries; retained communications; and participant
  attestations.
- Reproducible experiments, fixture identifiers and hashes, tests, results, and
  exact target versions.
- The specification team's complete classification and evidence chain,
  including source-informed records for implementation source it consulted.
- Independent review of separation, requirements, evidence, implementation,
  and verification.

The specification team's source access does not imply implementer access, but
source-derived expression must not cross a prohibited boundary.

## Schemas And Fixtures

The source-neutral protocol and its schemas belong to `onshape-export`. They
describe the process boundary and must not absorb target-derived slicer facts,
constants, templates, or compatibility claims.

Target-derived slicer schemas, serialized layouts, templates, and fixtures
belong in this repository and require capability-level classification, evidence,
input terms review, hashes, and release review. A target fixture used only for
provisional testing is still an evidence input or output and remains
non-publishable until its rights and final records permit publication.

## Local Implementation And Provenance Sets

Each implemented relationship must eventually record repository-relative local
paths, symbols or lines, full implementation commits, modifications when
applicable, authors and reviewers with dates, fixture hashes, test identifiers
and results, and exact target versions. Planned paths may appear in the approved
start record; release evidence must replace them with immutable references.

Records are append-only review evidence. Corrections create a superseding record
or provenance-set version and retain the earlier record.

Every advertised capability identifier and revision must resolve to one
immutable provenance-set version covering all implementation, constants,
target-derived schemas, templates, fixtures, tests, terms reviews, notices, and
evidence needed by that capability. Generator package metadata must identify the
same set. Sharing implementation among targets does not make their evidence,
terms, or compatibility results interchangeable.

## Generator Build And Release Review

Restricted builds and tests are permitted only as described under Provisional
Evidence-Completion Work. A distributable generator release requires:

- Final classifications and exactly matching evidence kinds for every covered
  relationship.
- Complete immutable evidence, local implementation records, terms and license
  reviews, required notices, fixtures, tests, and target-version results.
- An immutable provenance-set version linked to every included capability.
- The exact candidate package identity, metadata, dependency resolution,
  reproducible validation results, and artifact hashes.
- Review that destination crate licenses and the planned generator distribution
  comply with all applicable obligations.
- Named generator release reviewers, approval outcome, and date.

Unknown, disputed, provisional, incomplete, unverifiable, or internally
inconsistent records block generator distribution and release. Automation must
not provide a waiver around terms, licensing, notices, evidence, or provenance
requirements.

## Service And Publication Gates

A released generator is only a candidate input to `onshape-export`. That project
must separately review and approve the source-neutral interface, approved
generator manifest entry, package acquisition and distribution, sandbox and
deployment configuration, and independent validation and hashing. It owns
cache, publication, and revocation decisions for generated artifacts.

Generator release approval does not authorize capability advertisement by the
service, service deployment, generated-artifact publication, or production use.
Those remain blocked until the service-side records and reviews are complete and
linked to the immutable generator package and provenance-set identities.

Do not invent a source, revision, license, terms result, experiment, fixture,
hash, attestation, approval, compatibility result, package, or service
integration reference to satisfy any gate.

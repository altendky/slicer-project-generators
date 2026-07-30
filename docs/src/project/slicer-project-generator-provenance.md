# Slicer Project Generator Provenance Policy

> **Status: Normative.** This is the canonical generator-side policy for source
> access, evidence, target-derived implementation, target fixtures and schemas,
> generator package builds, generator release review, and provenance sets in
> this repository. It is process guidance, not legal advice.

## Authority And Ownership

This repository owns target-derived generator implementation, its evidence and
provenance records, target fixtures and schemas, generator package builds, and
generator package release decisions.

This repository exists to develop derivative generator implementations from
each crate's named upstream AGPL reference project. Contributors may inspect and
reference that project when working on the corresponding crate. Source-informed
derivation is the expected provenance path, not an exceptional one. This policy
records source influence and the obligations applicable to incorporation,
repository publication, distribution, and release. It does not itself grant
rights under upstream terms or replace review of the exact upstream revision
and planned use.

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

## Capability And Upstream Baseline

Assign each implemented capability a stable identifier and narrowly define its
scope. Maintain a version-controlled provenance record under `provenance/` that
identifies the destination crate, named upstream project or component,
canonical URL, pinned revision, capability scope, consulted upstream
references, source-influence classifications, and applicable licensing and
compatibility records. Name the records that collect implementation,
verification, terms, and release evidence.

The baseline may be developed alongside research and implementation and is not
an authorization gate. This policy does not require advance approval for
ordinary inspection or reference of the corresponding crate's named upstream
project. Before affected upstream-derived implementation, schemas, fixtures, or
evidence are pushed to a public branch, submitted for review, merged, or
otherwise distributed, the baseline must identify their exact upstream
component and revision, consulted references, and source-influence
classifications. Resolve the governing license, copyright, attribution, notice,
and destination-crate compatibility requirements applicable to that material.
Record uncertainties and block the affected public incorporation or
distribution until they are resolved.

Baseline and review records may be superseded only by new, version-controlled
records that preserve the earlier record and identify their replacement. A
baseline applies only to its named capability, destination crate, upstream
component, and pinned revision.

## Development And Distribution Boundary

Normal development may include source-informed implementation, target-derived
schemas and fixtures, builds, and tests for the crate's named reference project.
Incomplete evidence and unverified compatibility claims must be described
accurately.

Public source and evidence incorporation requires the applicable baseline,
source-influence provenance, license compatibility, attribution, and notice
requirements described above. Distribution of a generator package requires the
complete immutable evidence, validation, provenance-set, and named release
review described below. The service must not approve or run an unreleased
generator, and generated-artifact publication and production use remain subject
to the separate `onshape-export` review.

## Two-Level Taxonomy

Every relationship among a capability and its implementation, constants,
schemas, fixtures, tests, or behavioral claims has one classification and one
matching evidence kind.

Source-informed classifications are expected for derivative work in this
repository. Independent and clean-room classifications apply only when those
methods were actually used. The taxonomy records source influence; it does not
rank one permitted method above another or determine license compatibility.

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

For each capability and affected destination crate, maintain a named,
version-controlled review of the terms and license facts for upstream source
and other material that is incorporated, retained, tested, published, or
distributed. Identify the exact material and version, planned use, licenses,
copyrights, exceptions, notices, and resulting attribution, compatibility,
retention, modification, testing, and distribution requirements. Categories
may include implementation source, specifications, documentation, EULAs,
software, services, fixtures, schemas, observations, and captured or generated
outputs.

Access to a crate's named upstream source for ordinary research and development
does not require advance approval under this policy. Known access restrictions
or other applicable terms must still be observed. If governing material or the
resulting obligations cannot be determined for a planned incorporation or
distribution, obtain qualified review rather than treating the question as
resolved.

For implementation source, identify the exact target package or component and
pinned revision and review governing licenses, copyrights, exceptions, access
terms, attribution, source and modification obligations, and notices. Exact
files, symbols, and lines consulted are provenance references within that
package-level review; file-specific terms and notices must be incorporated when
applicable.

For every other category actually used, identify the exact version or immutable
artifact and review applicable licenses, EULAs, service and access terms,
confidentiality limits, copyright, exceptions, and permissions relevant to
observing, capturing, retaining, modifying, testing, publishing, or distributing
inputs and outputs. Public availability alone does not resolve those questions.

An unresolved or adverse review blocks the affected public incorporation,
publication, distribution, or release. It may require a new evidence plan,
specific access controls, changed crate licensing, omitted fixtures, additional
notices, or qualified legal review. Required terms and notices must be retained
and supplied with generator packages or fixtures where applicable.

## Evidence Requirements

### Source-Informed Evidence

`source_informed` and `source_informed_schema_fact` records must contain:

- The official repository owner, name, and canonical URL.
- The target release or tag, if any, and full commit hash.
- Repository-relative file, symbol when available, and exact relevant lines.
- An immutable official permalink and a stated-algorithm content hash.
- The capability/upstream baseline and applicable terms, license,
  compatibility, attribution, and notice review.
- A description of reuse, adaptation, or learned schema facts.
- Destination crate, local paths and immutable commits when available,
  authorship, review, fixtures, tests, and exact validated target versions.

Unresolved fields may be explicit during local development. Fields needed to
identify and classify publicly incorporated material must be resolved before
that incorporation; all required fields must be resolved for generator release.

### Independent Evidence

`independently_established_schema_fact` and
`independently_derived_behavior` records must contain:

- Immutable official specification or documentation references where
  available, including versions, sections, stable identifiers, hashes when
  obtainable, and their applicable terms reviews.
- A reviewed explanation of the evidence basis when no immutable authoritative
  reference exists.
- Reproducible experiment plans, inputs, observed outputs, results, dates,
  environments, and tool versions.
- Fixture identifiers and hashes linked to experiments and tests.
- Authorship, independent review, and a description of relevant source use.

These records must not invent implementation-source references. Black-box
observation and its fixtures remain subject to applicable terms, provenance,
publication, and distribution requirements.

### Clean-Room Evidence, When Used

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
source-derived expression must not cross a recorded clean-room boundary. These
controls apply only when a capability deliberately uses clean-room evidence or
an applicable review requires them; they are not requirements for ordinary
source-informed derivative development.

## Schemas And Fixtures

The source-neutral protocol and its schemas belong to `onshape-export`. They
describe the process boundary and must not absorb target-derived slicer facts,
constants, templates, or compatibility claims.

Target-derived slicer schemas, serialized layouts, templates, and fixtures
belong in this repository and require capability-level classification, evidence,
applicable terms review, hashes, and release review. Before publishing or
distributing them, resolve their retention, attribution, notice, and publication
requirements and retain the resulting evidence with the capability record.

## Local Implementation And Provenance Sets

Before public incorporation, each implemented relationship must record its
repository-relative local paths or symbols, modifications when applicable,
authorship, upstream references, and source-influence classification. Release
evidence must add full implementation commits, reviewers and dates, fixture
hashes, test identifiers and results, and exact target versions. Planned paths
may appear in the capability baseline but must be replaced with immutable
references for release.

Records are append-only review evidence. Corrections create a superseding record
or provenance-set version and retain the earlier record.

Every advertised capability identifier and revision must resolve to one
immutable provenance-set version covering all implementation, constants,
target-derived schemas, templates, fixtures, tests, terms reviews, notices, and
evidence needed by that capability. Generator package metadata must identify the
same set. Sharing implementation among targets does not make their evidence,
terms, or compatibility results interchangeable.

## Generator Build And Release Review

Builds and tests are ordinary development activities. A distributable generator
release requires:

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

# Bambu MVP Source-Neutral Input Requirements v1

## Identity

- Record ID: `BBL-MVP-INPUT-v1`
- Requirements revision: `1`
- Consumers: `bambu-studio.named-objects` revision `1` and
  `bambu-studio.support-blocking-volumes` revision `1`

This document defines logical information required by the Bambu MVP generator.
It does not define a transport encoding, target archive layout, target schema,
serialized constants, service API, or Bambu-specific validation claim.

## Logical Input

Each request provides a finite collection of logical geometry entries. Every
entry has:

- An opaque stable identity unique within the request.
- A geometry payload or immutable geometry reference accepted by the eventual
  versioned transport contract.
- Exactly one logical role: printable model geometry or support-blocking
  geometry.

An entry may carry a user-visible name. The named-objects capability requires a
nonempty name for every printable model entry. A support-blocking entry's name
is optional unless a later capability revision explicitly requires it.

Each support-blocking entry additionally identifies exactly one printable model
entry that it modifies. The association uses stable identities, not collection
ordering or names. A printable model may have zero or more blockers. A blocker
must not target another blocker.

Names are Unicode text. The final transport contract must define normalization,
length, and encoding limits before implementation conformance is claimed. The
generator must preserve each accepted name's association with its stable
identity; duplicate display text is permitted when identities remain distinct.
The generator must not infer identity from display text.

## Required Validation

The generator must reject the complete request before reporting success when
any of these conditions applies:

- An identity is absent, empty, or duplicated.
- A name required by the named-objects capability is absent or empty.
- A name-to-identity mapping is missing, duplicated, conflicting, or ambiguous.
- Geometry is absent, malformed, or unsupported.
- A role is absent, unsupported, duplicated, or contradictory.
- A blocker target is absent, unknown, ambiguous, or not printable model
  geometry.
- A requested capability or input revision is unsupported.
- Any requested object or blocker cannot be represented without omission,
  role conversion, or ambiguous association.

Failure must be explicit and structured by the eventual source-neutral service
protocol. Partial success must not silently remove requested objects, names, or
blockers.

## Output Expectations

For a successful named-objects request, each accepted printable model identity
corresponds to one distinguishable named logical object in the candidate target
project. For a successful support-blocking-volumes request, each accepted
blocker remains associated with its target model and is represented with the
support-blocking role.

These are logical expectations only. Target-derived fields, constants, archive
paths, templates, fixtures, and compatibility evidence belong to the capability
records and implementation in this repository, not to a neutral service
contract.

## Versioning And Evidence Boundary

Changing a required field, role meaning, validation rule, or success condition
requires a new requirements revision. A wire schema may reference this record
but must have its own version and review.

This requirements record is not evidence that either capability is implemented
or that any target version accepts generated output. Those claims require the
separate source-influence, implementation, verification, provenance-set, and
release records named by the capability baselines.

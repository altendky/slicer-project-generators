# Bambu 0.1.0 Neutral Deployment Handoff v1

This supplement supplies the unchanged published generator's complete reviewed
deployment metadata. Service consumers need only this directory and the pinned
release assets; they do not need target implementation source or fixtures.

Use [handoff.json](handoff.json), its [closed schema](handoff.schema.json),
[detached SHA-256](handoff.sha256), and
[reviewed release metadata](release-metadata.json). The handoff file is exact
RFC 8785 JCS UTF-8: no BOM, indentation or trailing newline. Its digest is over
those file bytes, without a domain separator. The digest is external to the
payload. The schema fixes this version's reviewed values and rejects unknown
fields at every object level. A correction requires a new supplement version.
The detached [named review](review.json) binds these exact metadata digests to
the original release bytes and bounded verification conclusions.

## Construct The Service Document

`immutableDeployment` contains exactly the 14 immutable fields of the pinned
Onshape Export deployed-generator v1 contract. Add only the service's own
absolute `executablePath` to form its closed configuration:

```console
jq --arg executablePath "$SERVICE_GENERATOR_EXECUTABLE" \
  '.immutableDeployment + {executablePath: $executablePath}' handoff.json
```

The service independently chooses and checks that path. No operational path is
part of the handoff. Validate the document using the unchanged pinned
[service schema](deployed-generator.schema.json), plus its strict JSON,
visible-ASCII identity and sorted/unique capability checks. Preserve the complete
capability array; subsets and inferred identities are not this reviewed binding.

`packageSha256` hashes the distributed package archive; `packageIdentity`
identifies its adjacent descriptor. `binarySha256` measures the executable;
`binaryIdentity` is a separately declared opaque identity. Their equal values
in this release do not merge the two fields. `settingsIdentity` varies with the
invocation and is excluded from static deployment metadata.

The service's static identity remains SHA-256 of
`onshape-export-deployed-generator-v1` followed by a NUL byte and JCS of the
14 immutable fields. For this handoff it is
`fd088d4e2066db7de9a26436ad081f045f1fac472429f249d58f623b3c6f8beb`.
Changing only the executable path preserves this identity. This existing scope
differs from the raw handoff-file digest and is not revised by the supplement.

## Package And Interface

The handoff pins the existing immutable tag/commit, package and corresponding-
source asset URLs, lengths/digests, executable/descriptor archive members,
descriptor digest and original release/provenance review references with hashes.
Keep the hash-verified `package.json` adjacent to the unchanged executable.
All installation paths are chosen outside this metadata.

The `interface` block declares the fixed request/result arguments, fresh private
invocation directory and immutable staged-input requirements, protocol/settings
versions, transport roles, parameterless media types, result and byte limits,
and ordinary exit/failure behavior. Treat dialect, input-kind/schema,
normalization, validation, capability and provenance values as opaque identities.
This packet contains no target-format layout or geometry interpretation rules.

The reviewed conclusions record successful acceptance and matching complete
reported bindings from the actual published executable, independently measured
artifact/descriptor/binary digests, and bounded ordinary failures. They do not
create service deployment approval or make a negative identity probe evidence
of successful acceptance. Unknown or contradictory bindings remain unavailable
until reviewed.

## Ownership And Publication

The original generator package, source and descriptor bytes remain unchanged.
This directory is a supplemental version-controlled metadata publication; pin
its full Git commit and detached digest when consuming it. Original release
approval and supplemental handoff review are separate from service deployment,
capability advertisement, cache use, publication and production authorization.

Repository-authored metadata, schema envelope and this documentation use the
repository's AGPL-3.0-only license. The copied service schema and adapted neutral
schema fragments retain the MIT grant in [LICENSE-MIT-service](LICENSE-MIT-service),
Copyright (c) 2026 The Onshape Export Authors. The handoff publishes opaque
interface metadata; it transfers no target implementation or fixtures.

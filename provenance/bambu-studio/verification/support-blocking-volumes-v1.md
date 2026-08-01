# Bambu Support-Blocking Volumes Verification v1

## Identity

- Record ID: `BBL-SUPPORT-BLOCKING-VOLUMES-VERIFICATION-v1`
- Capability: `bambu-studio.support-blocking-volumes` revision `1`
- Implementation commits:
  `50825bfc5628cc588a4ae2803b8e9477d7548b77`,
  `de0952d7ffc0476185dc9c0cf9ef2e136591361e`, and
  `2113110348d6f45f495b52481e8a5d0b9139dd7d`
- Verification date: 2026-07-31
- Verification author: OpenCode operating for Kyle Altendorf
- Independent review: fresh-context OpenCode `general` review agents on
  2026-07-31; identified implementation and evidence findings were resolved.

## Automated Tests

`cargo test --workspace` passes seven support-blocking integration tests in
addition to the six named-object regression tests. Blocker tests cover
deterministic bytes, exact archive members, containing-object/component/part
associations, normal and blocker subtypes, multiple models and blockers,
models without blockers, optional and escaped names, and returned identity
mappings. Failure tests cover absent and unsupported roles, duplicate and
contradictory identities, missing, empty, unknown, ambiguous, and non-printable
targets, absent printable models, invalid names, missing and empty meshes,
non-finite and out-of-target-range coordinates, ordinary and target-conversion
degeneracy, out-of-bounds indices, and indices outside the target's signed
range. Resource-count bounds are enforced by implementation validation but are
not allocation-tested; the smallest all-printable failing construction would
require 1,073,741,824 entries. Every failure occurs before archive creation.
Strict Clippy and formatting checks pass at the implementation commits.

The source-authored validation fixture was generated with:

```console
cargo run --quiet --example support_blocking_validation_fixture \
  -- support-blocking-input.3mf
```

Its SHA-256 is
`6910a5bcc6e1301637829079d03807edef5e6a7239b6e67a8388564ae97f2780`.
It is 4154 bytes and contains two named printable tetrahedra. Model A has named
blockers A1 and A2; model B has named blocker B. The archive has the four
members documented by the implementation record.

## Exact Target Compatibility

The hash-verified official Ubuntu 24.04 AppImage identity and terms are in
[`BBL-SUPPORT-BLOCKING-VOLUMES-TERMS-v1`](../reviews/support-blocking-volumes-v1.md).
The extracted AppImage ran in a Linux/amd64 container based on
`ubuntu:24.04@sha256:4fbb8e6a8395de5a7550b33509421a2bafbc0aab6c06ba2cef9ebffbc7092d90`.
The validation Dockerfile resolves apt packages through immutable Ubuntu
snapshot `20260731T230600Z`. Its local image ID was
`sha256:b06af6da115bd51b45d56f9967bed46a1eeb5e83dccb3f1e6044ab0bded9c89c`.
Its `dpkg-query` package-manifest SHA-256, produced by the exact command in the
validation README, was
`0801aac46d7139d1e3fdfaa792384deeb1526b9e6b98b5e076ca73882f0975a1`.
The pinned base image resolved locally to
`sha256:ef91e4b15da8323a1523adb2b371998dcd3063dae8553cc2744c178ccc065bc4`.
Both target invocations used `--network none`, dropped all capabilities, set
`no-new-privileges`, mounted inputs and the AppImage tree read-only, and did not
install or invoke the networking plugin. The retained
[validation commands](../validation/support-blocking-volumes-v1/README.md)
reproduce the workflow without
retaining the AppImage or target output.

Fixture generation used `rustc 1.94.1 (e408947bf 2026-03-25)` and Cargo
`1.94.1 (29ea6fb6a 2026-03-24)` on `x86_64-unknown-linux-gnu`. Container
execution used Docker `29.6.2` build `dfc4efb`; archive inspection used Info-ZIP
UnZip 6.00; hashing used uutils `sha256sum` 0.8.0. The validation Dockerfile is
immutable in the evidence commit, while the local image and package-manifest
hashes identify the exact realized apt environment used for these observations.
A separate `--no-cache` rebuild produced image ID
`sha256:2869a53a5b67437df513dedf1e76859371b7bc40d65b3b197bf9b5e4f54fd3cc`
and the same package-manifest SHA-256. Container-layer bytes include build-time
metadata and are not claimed reproducible; the digest-pinned base, exact CA
bootstrap version, timestamped Ubuntu snapshot, package resolution, commands,
and target inputs reproduce the tested runtime environment and observations.

The first command loaded the source-authored fixture and exported a native
project. Bambu Studio identified itself as `02.07.01.62`, read all four archive
members, generated leaf volumes 1, 2, 5 under the first object and 3, 4 under the
second, assembled two objects, preserved both model names, reached
`IMPORT_STAGE_FINISH`, and completed export. The target-saved project SHA-256 is
`f9ed8a21ea4129b3ef9a9a0d7d21924661d3e98d8ebaf48d0a1da4cdcd2cee14`.

Its `Metadata/model_settings.config` SHA-256 is
`4d464f1eb19483b0f880d0c089226c9f7a5d8c543b4edac499f9fd8ea0ec358c`.
The target reassigned IDs while preserving exact associations: saved object 4
is `Validation model A` with normal part 1 and support blockers 2 (`Blocker A1`)
and 3 (`Blocker A2`); saved object 7 is `Validation model B` with normal part 5
and support blocker 6 (`Blocker B`). Thus no requested blocker was omitted,
converted to a normal part, or moved to another model.

A second offline `--info` invocation loaded that target-produced project,
parsed all eleven members including model settings and split object models,
assembled both objects and all five volumes, and reached `IMPORT_STAGE_FINISH`.
Neither the AppImage nor generated or target-produced archives are committed or
redistributed.

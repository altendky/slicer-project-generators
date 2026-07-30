# Architecture

The repository is a Rust virtual workspace with three independent binary
packages:

| Package | Binary |
| --- | --- |
| `crates/bambu-studio` | `slicer-project-generator-bambu-studio` |
| `crates/orca-slicer` | `slicer-project-generator-orca-slicer` |
| `crates/prusa-slicer` | `slicer-project-generator-prusa-slicer` |

There is no shared crate or dependency. The repository's current capability
status is maintained in the [README](../../../README.md). Package separation
reserves boundaries; it does not establish compatibility, derivation, or
licensing conclusions.

Future capability architecture must follow the provenance policy before design
or implementation starts. Shared behavior must not be assumed across targets,
and no schema, format, fixture, or target-derived logic belongs in the scaffold
without feature-level evidence and review.

## Project Boundary

This repository owns target-derived generator implementation and evidence,
target schemas and fixtures, package builds, provenance sets, and generator
release review. The canonical rules are in the
[Provenance Policy](slicer-project-generator-provenance.md).

[`onshape-export`](https://github.com/altendky/onshape-export) owns the
source-neutral protocol and schemas, approved-generator manifest, sandbox,
independent validation and hashing, artifact cache, publication and revocation,
and interface, distribution, and deployment review. Target-derived schema facts
must not be moved into the neutral protocol. A released generator package is not
an approved service integration, and this repository does not publish generated
artifacts on behalf of the service.

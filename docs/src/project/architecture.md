# Architecture

The repository is a Rust virtual workspace with three independent packages:

| Package | Targets |
| --- | --- |
| `crates/bambu-studio` | named-objects library and protocol-stub binary |
| `crates/orca-slicer` | protocol-stub binary |
| `crates/prusa-slicer` | protocol-stub binary |

There is no shared crate. The repository's current capability status is
maintained in the [README](../../../README.md). Package separation preserves
target boundaries. Each package is intended to contain a derivative
implementation of its named upstream AGPL reference project, while its exact
reference revision, dependencies, and compatibility conclusions remain
capability-specific.

Shared behavior must not be assumed across targets. Target-derived schemas,
formats, fixtures, and logic belong in the corresponding crate and repository
provenance, not in the neutral scaffold or service protocol. Record them as
capabilities are developed and complete applicable public-incorporation,
distribution, and release review under the provenance policy.

## Project Boundary

This repository owns target-derived generator implementation and evidence,
target schemas and fixtures, package builds, provenance sets, and generator
release review. The canonical rules are in the
[Provenance Policy](slicer-project-generator-provenance.md).

The stable counterpart repository is
[`onshape-export`](https://github.com/altendky/onshape-export). Its pinned
[normative integration policy][service-integration-policy] and
[proposed generator contract][service-generator-contract] own the
source-neutral protocol and schemas, approved-generator manifest, sandbox,
independent validation and hashing, artifact cache, publication and revocation,
and interface, distribution, and deployment review. Target-derived schema facts
must not be moved into the neutral protocol. The local generator
[Provenance Policy](slicer-project-generator-provenance.md) remains canonical
for target source access, provenance, implementation, builds, and generator
release. The later service documents do not revise that policy. A released
generator package is not an approved service integration, and this repository
does not publish generated artifacts on behalf of the service.

[service-integration-policy]: https://github.com/altendky/onshape-export/blob/6555aaf80b05e3bebdeb8b4a78dafcb8d84b3320/docs/src/project/slicer-project-generator-integration.md
[service-generator-contract]: https://github.com/altendky/onshape-export/blob/6555aaf80b05e3bebdeb8b4a78dafcb8d84b3320/docs/src/project/slicer-project-generators.md

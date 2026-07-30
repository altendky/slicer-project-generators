# Contributing

Contributions must comply with [AGENTS.md](AGENTS.md) and the normative
[Provenance Policy](docs/src/project/slicer-project-generator-provenance.md).
Contributors may inspect and reference a crate's named upstream AGPL project
when developing its derivative implementation. Record the exact upstream
identity and revision, consulted references, source influence, and applicable
licensing, copyright, attribution, notice, compatibility, and distribution
facts in capability provenance.

Contributions to each workspace crate are submitted under the license declared
in that crate's `Cargo.toml`. Other repository-authored contributions are
submitted under the root `LICENSE`. Before affected upstream-derived material
is published, merged, or otherwise distributed, identify the exact target
package or component at a pinned revision and resolve applicable license,
copyright, compatibility, attribution, and notice requirements. Specifications,
documentation, EULAs, fixtures, software, and black-box observations require
review appropriate to their actual access, retention, incorporation, testing,
publication, and distribution. Clean-room separation and special isolation
controls apply only when chosen for the work or required by that review.

This repository reviews target-derived implementation, evidence, fixtures,
schemas, and generator package releases. The separate
[`onshape-export`](https://github.com/altendky/onshape-export) project reviews
the source-neutral protocol, service integration, runtime, and publication of
generated artifacts. Approval in either repository does not replace the other.

Run the checks listed in [README.md](README.md) before submitting changes.

# Contributing

Contributions must comply with [AGENTS.md](AGENTS.md) and the normative
[Provenance Policy](docs/src/project/slicer-project-generator-provenance.md).
Do not inspect upstream slicer implementation source or begin target-derived,
independent, or clean-room capability work until its approved, named,
version-controlled start record and all applicable input terms and license
reviews satisfy the policy. Only the isolated provisional work that policy
expressly permits may precede final evidence and generator release review.

Contributions to each workspace crate are submitted under the license declared
in that crate's `Cargo.toml`. Other repository-authored contributions are
submitted under the root `LICENSE`. Before target-derived work, complete the
crate-level terms and license review for the exact target package or component
at a pinned revision and preserve all applicable exceptions and notices. Record
the exact upstream files consulted in the capability provenance. That review
may require the destination crate's license declaration or repository notices
to change. Specifications, documentation, EULAs, fixtures, software, and
black-box observations also require applicable terms and license review before
use as evidence or implementation input.

This repository reviews target-derived implementation, evidence, fixtures,
schemas, and generator package releases. The separate
[`onshape-export`](https://github.com/altendky/onshape-export) project reviews
the source-neutral protocol, service integration, runtime, and publication of
generated artifacts. Approval in either repository does not replace the other.

Run the checks listed in [README.md](README.md) before submitting changes.

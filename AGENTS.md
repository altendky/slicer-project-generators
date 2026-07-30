# Source Provenance And Licensing

This repository is the AGPL-side workspace for derivative slicer project
generator implementations. Each workspace crate is intended to derive from its
named upstream AGPL reference project. The canonical generator-side source,
evidence, implementation, build, and release rules are in
`docs/src/project/slicer-project-generator-provenance.md`.

Each workspace crate declares its license in its `Cargo.toml` manifest.
Repository-authored material outside the workspace crates is licensed under the
root `LICENSE`.

Contributors may inspect and reference a crate's named upstream project while
developing that crate. Record the exact upstream identity and revision,
consulted files, source influence, and applicable licensing, copyright,
attribution, notice, and compatibility facts in capability provenance. Resolve
requirements applicable to affected material before public incorporation or
distribution, and complete the policy's test, provenance, and release evidence
before generator release. Clean-room separation or additional isolation
controls apply only when deliberately selected or required by an applicable
review.

Service approval and generated-artifact publication remain separate
responsibilities of
[`onshape-export`](https://github.com/altendky/onshape-export), governed by its
pinned [normative integration policy][service-integration-policy] and
[proposed generator contract][service-generator-contract]. Those service
documents own the neutral protocol, approval, sandbox, validation, cache,
publication, and revocation; they do not revise this repository's canonical
policy for target source access, provenance, implementation, builds, and
generator releases.

[service-integration-policy]: https://github.com/altendky/onshape-export/blob/6555aaf80b05e3bebdeb8b4a78dafcb8d84b3320/docs/src/project/slicer-project-generator-integration.md
[service-generator-contract]: https://github.com/altendky/onshape-export/blob/6555aaf80b05e3bebdeb8b4a78dafcb8d84b3320/docs/src/project/slicer-project-generators.md

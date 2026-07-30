# Source Provenance And Licensing

This repository is the isolated AGPL-side project for slicer project generator
work. The canonical generator-side source, evidence, implementation, build, and
release rules are in
`docs/src/project/slicer-project-generator-provenance.md`.

Each workspace crate declares its license in its `Cargo.toml` manifest.
Repository-authored material outside the workspace crates is licensed under the
root `LICENSE`.

Do not inspect upstream slicer implementation source or begin any slicer
capability work until the policy's approved, named, version-controlled start
record and all applicable input terms and license reviews are complete. This
applies to direct reuse, adapted work, schema facts, independently derived work,
and clean-room work.

Before using any specification, documentation, EULA, fixture, software, source,
or black-box observation as evidence or an implementation input, complete the
policy's applicable terms and license review for each affected destination
crate. For target-derived work, identify the exact target package or component
and pinned revision, record its governing licenses, copyrights, exceptions, and
notices, and preserve them as required. Record the exact upstream files used as
references in the capability provenance; they do not require separate license
metadata. The resulting review may require adjusting the destination crate's
license declaration or repository notices.

After those gates, only the policy's isolated, provisional, non-distributable
evidence-completion work is permitted until final records and generator release
review are complete. Service approval and generated-artifact publication remain
separate responsibilities of
[`onshape-export`](https://github.com/altendky/onshape-export), governed by its
pinned [normative integration policy][service-integration-policy] and
[proposed generator contract][service-generator-contract]. Those service
documents own the neutral protocol, approval, sandbox, validation, cache,
publication, and revocation; they do not revise this repository's canonical
policy for target source access, provenance, implementation, builds, and
generator releases.

[service-integration-policy]: https://github.com/altendky/onshape-export/blob/6555aaf80b05e3bebdeb8b4a78dafcb8d84b3320/docs/src/project/slicer-project-generator-integration.md
[service-generator-contract]: https://github.com/altendky/onshape-export/blob/6555aaf80b05e3bebdeb8b4a78dafcb8d84b3320/docs/src/project/slicer-project-generators.md

# Licensing

Each workspace member explicitly declares `AGPL-3.0-only`. The workspace does
not provide an inherited license. This is the chosen license for newly authored
neutral scaffold material, not a conclusion about upstream license grants.

Repository-authored support material outside the workspace crates is licensed
under the same terms. The complete AGPL-3.0-only text is stored in the root
`LICENSE`.

Each crate is intended to contain a derivative implementation of its named
upstream AGPL reference project. Contributors may inspect and reference that
project during ordinary research and development. For each implemented
capability, identify the exact upstream package or component and pinned
revision, then record applicable licenses, copyrights, exceptions, notices, and
terms together with the destination-crate compatibility and distribution
review. Record exact upstream files, symbols, and lines as capability
provenance.

`AGPL-3.0-only` in a crate manifest is that crate's declared license; it is not
by itself a conclusion about compatibility with every future upstream-derived
contribution. Preserve and satisfy applicable attribution, notice, source,
modification, fixture, and distribution requirements before affected material
is published or distributed. Unresolved questions block the affected public
incorporation, distribution, or release, not ordinary source inspection.
Generator release still requires final review under the
[Provenance Policy](slicer-project-generator-provenance.md).

Dependency and package-license policy is checked by `cargo deny`.

An official binary distribution of an AGPL-covered upstream program must
preserve the applicable AGPL rights and may be used for internal compatibility
testing under those rights. Record and hash the exact artifact, review embedded
notices and any artifact-specific terms, do not invoke excluded non-free
components, isolate it from the network, and do not redistribute it as project
evidence. General website or service terms are not by themselves a substitute
for, or a restriction on, the rights conveyed with the covered program. A
concrete contradictory artifact-specific license or click-through must still be
resolved before use.

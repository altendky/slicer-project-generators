# Licensing

Each workspace member explicitly declares `AGPL-3.0-only`. The workspace does
not provide an inherited license. This is the chosen license for newly authored
neutral scaffold material, not a conclusion about upstream license grants.

Repository-authored support material outside the workspace crates is licensed
under the same terms. The complete AGPL-3.0-only text is stored in the root
`LICENSE`.

Before accessing or using any evidence or implementation-input category,
complete the named, version-controlled terms and license review required by the
[Provenance Policy](slicer-project-generator-provenance.md). Reviews cover exact
target packages and pinned revisions as well as specifications, documentation,
EULAs, software access, services, schemas, fixtures, observations, and captured
or generated outputs. The exact upstream files, symbols, and lines remain
provenance references within the applicable package or component review.

Preserve all applicable terms, copyrights, exceptions, duties, and notices.
Depending on the result, crate declarations, distribution plans, fixture
handling, or repository notices may need adjustment. An unresolved review
blocks use of its input. After approved start and input review, provisional work
may proceed only under the policy's isolated, non-distributable controls; final
review remains mandatory for generator release.

Repository file licensing is declared through `REUSE.toml`. `reuse lint` checks
that declaration; dependency and package-license policy is checked separately by
`cargo deny`.

# Bambu Named Objects Verification v1

## Identity

- Record ID: `BBL-NAMED-OBJECTS-VERIFICATION-v1`
- Capability: `bambu-studio.named-objects` revision `1`
- Implementation commit:
  `383e480673d73096942cd11d04ebe597f467a05b`
- Verification date: 2026-07-31

## Automated Tests

`cargo test --workspace` passes six named-object integration tests. They cover
deterministic bytes, required package entries, object-ID mapping, distinct
Unicode and escaped names, duplicate display names, empty requests, missing and
empty mappings, duplicate/conflicting/ambiguous identities, invalid XML
characters, absent/empty geometry, non-finite coordinates, out-of-range indices,
and degenerate triangles. `cargo clippy --workspace --all-targets -- -D warnings`
and `cargo fmt --all --check` pass at the same implementation.

The source-authored validation fixture was generated with:

```console
cargo run --quiet --example validation_fixture -- named-objects-input.3mf
```

Its SHA-256 was
`77dad0c1544adcd9c441e658bc8489c915b6a463fb3792a88766e9a517c53f3c`.
It contains four tetrahedra named `Bracket & <left> "A"`, `支持部品 café`, and
two distinct objects both named `Duplicate display name`.

## Exact Target Compatibility

The hash-verified official Ubuntu 24.04 AppImage identity and review are in
[`BBL-UPSTREAM-TERMS-v1`](../reviews/bambu-studio-v02.07.01.62.md). The extracted
AppImage ran in a Linux/amd64 container based on
`ubuntu:24.04@sha256:4fbb8e6a8395de5a7550b33509421a2bafbc0aab6c06ba2cef9ebffbc7092d90`.
The local validation image ID was
`sha256:db50dd267ca24321f17570ec12b7dc128e12d4ae24228b8cb43b7cbf3b51f32f`.
Every target invocation used `--network none`, dropped all capabilities, set
`no-new-privileges`, mounted the AppImage tree read-only, and did not install or
invoke the networking plugin. The retained
[validation environment and commands](../validation/README.md) reproduce the
workflow without retaining the AppImage.

The target command was equivalent to:

```console
/app/AppRun --debug 5 \
  --export-3mf /output/roundtrip-1.3mf /input.3mf
```

Bambu Studio identified itself as `02.07.01.62`, parsed the three generated
entries, assembled four objects, logged every expected name including escaped
characters, Unicode, and the two duplicate display names, and successfully
exported a native project. The saved project SHA-256 was
`e79f1ba7dabd60d909910b2fc098e2e80bc979d865bb11183ffb180bb33b7ef9`.

The saved `Metadata/model_settings.config` SHA-256 was
`b1e405aee977d3e2cc561a013ae69a3bedba8f96cbca1b1bccf9f476aad4df81`.
It contained four separate object records and preserved all expected names,
including both records with duplicate display text. A second offline invocation
loaded that target-produced project through `IMPORT_STAGE_FINISH`, parsed its
model settings, and reassembled all four objects, demonstrating reload. A
further second-save attempt did not produce an artifact and is not claimed as a
success; the successful sequence claimed here is generated project load, target
save, and target-produced project reload through importer completion.

Neither the AppImage nor target-generated archives are committed or
redistributed. The source-authored fixture recipe, commands, identities, hashes,
and results above are sufficient to reproduce the compatibility observation.

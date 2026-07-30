# Slicer Project Generators

This repository is the isolated home for slicer project generator packages and
their target-derived provenance.

**Current status:** The repository contains three neutral command stubs and no
implemented capabilities, slicer schemas, formats, fixtures, target-derived
logic, released generator packages, or approved service integrations.

Each command reports that it has no implemented capabilities and exits with a
nonzero status. See [Architecture](docs/src/project/architecture.md),
[Licensing](docs/src/project/licensing.md), and the normative
[Provenance Policy](docs/src/project/slicer-project-generator-provenance.md)
before proposing implementation work.

The related [`onshape-export`](https://github.com/altendky/onshape-export)
project owns the source-neutral protocol, runtime isolation, independent output
validation, and generated-artifact publication. Generator release here and
service approval there are separate reviews. No service integration commit is
pinned yet.

## Development

The workspace requires Rust 1.94.
The scaffold check requires Bash and `jq`; Ubuntu GitHub-hosted runners provide
`jq`, and CI verifies it explicitly.

```console
cargo metadata --locked --no-deps --format-version 1
cargo build --workspace --locked
scripts/check-scaffold.sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo deny check
reuse lint
actionlint
shellcheck scripts/check-scaffold.sh
```

Each workspace crate declares its license in its `Cargo.toml` manifest.
Repository-authored support material is licensed under AGPL-3.0-only; the full
text is in [LICENSE](LICENSE).

# Slicer Project Generators

This repository is the home for derivative slicer project generator packages
and their target-derived provenance. Each workspace crate corresponds to its
named upstream AGPL reference project and records the exact upstream reference
used by implemented capabilities.

**Current status:** `crates/bambu-studio` implements the protocol-independent
`bambu-studio.named-objects` and `bambu-studio.support-blocking-volumes`
capabilities at revision 1 as a Rust library, validated against Bambu Studio
2.7.1.62. The repository has no released generator package or approved service
integration.

All three commands remain protocol stubs that report no command-accessible
capabilities and exit with a nonzero status. See
[Architecture](docs/src/project/architecture.md),
[Licensing](docs/src/project/licensing.md), and the normative
[Provenance Policy](docs/src/project/slicer-project-generator-provenance.md)
when contributing implementation work.

The stable counterpart repository is
[`onshape-export`](https://github.com/altendky/onshape-export). Its pinned
[normative integration policy][service-integration-policy] and
[proposed generator contract][service-generator-contract] own the
source-neutral protocol, generator approval, runtime sandbox, independent
validation and hashing, cache, publication, and revocation. This repository's
local [Provenance Policy](docs/src/project/slicer-project-generator-provenance.md)
remains canonical for target source access, provenance, implementation, builds,
and generator releases. Generator release here and service approval there are
separate reviews; the later service documents do not revise the generator
policy.

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
actionlint
shellcheck scripts/check-scaffold.sh
while IFS= read -r -d '' path; do
    case "${path,,}" in
        *.md|*.markdown|*.mdown|*.mdx|*.mkd|*.mdwn|*.mkdn|*.mkdown)
            printf '%s\0' "${path}"
            ;;
    esac
done < <(git ls-files -z) \
    | xargs -0 lychee --include-fragments --no-progress
```

Each workspace crate declares its license in its `Cargo.toml` manifest.
Repository-authored support material is licensed under AGPL-3.0-only; the full
text is in [LICENSE](LICENSE).

[service-integration-policy]: https://github.com/altendky/onshape-export/blob/6555aaf80b05e3bebdeb8b4a78dafcb8d84b3320/docs/src/project/slicer-project-generator-integration.md
[service-generator-contract]: https://github.com/altendky/onshape-export/blob/6555aaf80b05e3bebdeb8b4a78dafcb8d84b3320/docs/src/project/slicer-project-generators.md

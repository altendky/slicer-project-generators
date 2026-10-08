# Bambu Studio Generator 0.1.0 Reproduction

The implementation is exactly commit
`54a0fa1635f75039d6b858a95bbe803bb61103cf`. This package targets
`x86_64-unknown-linux-gnu`, built on Ubuntu 24.04 with Rust 1.94.1.
The complete corresponding-source artifact contains this repository snapshot,
unchanged checksum-verified Cargo dependencies, exact Rust library sources,
notices, and the build/package recipes. The generator is AGPL-3.0-only;
dependency license texts and copyrights accompany both distributions.

Obtain the official Rust 1.94.1 toolchain. Its exact installed file hashes are
in `build-inputs.json`; the pinned channel manifest identifies the official
distribution archives. Extract the corresponding-source artifact into
`BBL_SOURCE`, create a fresh `BBL_BUILD` directory, and set `BBL_RUST` to the
toolchain installation. Build the pinned compiler environment:

```console
docker build --platform linux/amd64 -t bambu-generator-build:0.1.0 \
  -f "$BBL_SOURCE/release/Dockerfile" "$BBL_SOURCE/release"
docker run --rm --platform linux/amd64 --network none --cap-drop ALL \
  --security-opt no-new-privileges --user "$(id -u):$(id -g)" \
  --env RUSTFLAGS='--remap-path-prefix=/src=/source --remap-path-prefix=/build=/build' \
  --mount type=bind,src="$BBL_RUST",dst=/rust,readonly \
  --mount type=bind,src="$BBL_SOURCE",dst=/src,readonly \
  --mount type=bind,src="$BBL_BUILD",dst=/build \
  bambu-generator-build:0.1.0 cargo build --release --locked --offline \
  --bin slicer-project-generator-bambu-studio
uv run --python 3.14.8 "$BBL_SOURCE/release/build.py" "$BBL_SOURCE" \
  "$BBL_BUILD/target/release/slicer-project-generator-bambu-studio" "$BBL_PACKAGE"
```

`BBL_PACKAGE` must not exist. Python 3.14.8 with zlib 1.3.2 tar/gzip packaging
fixes ordering, ownership, permissions, mtimes and compression headers. Repeat
with the exact interpreter build recorded in `build-inputs.json` (including its
compiler/build string and executable SHA-256); the recipe rejects a differing
`sys.version` or zlib runtime. The reviewed interpreter was uv-managed CPython
`3.14.8 (main, Oct 1 2026, 21:01:09) [Clang 22.1.3 ]`, with the precise spacing
preserved in the JSON record. Source executable flags are normalized to 0755
and other source files to 0644 and recorded in `release/source-modes.json`.
Repeat
in separate extracted source/build/package directories, with no shared target
or Cargo caches, and compare the executable and archive SHA-256 values.
`files.sha256.json` inventories every package file except itself. Package
identity is the protocol's JCS hash of its descriptor, distinct from the
distribution archive hash. The immutable aggregate provenance identity is
`BBL-MVP-PROVENANCE-SET-v1`.

Run all source tests in the same offline container using
`cargo test --workspace --release --locked --offline`. The source includes the
five synthetic fixture recipe and independent protocol/target validators under
`provenance/bambu-studio/validation/protocol-v1`. Release validation must use the
shipped descriptor and executable, as recorded in the release evidence. Generate
fresh fixtures with the source's `protocol_validation_fixture` example, then
run `uv run --python 3.14.8 release/validate.py "$BBL_PACKAGE/bambu-studio-v0.1.0-x86_64-unknown-linux-gnu" "$BBL_FIXTURES" "$BBL_REPORT"`.
This replaces only release identity bindings, runs five successful and eleven
failed invocations, and compares successful candidates to the reviewed hashes.
It does not manufacture a CLI self-validation failure: direct corruption tests
exercise that validator in the release-profile suite.

The archive contains `slicer-project-generator-bambu-studio` and its adjacent
`package.json`. Invoke it from a private immutable-input invocation directory:
`slicer-project-generator-bambu-studio --request request.json --result result.json`.
Exact input limits, supported core 3MF profile and initially unsupported cases
are documented in the source's `docs/src/project/bambu-protocol-v1.md`.
Compatibility is limited to the recorded Bambu Studio 2.7.1.62 synthetic cases.
Generator release does not approve service integration or production use.

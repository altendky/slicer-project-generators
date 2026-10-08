# Bambu 0.1.0 Release Verification

- Record ID: `BBL-MVP-RELEASE-VERIFICATION-v1`
- Author: Codex operating for Kyle Altendorf; date: 2026-10-08
- Exact implementation snapshot: `54a0fa1635f75039d6b858a95bbe803bb61103cf`
- Reproduction: [README](README.md), [preparation](prepare.py),
  [packaging](build.py), [CLI validation](validate.py), [Dockerfile](Dockerfile)

## Frozen Inputs And Reproduction

[build-inputs.json](build-inputs.json) fixes Rust 1.94.1 (commit
`e408947bfd200af42db322daf0fadfe7e26d3bd1`, LLVM 21.1.8), installed toolchain
file hashes, target `x86_64-unknown-linux-gnu`, complete snapshot-pinned Ubuntu
system resolution, GCC 13.3.0, binutils 2.42, glibc 2.39, release-profile
command, disabled incremental compilation, path-remapping flags, locked Cargo
graph, recipe hashes, notice hashes and exact packaging interpreter/zlib.
The compiler image ID is
`sha256:60baf59a2edb0ca6c5d2d3a0d74bd487f840abaa626e3f8696d6fc575f10145f`.
Its base digest and apt snapshot `20260731T230600Z` are in the Dockerfile.
APT repository signature checking remains enabled; the inherited snapshot
bootstrap command disables TLS peer verification as explicitly shown there.
Compiler runs are offline, drop capabilities, use no-new-privileges, run as
UID/GID 1000, and mount source/toolchain read-only.

Two independently prepared full source trees vendor the entire locked graph
offline. Every vendor file checksum matches Cargo's registry resolution.
Complete [dependency resolution](dependencies.json),
[source hashes](source-files.json) and [source executable modes](source-modes.json)
are retained. Normalized source executable flags preserve the scaffold command.
Two separate clean target/Cargo directories compile the implementation with
the same reviewed inputs; no target cache is shared. Their executables match
byte-for-byte with SHA-256
`734aa2340c954b7d06f1cea0d6c307b33279ffdd74b10cfe1639cdefc7e5e44c`.

Final source preparation and packaging each run independently twice. Binary,
descriptor and tar.gz archives match byte-for-byte. The source archive has
2,186 inventoried files plus its own hash inventory. Packaging fixes file order,
UID/GID, names, modes, source commit epoch `1791483663`, tar format and gzip
headers. The final validation recipe correction fixes negative-fixture ZIP
timestamps; compilation inputs outside `release/` were independently compared
and remain byte-identical to the two clean builds. Package construction verifies
the full source file set, every listed digest and executable mode.

[identities.json](identities.json) distinguishes binary SHA-256, build descriptor
identity, package JCS descriptor identity and archive hash. The
[artifact record](artifacts.json) fixes byte lengths/hashes for package and
complete corresponding source; [package-files.json](package-files.json) binds
all package files. No archive identity is substituted for the protocol package
identity and no self-referential hash is used.

The executable dynamically requires libc, libgcc_s and the system loader;
the largest referenced glibc symbol version is GLIBC_2.34. Runtime compatibility
was exercised on Ubuntu 24.04 with glibc 2.39 and the current test host with
glibc 2.43; no broader OS/ABI support is inferred.

## Tests And Release CLI Results

`cargo test --workspace --release --locked --offline` passes all 47 tests:
20 unit, 6 named-object, 7 blocker and 14 protocol/runtime tests. They use the
frozen implementation and locked dependencies in the pinned build environment.
Representative direct final-validator failures reject changed vertices, placement,
build membership, names, blocker roles/associations, content types, unexpected
XML structure, foreign namespaces and nested parts. These are corruption tests
of the actual release implementation; no successful CLI run is falsely described
as producing a self-validation failure.

The exact distribution archive was extracted and its adjacent descriptor and
executable used for five successful and eleven failed CLI invocations, twice
in separate fresh invocation trees. The two complete JSON result reports match
byte-for-byte; [cli-results.json](cli-results.json) records actual release
identities, invocation/settings identities, candidate hashes and bounded errors.
Successful results independently match the request's invocation, complete
identity bindings and declared output identity/role/path/media type/length/hash.
No temporary output remains. Ten writable-result failures are bounded below
the protocol's 256 KiB result limit and have one structured error and no output.
The result-temp conflict cannot commit a result, exits 1, removes its new
candidate, and preserves the existing marker; this is explicitly distinct from
a successfully committed structured failure.

| Trial | Actual error category/code |
| --- | --- |
| Malformed and oversized request | malformedRequest / malformed_request |
| Wrong package identity | unsupportedRequest / unsupported_generator_identity |
| Unsupported materials profile | unsupportedRequest / unsupported_geometry_profile |
| Retained hash/length mismatch, missing file, symlink | invalidInput / raw_input_file_invalid |
| One-byte output limit | generationFailed / output_byte_limit |
| Existing candidate temporary file | generationFailed / candidate_write_failed; marker preserved |
| Existing result temporary file | No result commit; new candidate removed; marker preserved |
| Wrong installation sidecar | internalError / package_identity_invalid |

The actual installed package also completes a fresh configured invocation in
the Ubuntu 24.04 container; result and candidate bytes match the host trial.
An initial smoke attempt reused an already completed invocation and correctly
returned `output_path_invalid`; that attempt is not counted as a success.
The independent jsonschema 4.25.1 Draft 2020-12 validator accepts all five
release requests, manifests, settings and results using the unchanged official
schemas. Failure outcomes are also checked by the CLI recipe; inherited
protocol tests verify structured failure schema constraints.

## Exact-Target Compatibility

The five successful release candidates match every SHA-256 in the preserved
[protocol verification](../../verification/protocol-v1.md). That record fixes
the official Bambu Studio 2.7.1.62 AppImage, upstream revision, isolated
validation image, bundled compatibility-only presets, complete native
save/reload exit-0 results, input/native hashes and diagnosed initial no-preset
reload crash. Existing native archives are reused only after exact candidate
byte equality; they are not claimed as fresh release-time saves.

The independent geometry verifier was rerun against the release candidates and
the same hash-bound native archives. [target-results.json](target-results.json)
records all five passing comparisons. Raw realization and neutral placement
match exactly; the largest native round-trip delta is
`1.4210854715202004e-14` mm, below the fixture tolerance `1e-9` mm.
Names, order, every model/blocker role and association survive native save and
reload. Native target archives contain nondeterministic metadata; their bytes
are not claimed reproducible and are not distributed.

Coverage includes ordinary configured Part Studio and rigid Assembly synthetic
inputs, multiple printables/blockers, duplicate names, equal bytes at distinct
paths with independent object identities, ordering, roles, associations and
placements, centimeters/internal component and build transforms plus neutral
placement exactly once. Unit/reflection variants are covered by source tests.
Initially unsupported profiles fail with no candidate. Compatibility is limited
to the recorded Bambu Studio 2.7.1.62 cases, not arbitrary projects, all target
versions or actual proprietary Onshape fixtures.

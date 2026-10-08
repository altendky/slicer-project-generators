# Bambu 0.1.0 Distribution Review

- Record ID: `BBL-MVP-DISTRIBUTION-v1`
- Author: Codex operating for Kyle Altendorf; date: 2026-10-08
- Exact candidate identities: [identities](identities.json) and
  [artifact hashes](artifacts.json)
- Implementation: `54a0fa1635f75039d6b858a95bbe803bb61103cf`

This supplements the existing Bambu source/AppImage, named-object,
support-blocking and protocol terms reviews at that snapshot. The AGPL source
lineage remains Bambu Studio, PrusaSlicer and Slic3r, with the consulted
contributors preserved in `NOTICE` and relationship records. Source-derived
implementation remains AGPL-3.0-only; original contributor attribution and
dated modification notices are retained. Neutral code/schemas retain the
Onshape Export authors' complete MIT grant.

The binary distribution supplies full `LICENSE`, `NOTICE`,
`LICENSE-neutral-MIT` and `third-party-notices/`. Its
[file inventory](package-files.json) hashes every supplied file except the
inventory itself. The corresponding-source distribution supplies the exact
full repository snapshot, every locked dependency's unmodified complete source,
Cargo source replacement configuration, recipe inputs and scripts, notices,
source hash/mode inventories, and the official Rust 1.94.1 library-source
archive. Both artifacts are published together; no written source offer or
unavailable external application-source dependency substitutes for that source.
The scripts used to control compilation and packaging are included. Modified
copies remain subject to the applicable AGPL source/network-source obligations.

## Locked Dependencies And Notices

[dependencies.json](dependencies.json) records all 36 registry packages,
versions, registry checksums, declared licenses and exact retained notice hashes.
This graph includes test/build packages as well as linked packages. It matches
the reviewed Cargo.lock SHA-256
`4fd2be4c07684a3cfce5693067e06c8c6cdcba99887d591e41203a7abcb278f4`.
Every file listed in each vendor checksum manifest was verified; source is not
altered. The prepared source retains original Cargo metadata and source notices.

The MIT choices in the protocol dependency review remain selected. Exceptions
are ryu-js 0.2.2 (BSL-1.0), unicode-ident 1.0.26 (MIT **and** Unicode-3.0), and
zlib-rs 0.6.8 (Zlib). Complete original license/copyright/permission/disclaimer
files are supplied in `third-party-notices/<crate>-<version>/`, including
nested source-package notice files. These selections permit this linked AGPL
distribution with those notices preserved; no grant is silently relicensed.

## Linked Standard Library And Startup Material

The exact Rust toolchain bin/lib file hashes and version are recorded in
[build-inputs.json](build-inputs.json). Its standard library generally offers
MIT or Apache-2.0; MIT is selected where offered. The complete official
`COPYRIGHT-library.html`, toolchain `COPYRIGHT.html`, and license directory are
retained in `third-party-notices/rust-1.94.1/`. All notice files from the exact
Rust library-source archive are also retained separately under
`third-party-notices/rust-library-source/`, including compiler-builtins and
libm `LICENSE.txt`.

Rust `compiler_builtins` 0.1.160 declares
`MIT AND Apache-2.0 WITH LLVM-exception AND (MIT OR Apache-2.0)`; select MIT
for the final alternative while retaining mandatory MIT, Apache and LLVM
exception texts. The libm notice preserves Jorge Aparicio, musl, Sun and other
source-specific contributions. Unicode-3.0 notices for core tables are retained.
The recorded Fuchsia/SGX entries describe other platforms and are not a claim
that their unused components are linked into this Linux executable.

The official Rust source artifact is
`https://static.rust-lang.org/dist/2026-03-26/rust-src-1.94.1.tar.xz`, SHA-256
`cb3756156fe6d2d6cedad327c94ad3721b612c4cd20dfb226d7543250788b66c`.
Its hash was checked against the installed official channel manifest and by the
independent release reviewer. The source archive includes compiler-builtins;
the exact standard-library sources and original notices remain unmodified.

GCC 13.3.0/libgcc startup material is built in the pinned Ubuntu snapshot with
the GCC Runtime Library Exception where applicable. Eligible compiled
combinations may retain the independent program's AGPL license; this is not an
exception claimed for every GCC file. The exact Ubuntu GCC and glibc copyright
files and all referenced full common-license texts (including LGPL-2.1)
are retained under `third-party-notices/system-runtime/`; glibc startup
terms and permitted linking are preserved. The complete application source and
build recipes permit rebuilding/relinking. The dynamic system `libc`,
`libgcc_s` and loader are external runtime requirements, not redistributed
shared libraries. No compiler executable or Bambu AppImage is redistributed.

## Distribution Conclusion

The retained lineage, modification notices, permissive grants, specific runtime
exceptions, AGPL license and complete corresponding source support the planned
distribution. No unknown or adverse obligation was identified in the reviewed
affected material. This records an actual agent review of exact material, not
an invented human/legal attestation. Synthetic fixture recipes are
repository-authored; no proprietary Onshape geometry, image, target-produced
archive, networking plugin or bundled Bambu preset is distributed. Target
testing continues under the preserved exact-AppImage review.

The actual package release still requires the named review outcome; service
distribution/selection, deployment and generated-artifact publication remain
separate responsibilities.

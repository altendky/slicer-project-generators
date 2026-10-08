# Bambu Protocol v1 Terms And Dependency Review

- Record ID: `BBL-PROTOCOL-TERMS-v1`
- Author: Codex operating for Kyle Altendorf; date: 2026-10-08
- Destination: `crates/bambu-studio`, AGPL-3.0-only
- Lockfile SHA-256: `4fd2be4c07684a3cfce5693067e06c8c6cdcba99887d591e41203a7abcb278f4`

## Source And Inputs

The Bambu source reference remains tag `v02.07.01.62`, full commit
`42d319c6692fa8e64790fddf0cdaafd2a4254bcc`. The
[existing source/AppImage review](bambu-studio-v02.07.01.62.md) and
[blocker supplement](support-blocking-volumes-v1.md) govern the same consulted
3MF implementation and target schema relationships. They permit this derivative
AGPL-3.0-only source with retained lineage, contribution notices, modification
dates, complete corresponding source on distribution, and applicable network
source-offer obligations. No additional exception was found in the consulted
regions. No Bambu source text, optional networking plugin, real Onshape fixture,
or target-generated archive is redistributed.

Neutral code, schemas, and examples are from `altendky/onshape-export` commit
`793ac0d5ab8e1db1ff6183fc651a568e5bacfe34`, declared `MIT OR Apache-2.0`.
The MIT choice permits the recorded reuse/adaptation into the AGPL crate, with
Copyright (c) 2026 The Onshape Export Authors and the complete permission and
disclaimer retained in `crates/bambu-studio/neutral/LICENSE-MIT`. Modified files
identify their source and modification date. Neutral protocol ownership remains
with that project. Consulted neutral runtime/settings docs and code are covered
by the same exact-material license; no service/source resolver code is copied.

All newly authored geometry fixtures are synthetic tetrahedra and neutral
placement/mapping records. They are repository-authored AGPL material; no user
image, proprietary selection evidence, or acquired target fixture is retained.
The public 3MF Consortium core specification 1.4.0 was consulted as ancillary
context; no specification prose, schema, fixture, or code is incorporated.
Implemented schema/transform facts use the pinned Bambu source evidence above.

## Exact Dependency Resolution

The graph is crates.io-only, with registry archive checksums retained in
`Cargo.lock`. Runtime ZIP feature is `deflate-flate2-zlib-rs`; no optional ZIP
encryption, Zopfli compressor, or networking dependency is enabled.

MIT is selected for: anyhow 1.0.104, block-buffer 0.12.1, cfg-if 1.0.4,
const-oid 0.10.2, cpufeatures 0.3.1, crc32fast 1.5.0, crypto-common 0.2.2,
digest 0.11.3, equivalent 1.0.2, flate2 1.1.10, hashbrown 0.17.1,
hybrid-array 0.4.15, indexmap 2.14.0, itoa 1.0.18, libc 0.2.190,
memchr 2.8.3, proc-macro2 1.0.107, quick-xml 0.41.0, quote 1.0.47,
roxmltree 0.21.1, serde 1.0.229, serde_core 1.0.229, serde_derive 1.0.229,
serde_jcs 0.2.0, serde_json 1.0.151, sha2 0.11.0, syn 3.0.6,
thiserror 2.0.21, thiserror-impl 2.0.21, typed-path 0.12.3, typenum 1.20.1,
unicode-ident 1.0.26 (also Unicode-3.0), zip 7.2.0, and zmij 1.0.23.
Quick-xml is test-only; derive/proc-macro crates are build dependencies.

Three narrowly versioned audit exceptions reflect reviewed source-package
license texts, not waivers:

- ryu-js 0.2.2: select BSL-1.0 from `Apache-2.0 OR BSL-1.0`. Retain full
  `LICENSE-BOOST` and source copyright notices with source distributions;
  executable-only distributions have the license's stated notice exception.
- unicode-ident 1.0.26: select MIT and the mandatory Unicode-3.0 component.
  Retain `LICENSE-MIT` and `LICENSE-UNICODE`, including Unicode, Inc.
  copyright and disclaimer; do not use its name for promotion without permission.
- zlib-rs 0.6.8: Zlib, Copyright (C) 2024 Trifecta Tech Foundation. Preserve
  its `LICENSE`, identify its origin, and mark any altered source. Dependency
  source is unchanged.

These permissive grants permit the planned linked AGPL distribution with their
notices preserved. Every MIT grant's copyright/permission/disclaimer must be
supplied with the relevant copied software. Future package construction must
collect exact notices from every locked source package and provide complete
corresponding source. No custom license-file, unknown registry, Git dependency,
duplicate dependency version, or unreviewed license is present. `cargo deny
check` passes advisories, bans, licenses, and sources after these reviews.

This public-incorporation review does not authorize a generator package release.
Issue #9 must bind exact distribution bytes and corresponding source to this
graph, notices, reproducible build and compatibility evidence, and named review.

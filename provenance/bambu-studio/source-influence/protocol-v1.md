# Bambu Protocol v1 Source Influence

- Record ID: `BBL-PROTOCOL-SOURCE-INFLUENCE-v1`
- Author and date: Codex operating for Kyle Altendorf, 2026-10-08
- Reviewers: Codex `protocol_research` and `geometry_research`, 2026-10-08
- Destination: `crates/bambu-studio`; source influence is recorded per relationship
- Target revision: Bambu Studio `42d319c6692fa8e64790fddf0cdaafd2a4254bcc`
- Neutral revision: Onshape Export `793ac0d5ab8e1db1ff6183fc651a568e5bacfe34`

Hashes below cover complete raw blobs, obtained from full local repositories.
No Bambu source expression or target fixture bytes are copied into the crate.
Neutral code and schemas are retained/adapted under MIT with their full notice.

| Source file | SHA-256 |
| --- | --- |
| Bambu `src/libslic3r/Format/bbs_3mf.cpp` | `e60656063798e92c1136e440024d2684f99b2be9e4b6f77a7de69a25c8a52877` |
| Onshape Export `src/generator_protocol.rs` | `d7df7dd4a227824e95dccb7c7bd1bae581cb58e6c672f78702517de5ec83bf95` |
| Onshape Export `src/cache_key.rs` | `26c586211939fb6c558517b6539f1ccf5183d1261df0e3ebd3e6412da0d5f95a` |
| Onshape Export `src/onshape_annotation.rs` | `d946c0e04efa90200276c6af35f1072683c302c6fbed5b2fa0f6d75292e0c57b` |
| Neutral protocol schema | `604676b76189003392332a94dd97a7dd7f0ef992321022639f18fff8c7ea403a` |
| Neutral settings-v2 schema | `f7ef791c8f0666a599157a34e3f4984f80026a3d7a121ebc04644281652ae06c` |

| Local relationship | Classification | Evidence kind | Evidence and influence |
| --- | --- | --- | --- |
| `generator_protocol.rs` types, validation, identity payloads and inherited tests | `direct_source_reuse` | `source_informed` | [Neutral module, lines 1–1440](https://github.com/altendky/onshape-export/blob/793ac0d5ab8e1db1ff6183fc651a568e5bacfe34/src/generator_protocol.rs#L1-L1440). Adapted visibility, include paths, non-null optional deserialization, and tests; retained runtime algorithms. |
| `cache_key.rs` canonicalization and hashing | `direct_source_reuse` | `source_informed` | [Neutral helper, lines 1–79](https://github.com/altendky/onshape-export/blob/793ac0d5ab8e1db1ff6183fc651a568e5bacfe34/src/cache_key.rs#L1-L79). Retained with MIT attribution. |
| `neutral/` schemas/examples | `direct_source_reuse` | `source_informed` | [Protocol schema](https://github.com/altendky/onshape-export/blob/793ac0d5ab8e1db1ff6183fc651a568e5bacfe34/protocol/generator/v1/generator-protocol.schema.json) and [settings schema](https://github.com/altendky/onshape-export/blob/793ac0d5ab8e1db1ff6183fc651a568e5bacfe34/protocol/generator-settings/v2/generator-settings.schema.json); unchanged source-neutral data and golden examples. |
| `settings.rs` types, normalization, contextual role/order validation and identity | `adapted_algorithm` | `source_informed` | [Neutral settings types, 148–194](https://github.com/altendky/onshape-export/blob/793ac0d5ab8e1db1ff6183fc651a568e5bacfe34/src/onshape_annotation.rs#L148-L194), [validation, 531–760](https://github.com/altendky/onshape-export/blob/793ac0d5ab8e1db1ff6183fc651a568e5bacfe34/src/onshape_annotation.rs#L531-L760), [identity, 782–828](https://github.com/altendky/onshape-export/blob/793ac0d5ab8e1db1ff6183fc651a568e5bacfe34/src/onshape_annotation.rs#L782-L828). Extracted the neutral subset; no Onshape authoring semantics. |
| `geometry.rs` `transform`, unit factors, mesh/component/build schema | `schema_fact` | `source_informed_schema_fact` | [Bambu transform and units, 566–636](https://github.com/bambulab/BambuStudio/blob/42d319c6692fa8e64790fddf0cdaafd2a4254bcc/src/libslic3r/Format/bbs_3mf.cpp#L566-L636), [vertices, 3854–3865](https://github.com/bambulab/BambuStudio/blob/42d319c6692fa8e64790fddf0cdaafd2a4254bcc/src/libslic3r/Format/bbs_3mf.cpp#L3854-L3865), [components/build, 3981–4037](https://github.com/bambulab/BambuStudio/blob/42d319c6692fa8e64790fddf0cdaafd2a4254bcc/src/libslic3r/Format/bbs_3mf.cpp#L3981-L4037). Learned serialization and frame facts; newly authored strict parser. |
| `geometry.rs` package namespaces, primary relationship and content types | `adapted_constant` | `source_informed` | Existing [named-object source influence](named-objects-v1.md) supplies exact package constants/templates. |
| `geometry.rs` component traversal and matrix composition | `adapted_algorithm` | `source_informed` | [Bambu component composition, 4196–4291](https://github.com/bambulab/BambuStudio/blob/42d319c6692fa8e64790fddf0cdaafd2a4254bcc/src/libslic3r/Format/bbs_3mf.cpp#L4196-L4291). Adapted traversal/frame relationships; explicit failure replaces permissive fallback. |
| `runtime.rs` target archive/readback schema, roles/names/component associations | `schema_fact` | `source_informed_schema_fact` | Reuses [named-object](named-objects-v1.md) and [blocker](support-blocking-volumes-v1.md) exact upstream evidence. Independently implemented validator compares realized expectations against parsed output. |
| `runtime.rs` dispatch and internal volume mappings | `adapted_algorithm` | `source_informed` | Reuses the reviewed local named-object/support-blocker generator APIs and their recorded target derivation. Multi-target replication and positional identities are newly authored. |
| `geometry.rs` archive/XML/resource bounds, cycle rejection, winding correction and placement pipeline | `independently_derived_behavior` | `independently_derived_behavior` | Repository-authored bounded profile and synthetic tests; elementary affine arithmetic, no copied validation algorithm. |
| `runtime.rs` package metadata binding, filesystem checks, hash/length checks, failure/atomic orchestration; `main.rs` CLI | `independently_derived_behavior` | `independently_derived_behavior` | Newly authored against the pinned neutral contract and runtime profile; separate agents reviewed failure paths. |
| `tests/common`, `tests/protocol_runtime.rs`, protocol fixture example | `independently_derived_behavior` | `independently_derived_behavior` | Locally authored tetrahedra, opaque identities, duplicate names/equal-byte paths, configured/rigid placement scenarios, and failure mutations. Structural assertions inherit the schema-fact rows above. No real source selectors or upstream fixtures. |

The existing Bambu/PrusaSlicer/Slic3r lineage and source notices continue to apply.
Exact terms and the neutral/dependency license selections are in the
[protocol review](../reviews/protocol-v1.md). This record is completed by the
immutable implementation and verification references in the protocol provenance
set; it makes no package release or service approval claim.

# Bambu Support-Blocking Volumes Source Influence v1

## Identity

- Record ID: `BBL-SUPPORT-BLOCKING-VOLUMES-SOURCE-INFLUENCE-v1`
- Capability: `bambu-studio.support-blocking-volumes` revision `1`
- Destination: `crates/bambu-studio`
- Upstream: `bambulab/BambuStudio` tag `v02.07.01.62`, commit
  `42d319c6692fa8e64790fddf0cdaafd2a4254bcc`
- Implementation commits:
  `50825bfc5628cc588a4ae2803b8e9477d7548b77`,
  `de0952d7ffc0476185dc9c0cf9ef2e136591361e`, and
  `2113110348d6f45f495b52481e8a5d0b9139dd7d`
- Author: Kyle Altendorf <sda@fstab.net>
- Date: 2026-07-31
- Independent review: fresh-context OpenCode `general` review agents checked the
  implementation and evidence on 2026-07-31; findings were resolved before the
  evidence commit.

## Content Identity

Hashes are SHA-256 over the complete raw Git blob contents at the pinned
commit, obtained through the GitHub contents API with its raw media type:

| Upstream file | SHA-256 |
| --- | --- |
| `src/libslic3r/Format/bbs_3mf.cpp` | `e60656063798e92c1136e440024d2684f99b2be9e4b6f77a7de69a25c8a52877` |
| `src/libslic3r/Model.hpp` | `dd9f7857725fd28be7eb8e3772f92dfddd79c26b05b494c2d9f3e4e44074f4c7` |
| `src/libslic3r/Model.cpp` | `bcdfd5b6ddb006ae15abff9e788b50216e03d29963024dd5230d0f7bdc227b73` |

## Relationship Classifications

| Local path or symbol | Local relationship | Evidence | Classification | Evidence kind | Influence |
| --- | --- | --- | --- | --- | --- |
| `crates/bambu-studio/src/lib.rs`: `SUPPORT_BLOCKING_VOLUMES_CAPABILITY_ID`, `SUPPORT_BLOCKING_VOLUMES_CAPABILITY_REVISION` | Capability identity | [`BBL-SUPPORT-BLOCKING-VOLUMES-BASELINE-v1`](../capabilities/support-blocking-volumes-v1.md) | `independently_derived_behavior` | `independently_derived_behavior` | Repository-authored identity from the source-neutral baseline. |
| `crates/bambu-studio/src/lib.rs`: package path constants and `generate_support_blocking_volumes` | Shared package paths and templates | [`BBL-NAMED-OBJECTS-SOURCE-INFLUENCE-v1`](named-objects-v1.md) and [`bbs_3mf.cpp` 150-172](https://github.com/bambulab/BambuStudio/blob/42d319c6692fa8e64790fddf0cdaafd2a4254bcc/src/libslic3r/Format/bbs_3mf.cpp#L150-L172) | `adapted_constant` | `source_informed` | Reused the reviewed target paths and minimal package templates. |
| `crates/bambu-studio/src/lib.rs`: `MODEL_SETTINGS_PATH` | `Metadata/model_settings.config` path | [`bbs_3mf.cpp` 168-171](https://github.com/bambulab/BambuStudio/blob/42d319c6692fa8e64790fddf0cdaafd2a4254bcc/src/libslic3r/Format/bbs_3mf.cpp#L168-L171) | `adapted_constant` | `source_informed` | Reused the exact target archive path. |
| `crates/bambu-studio/src/lib.rs`: `write_mesh_object` | Leaf object and mesh layout | [`bbs_3mf.cpp` 7401-7490](https://github.com/bambulab/BambuStudio/blob/42d319c6692fa8e64790fddf0cdaafd2a4254bcc/src/libslic3r/Format/bbs_3mf.cpp#L7401-L7490) | `schema_fact` | `source_informed_schema_fact` | Learned the per-volume mesh representation; Rust rendering is newly authored. |
| `crates/bambu-studio/src/lib.rs`: `generate_support_blocking_volumes` object-type match | `model` and `other` values | [`bbs_3mf.cpp` 7401-7427](https://github.com/bambulab/BambuStudio/blob/42d319c6692fa8e64790fddf0cdaafd2a4254bcc/src/libslic3r/Format/bbs_3mf.cpp#L7401-L7427) | `adapted_constant` | `source_informed` | Reused exact target object-type values. |
| `crates/bambu-studio/src/lib.rs`: containing-object rendering | Component-to-leaf association | [`bbs_3mf.cpp` 7264-7306](https://github.com/bambulab/BambuStudio/blob/42d319c6692fa8e64790fddf0cdaafd2a4254bcc/src/libslic3r/Format/bbs_3mf.cpp#L7264-L7306) | `schema_fact` | `source_informed_schema_fact` | Learned the association; grouping by opaque target identity is newly authored. |
| `crates/bambu-studio/src/lib.rs`: component and build rendering | Twelve-number identity transforms | [`bbs_3mf.cpp` 7285-7301](https://github.com/bambulab/BambuStudio/blob/42d319c6692fa8e64790fddf0cdaafd2a4254bcc/src/libslic3r/Format/bbs_3mf.cpp#L7285-L7301), [`7573-7597`](https://github.com/bambulab/BambuStudio/blob/42d319c6692fa8e64790fddf0cdaafd2a4254bcc/src/libslic3r/Format/bbs_3mf.cpp#L7573-L7597) | `adapted_constant` | `source_informed` | Adapted the exact transform representation. |
| `crates/bambu-studio/src/lib.rs`: build item rendering | Build references containing models | [`bbs_3mf.cpp` 7573-7597](https://github.com/bambulab/BambuStudio/blob/42d319c6692fa8e64790fddf0cdaafd2a4254bcc/src/libslic3r/Format/bbs_3mf.cpp#L7573-L7597) | `schema_fact` | `source_informed_schema_fact` | Learned the build-to-containing-object relationship. |
| `crates/bambu-studio/src/lib.rs`: build item rendering | `printable="1"` | [`bbs_3mf.cpp` 7573-7597](https://github.com/bambulab/BambuStudio/blob/42d319c6692fa8e64790fddf0cdaafd2a4254bcc/src/libslic3r/Format/bbs_3mf.cpp#L7573-L7597) | `adapted_constant` | `source_informed` | Reused the exact target serialized value. |
| `crates/bambu-studio/src/lib.rs`: model-settings rendering | Settings object, part, subtype, and metadata layout | [`bbs_3mf.cpp` 4321-4369](https://github.com/bambulab/BambuStudio/blob/42d319c6692fa8e64790fddf0cdaafd2a4254bcc/src/libslic3r/Format/bbs_3mf.cpp#L4321-L4369), [`7951-7998`](https://github.com/bambulab/BambuStudio/blob/42d319c6692fa8e64790fddf0cdaafd2a4254bcc/src/libslic3r/Format/bbs_3mf.cpp#L7951-L7998) | `schema_fact` | `source_informed_schema_fact` | Learned containing-model and child-volume role associations. |
| `crates/bambu-studio/src/lib.rs`: model-settings subtype match | `normal_part` and `support_blocker` | [`Model.cpp` 3400-3426](https://github.com/bambulab/BambuStudio/blob/42d319c6692fa8e64790fddf0cdaafd2a4254bcc/src/libslic3r/Model.cpp#L3400-L3426), [`Model.hpp` 328-336](https://github.com/bambulab/BambuStudio/blob/42d319c6692fa8e64790fddf0cdaafd2a4254bcc/src/libslic3r/Model.hpp#L328-L336) | `adapted_constant` | `source_informed` | Reused exact role strings and learned their meanings. |
| `crates/bambu-studio/src/lib.rs`: settings name rendering and `escape_xml` | Optional names and escaping | [`bbs_3mf.cpp` 7961-7969](https://github.com/bambulab/BambuStudio/blob/42d319c6692fa8e64790fddf0cdaafd2a4254bcc/src/libslic3r/Format/bbs_3mf.cpp#L7961-L7969), [`7992-7998`](https://github.com/bambulab/BambuStudio/blob/42d319c6692fa8e64790fddf0cdaafd2a4254bcc/src/libslic3r/Format/bbs_3mf.cpp#L7992-L7998) | `schema_fact` | `source_informed_schema_fact` | Learned optional placement; reused the reviewed local escaping helper. |
| `crates/bambu-studio/src/lib.rs`: resource and mesh serialization | Signed ID/index and float storage | [`bbs_3mf.cpp` 4325-4368](https://github.com/bambulab/BambuStudio/blob/42d319c6692fa8e64790fddf0cdaafd2a4254bcc/src/libslic3r/Format/bbs_3mf.cpp#L4325-L4368), [`7401-7448`](https://github.com/bambulab/BambuStudio/blob/42d319c6692fa8e64790fddf0cdaafd2a4254bcc/src/libslic3r/Format/bbs_3mf.cpp#L7401-L7448), [`7480-7490`](https://github.com/bambulab/BambuStudio/blob/42d319c6692fa8e64790fddf0cdaafd2a4254bcc/src/libslic3r/Format/bbs_3mf.cpp#L7480-L7490) | `schema_fact` | `source_informed_schema_fact` | Learned target storage types. |
| `crates/bambu-studio/src/lib.rs`: `validate_support_blocking_volumes`, `is_degenerate_in_target` | Conservative range and post-conversion nondegeneracy rejection | [`BBL-MVP-INPUT-v1`](../requirements/mvp-input-v1.md) and local tests | `independently_derived_behavior` | `independently_derived_behavior` | Newly authored rejection behavior based on representability requirements; it does not require exact f64-to-f32 equality. |
| `crates/bambu-studio/src/lib.rs`: `validate_support_blocking_volumes` | Full-request validation and atomic failure | [`BBL-MVP-INPUT-v1`](../requirements/mvp-input-v1.md) and failure tests | `independently_derived_behavior` | `independently_derived_behavior` | Newly authored without adapting upstream validation flow. |
| `crates/bambu-studio/src/lib.rs`: `generate_support_blocking_volumes`; `crates/bambu-studio/tests/support_blocking_volumes.rs` | Determinism, mappings, archive ordering, and atomic output | Local implementation and tests | `independently_derived_behavior` | `independently_derived_behavior` | Newly authored behavior not copied from target control flow. |
| `crates/bambu-studio/examples/support_blocking_validation_fixture.rs` | Fixture identities, names, geometry, and association scenario | Fixture recipe and [`BBL-MVP-INPUT-v1`](../requirements/mvp-input-v1.md) | `independently_derived_behavior` | `independently_derived_behavior` | Locally authored inputs; no upstream fixture bytes were copied. |
| `crates/bambu-studio/tests/support_blocking_volumes.rs`: structural success assertions | Archive schema and role associations | Classified serializer relationships above | `schema_fact` | `source_informed_schema_fact` | Tests verify source-informed schema without copying upstream test code. |
| `crates/bambu-studio/tests/support_blocking_volumes.rs`: failure and determinism assertions | Failure, determinism, mapping, and atomicity behavior | Repository requirements and local implementation | `independently_derived_behavior` | `independently_derived_behavior` | Locally authored tests exercise repository-defined behavior. |
| `provenance/bambu-studio/verification/support-blocking-volumes-v1.md` | Exact-target load, association preservation, save, and reload | Source-informed schema references and exact-target observations | `schema_fact` | `source_informed_schema_fact` | Offline validation confirms compatibility with target 2.7.1.62. |

No upstream source text is retained verbatim. Git blame of the exact consulted
regions attributes the model-settings importer and role mappings primarily to
lane.wei, component serialization primarily to chunmao.guo, and later relevant
component lines to maosheng.wei and zhimin.zeng; zhou.xu authored a nearby
model-settings face-count addition that was reviewed but not incorporated. The
broader Bambu Studio, PrusaSlicer, and Slic3r lineage and these contributors are
preserved in the capability notice. The applicable source, AppImage, and
attribution review is
[`BBL-SUPPORT-BLOCKING-VOLUMES-TERMS-v1`](../reviews/support-blocking-volumes-v1.md).

# Bambu Named Objects Source Influence v1

## Identity

- Record ID: `BBL-NAMED-OBJECTS-SOURCE-INFLUENCE-v1`
- Capability: `bambu-studio.named-objects` revision `1`
- Destination: `crates/bambu-studio`
- Upstream: `bambulab/BambuStudio` tag `v02.07.01.62`, commit
  `42d319c6692fa8e64790fddf0cdaafd2a4254bcc`
- Implementation commit:
  `383e480673d73096942cd11d04ebe597f467a05b`
- Author: Kyle Altendorf <sda@fstab.net>
- Date: 2026-07-31

## Content Identity

Hashes are SHA-256 over the complete raw Git blob contents checked out at the
pinned commit:

| Upstream file | SHA-256 |
| --- | --- |
| `src/libslic3r/Format/bbs_3mf.cpp` | `e60656063798e92c1136e440024d2684f99b2be9e4b6f77a7de69a25c8a52877` |
| `src/libslic3r/Model.hpp` | `dd9f7857725fd28be7eb8e3772f92dfddd79c26b05b494c2d9f3e4e44074f4c7` |
| `README.md` | `fb80b4eecdd5778ecfed7aa2c4e37ac79bdf7523364bea57973500084d628456` |
| `LICENSE` | `57c8ff33c9c0cfc3ef00e650a1cc910d7ee479a8bc509f6c9209a7c2a11399d6` |

## Relationship Classifications

| Local relationship | Upstream evidence | Classification | Evidence kind | Influence |
| --- | --- | --- | --- | --- |
| Archive paths and three-part package constants in `src/lib.rs` | [`bbs_3mf.cpp` 150-172](https://github.com/bambulab/BambuStudio/blob/42d319c6692fa8e64790fddf0cdaafd2a4254bcc/src/libslic3r/Format/bbs_3mf.cpp#L150-L172) | `adapted_constant` | `source_informed` | Adapted the exact model, content-types, and relationships paths; omitted Bambu-only optional parts from generated input. |
| Core 3MF model/object/mesh/build layout in `generate_named_objects` | [`bbs_3mf.cpp` 6693-6712](https://github.com/bambulab/BambuStudio/blob/42d319c6692fa8e64790fddf0cdaafd2a4254bcc/src/libslic3r/Format/bbs_3mf.cpp#L6693-L6712), [`6941-7073`](https://github.com/bambulab/BambuStudio/blob/42d319c6692fa8e64790fddf0cdaafd2a4254bcc/src/libslic3r/Format/bbs_3mf.cpp#L6941-L7073), and [`7330-7445`](https://github.com/bambulab/BambuStudio/blob/42d319c6692fa8e64790fddf0cdaafd2a4254bcc/src/libslic3r/Format/bbs_3mf.cpp#L7330-L7445) | `schema_fact` | `source_informed_schema_fact` | Learned the target-accepted XML namespaces, numeric object IDs, vertices, triangle indices, and build references. The Rust control flow and output minimization are newly authored. |
| Object-name association and escaping | [`bbs_3mf.cpp` 2161-2196](https://github.com/bambulab/BambuStudio/blob/42d319c6692fa8e64790fddf0cdaafd2a4254bcc/src/libslic3r/Format/bbs_3mf.cpp#L2161-L2196), [`3673-3676`](https://github.com/bambulab/BambuStudio/blob/42d319c6692fa8e64790fddf0cdaafd2a4254bcc/src/libslic3r/Format/bbs_3mf.cpp#L3673-L3676), and [`7961-7969`](https://github.com/bambulab/BambuStudio/blob/42d319c6692fa8e64790fddf0cdaafd2a4254bcc/src/libslic3r/Format/bbs_3mf.cpp#L7961-L7969) | `schema_fact` | `source_informed_schema_fact` | Learned that the core object `name` is imported as fallback and that Bambu saves escaped names in model settings. |
| In-memory meaning of a named printable object | [`Model.hpp` 342-355](https://github.com/bambulab/BambuStudio/blob/42d319c6692fa8e64790fddf0cdaafd2a4254bcc/src/libslic3r/Model.hpp#L342-L355) | `schema_fact` | `source_informed_schema_fact` | Confirmed one logical object has a name and instances. |
| Duplicate target object-ID rejection claim | [`bbs_3mf.cpp` 4321-4337](https://github.com/bambulab/BambuStudio/blob/42d319c6692fa8e64790fddf0cdaafd2a4254bcc/src/libslic3r/Format/bbs_3mf.cpp#L4321-L4337) | `schema_fact` | `source_informed_schema_fact` | Confirmed duplicate model-settings IDs are rejected by the pinned importer. |
| Full-request validation, structured error taxonomy, opaque identity mapping, and duplicate-name allowance | [`BBL-MVP-INPUT-v1`](../requirements/mvp-input-v1.md) and local tests | `independently_derived_behavior` | `independently_derived_behavior` | Newly authored from repository requirements without adapting upstream validation control flow. |
| Source-authored tetrahedron fixtures and test assertions | Local requirements and generated observations | `independently_derived_behavior` | `independently_derived_behavior` | No upstream fixture bytes or test expression were copied. |

No upstream source text is retained verbatim. The package layout and target
schema facts are AGPL source-informed derivative material governed by
[`BBL-UPSTREAM-TERMS-v1`](../reviews/bambu-studio-v02.07.01.62.md). Line history
attributes the principal consulted regions to Bambu Studio contributors
including lane.wei, chunmao.guo, zhimin.zeng, and zhou.xu; the broader Bambu
Studio, PrusaSlicer, and Slic3r lineage is preserved in the notice record.

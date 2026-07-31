# Bambu Named Objects Implementation v1

## Identity

- Record ID: `BBL-NAMED-OBJECTS-IMPLEMENTATION-v1`
- Capability ID and revision: `bambu-studio.named-objects`, `1`
- Exact validated target: Bambu Studio `2.7.1.62`
- Implementation commit:
  `383e480673d73096942cd11d04ebe597f467a05b`
- Author: Kyle Altendorf <sda@fstab.net>
- Authored: 2026-07-31
- License: `AGPL-3.0-only`

## Contract And Local Material

`crates/bambu-studio/src/lib.rs` exposes the capability identity, target
version, optional input mapping fields, indexed triangle mesh, object mapping,
structured validation errors, and `generate_named_objects`. It validates every
object before creating a ZIP writer. Success returns deterministic project bytes
and opaque-identity-to-target-ID mappings; failure returns no candidate bytes.

Accepted revision-1 input is a nonempty finite collection of objects. Each has
a nonempty opaque UTF-8 identity, nonempty XML-1.0-compatible Unicode display
name, and nonempty indexed triangle mesh with finite coordinates, in-range
indices, and nondegenerate triangles. Duplicate display names are allowed for
distinct identities. Missing, duplicate, conflicting, or ambiguous identity and
name mappings and invalid geometry produce structured errors.

The output is an uncompressed deterministic ZIP with fixed 1980 timestamps and
exactly `[Content_Types].xml`, `_rels/.rels`, and `3D/3dmodel.model`. Input order
assigns one-based target object IDs. The binary in `src/main.rs` remains the
failing protocol stub pending issue #8.

| Local material | SHA-256 at implementation commit |
| --- | --- |
| `crates/bambu-studio/src/lib.rs` | `93fc691fac8b97afeec489c9e878bc3f2c1da2dafb792f72de9d8068aebf2c2c` |
| `crates/bambu-studio/examples/validation_fixture.rs` | `bd5880d63b192c6ec20b3deca57d00f8d9cd8ef776a618296892f5760744488c` |
| `crates/bambu-studio/tests/fixtures.rs` | `d0cf9d86341ee6954760b0032e987c3d816667972e42f111ec00596bf629fe9c` |
| `crates/bambu-studio/tests/named_objects.rs` | `f364104e49bc37b07a21aa7fcb07b03c8695d4c7d7894ef99c6e5453f933d2b8` |

Dependencies and source influence resolve through
[`BBL-NAMED-OBJECTS-DEPENDENCIES-v1`](../reviews/rust-dependencies-v1.md) and
[`BBL-NAMED-OBJECTS-SOURCE-INFLUENCE-v1`](../source-influence/named-objects-v1.md).
